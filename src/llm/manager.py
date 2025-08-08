from __future__ import annotations

"""Management utilities for multiple LLM backends.

This module provides :class:`LLMManager` which keeps track of registered
models, their characteristics and performs model selection and simple
ensemble aggregation.  The manager can also forward interaction results to
:class:`~src.learning.learning_system.LearningSystem` for adaptive feedback.
"""

from dataclasses import dataclass, field
import time
from typing import Any, Callable, Dict, Optional, Tuple

try:  # pragma: no cover - optional peft dependency
    from peft import PeftModel  # type: ignore
except ModuleNotFoundError:  # pragma: no cover - optional dependency
    PeftModel = None  # type: ignore

from .base_llm import BaseLLM
from src.learning.learning_system import LearningSystem
from .prompts import chat_prompt


@dataclass
class ModelSpec:
    """Container describing a registered model and its meta data."""

    llm: BaseLLM
    speed: float
    cost: float
    accuracy: float
    capabilities: set[str] = field(default_factory=set)
    prompt_adapter: Optional[Callable[[str], str]] = None

    def adapt_prompt(self, prompt: str) -> str:
        """Apply the optional ``prompt_adapter`` to ``prompt``."""

        if self.prompt_adapter is None:
            return prompt
        return self.prompt_adapter(prompt)


@dataclass
class Task:
    """Describe a generation task and user context."""

    prompt: str
    user_id: str | None = None
    request_type: str = "general"
    context: Dict[str, Any] | None = None
    lora_adapters: list[str] | None = None


class LLMManager:
    """Manage multiple language models and route requests accordingly."""

    def __init__(self, learning_system: Optional[LearningSystem] = None) -> None:
        self.models: Dict[str, ModelSpec] = {}
        self.learning_system = learning_system or LearningSystem()
        # name -> {calls, successes, total_time}
        self.model_metrics: Dict[str, Dict[str, float]] = {}

    # ------------------------------------------------------------------
    def register_model(
        self,
        name: str,
        llm: BaseLLM,
        *,
        speed: float,
        cost: float,
        accuracy: float,
        prompt_adapter: Optional[Callable[[str], str]] = None,
        capabilities: set[str] | None = None,
    ) -> None:
        """Register an ``llm`` with its performance characteristics."""

        self.models[name] = ModelSpec(
            llm=llm,
            speed=speed,
            cost=cost,
            accuracy=accuracy,
            capabilities=capabilities or set(),
            prompt_adapter=prompt_adapter,
        )

    # ------------------------------------------------------------------
    def _apply_lora(self, llm: BaseLLM, adapters: list[str] | None) -> None:
        """Attach ``adapters`` to ``llm`` using :mod:`peft` if available."""

        if not adapters or PeftModel is None or not hasattr(llm, "model"):
            return
        model = getattr(llm, "model", None)
        if model is None:
            return
        for path in adapters:
            model = PeftModel.from_pretrained(model, path)
        llm.model = model

    # ------------------------------------------------------------------
    def select_model(self, task: Task) -> Tuple[str, BaseLLM, str]:
        """Return the model best suited for ``task``.

        Selection is based on previously recorded metrics, the requested
        ``task.request_type`` and any user preferences contained in
        ``task.context``.
        """

        available = [
            (name, spec)
            for name, spec in self.models.items()
            if spec.llm.is_available()
        ]
        if not available:
            raise RuntimeError("No available models")

        if task.context and "preferred_model" in task.context:
            preferred = task.context["preferred_model"]
            for name, spec in available:
                if name == preferred:
                    adapted = spec.adapt_prompt(task.prompt)
                    return name, spec.llm, adapted

        if task.request_type == "code":
            code_models = [
                (name, spec)
                for name, spec in available
                if "code" in spec.capabilities
            ]
            if code_models:
                available = code_models

        def success_rate(model_name: str) -> float:
            metrics = self.model_metrics.get(model_name, {})
            calls = metrics.get("calls", 0)
            if not calls:
                return self.models[model_name].accuracy
            return metrics.get("successes", 0) / calls

        def avg_time(model_name: str) -> float:
            metrics = self.model_metrics.get(model_name, {})
            calls = metrics.get("calls", 0)
            if not calls:
                return 1.0 / self.models[model_name].speed
            return metrics.get("total_time", 0.0) / calls

        length = len(task.prompt)

        if task.request_type == "fast":
            name, spec = min(available, key=lambda item: avg_time(item[0]))
        elif task.request_type == "cheap":
            name, spec = min(available, key=lambda item: item[1].cost)
        else:
            if length > 100:
                name, spec = min(available, key=lambda item: avg_time(item[0]))
            else:
                name, spec = max(available, key=lambda item: success_rate(item[0]))

        adapted_prompt = spec.adapt_prompt(task.prompt)
        return name, spec.llm, adapted_prompt

    # ------------------------------------------------------------------
    def generate(
        self,
        task: Task,
        *,
        ensemble: bool = False,
        success: bool = True,
        **kwargs,
    ) -> str:
        """Generate a response for ``task`` using the selected model.

        When ``ensemble`` is ``True`` all available models are invoked and the
        result from the most accurate one is returned.
        """

        task.prompt = chat_prompt(
            task.prompt, user_id=task.user_id, style_memory=self.learning_system.style_memory
        )

        if ensemble:
            outputs: Dict[str, str] = {}
            for name, spec in self.models.items():
                if not spec.llm.is_available():
                    continue
                self._apply_lora(spec.llm, task.lora_adapters)
                adapted = spec.adapt_prompt(task.prompt)
                start = time.perf_counter()
                outputs[name] = spec.llm.generate(adapted, **kwargs)
                duration = time.perf_counter() - start
                self._update_metrics(name, duration, success)
            if not outputs:
                raise RuntimeError("No available models")
            best_name = max(outputs, key=lambda n: self.models[n].accuracy)
            result = outputs[best_name]
            self._record_interaction(task.prompt, result, best_name)
            return result

        name, model, adapted = self.select_model(task)
        self._apply_lora(model, task.lora_adapters)
        start = time.perf_counter()
        result = model.generate(adapted, **kwargs)
        end = time.perf_counter()
        self._update_metrics(name, end - start, success)
        self._record_interaction(
            task.prompt, result, name, start_time=start, end_time=end
        )
        return result

    # ------------------------------------------------------------------
    def _record_interaction(
        self,
        prompt: str,
        response: str,
        model_name: str,
        *,
        start_time: float | None = None,
        end_time: float | None = None,
    ) -> None:
        """Forward evaluation information to the learning system."""

        rating = int(self.models[model_name].accuracy * 100)
        context: Dict[str, Any] = {"model": model_name}
        if start_time is not None and end_time is not None:
            context.update({"start_time": start_time, "end_time": end_time})
        self.learning_system.learn_from_interaction(
            prompt, response, rating, context=context
        )

    def _update_metrics(self, model_name: str, duration: float, success: bool) -> None:
        """Update usage metrics for ``model_name``."""

        metrics = self.model_metrics.setdefault(
            model_name, {"calls": 0, "successes": 0, "total_time": 0.0}
        )
        metrics["calls"] += 1
        metrics["total_time"] += duration
        if success:
            metrics["successes"] += 1


__all__ = ["ModelSpec", "LLMManager", "Task"]
