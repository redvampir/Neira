from __future__ import annotations

"""Management utilities for multiple LLM backends.

This module provides :class:`LLMManager` which keeps track of registered
models, their characteristics and performs model selection and simple
ensemble aggregation.  The manager can also forward interaction results to
:class:`~src.learning.learning_system.LearningSystem` for adaptive feedback.
"""

from dataclasses import dataclass
from typing import Callable, Dict, Optional, Tuple

from .base_llm import BaseLLM
from src.learning.learning_system import LearningSystem


@dataclass
class ModelSpec:
    """Container describing a registered model and its meta data."""

    llm: BaseLLM
    speed: float
    cost: float
    accuracy: float
    prompt_adapter: Optional[Callable[[str], str]] = None

    def adapt_prompt(self, prompt: str) -> str:
        """Apply the optional ``prompt_adapter`` to ``prompt``."""

        if self.prompt_adapter is None:
            return prompt
        return self.prompt_adapter(prompt)


class LLMManager:
    """Manage multiple language models and route requests accordingly."""

    def __init__(self, learning_system: Optional[LearningSystem] = None) -> None:
        self.models: Dict[str, ModelSpec] = {}
        self.learning_system = learning_system or LearningSystem()

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
    ) -> None:
        """Register an ``llm`` with its performance characteristics."""

        self.models[name] = ModelSpec(
            llm=llm,
            speed=speed,
            cost=cost,
            accuracy=accuracy,
            prompt_adapter=prompt_adapter,
        )

    # ------------------------------------------------------------------
    def select_model(
        self, prompt: str, *, request_type: str = "general"
    ) -> Tuple[str, BaseLLM, str]:
        """Return the model best suited for ``prompt``.

        The choice considers ``request_type`` (e.g. ``"fast"`` or ``"cheap"``),
        the length of ``prompt`` and the availability of registered models.
        The returned tuple contains the model name, the model instance and the
        adapted prompt for that model.
        """

        available = [
            (name, spec)
            for name, spec in self.models.items()
            if spec.llm.is_available()
        ]
        if not available:
            raise RuntimeError("No available models")

        length = len(prompt)

        if request_type == "fast":
            name, spec = max(available, key=lambda item: item[1].speed)
        elif request_type == "cheap":
            name, spec = min(available, key=lambda item: item[1].cost)
        else:
            if length > 100:
                name, spec = max(available, key=lambda item: item[1].speed)
            else:
                name, spec = max(available, key=lambda item: item[1].accuracy)

        adapted_prompt = spec.adapt_prompt(prompt)
        return name, spec.llm, adapted_prompt

    # ------------------------------------------------------------------
    def generate(
        self,
        prompt: str,
        *,
        request_type: str = "general",
        ensemble: bool = False,
        **kwargs,
    ) -> str:
        """Generate a response for ``prompt`` using the selected model.

        When ``ensemble`` is ``True`` all available models are invoked and the
        result from the most accurate one is returned.
        """

        if ensemble:
            outputs: Dict[str, str] = {}
            for name, spec in self.models.items():
                if not spec.llm.is_available():
                    continue
                adapted = spec.adapt_prompt(prompt)
                outputs[name] = spec.llm.generate(adapted, **kwargs)
            if not outputs:
                raise RuntimeError("No available models")
            best_name = max(outputs, key=lambda n: self.models[n].accuracy)
            result = outputs[best_name]
            self._record_interaction(prompt, result, best_name)
            return result

        name, model, adapted = self.select_model(prompt, request_type=request_type)
        result = model.generate(adapted, **kwargs)
        self._record_interaction(prompt, result, name)
        return result

    # ------------------------------------------------------------------
    def _record_interaction(self, prompt: str, response: str, model_name: str) -> None:
        """Forward evaluation information to the learning system."""

        rating = int(self.models[model_name].accuracy * 100)
        context = {"model": model_name}
        self.learning_system.learn_from_interaction(
            prompt, response, rating, context=context
        )


__all__ = ["ModelSpec", "LLMManager"]
