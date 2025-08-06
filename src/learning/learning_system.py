from __future__ import annotations

from dataclasses import dataclass, field
import json
from pathlib import Path
from typing import Any, Dict, List, Optional

from src.neurons import Neuron, NeuronFactory
from src.neurons.evolution import EvolutionConfig, evolve
from src.memory import StyleMemory
from src.learning.error_analysis import classify_error, recommend_action
from src.learning.feedback import FeedbackInterface


@dataclass
class LearningSystem:
    """Adaptive learning system for long-term interaction tracking.

    Attributes
    ----------
    experience_buffer:
        List of recorded interaction dictionaries.
    success_metrics:
        Mapping of metric name to value (e.g. positive/negative counts).
    failure_analysis:
        Logged failures with error description, context, model and response.
    adaptation_weights:
        Threshold values used to decide when to create new neuron types.
    """

    experience_buffer: List[Dict[str, Any]] = field(default_factory=list)
    success_metrics: Dict[str, int] = field(
        default_factory=lambda: {"positive": 0, "negative": 0}
    )
    failure_analysis: List[Dict[str, Any]] = field(default_factory=list)
    adaptation_weights: Dict[str, int] = field(default_factory=dict)
    style_memory: StyleMemory = field(default_factory=StyleMemory)
    response_cache: Dict[str, str] = field(default_factory=dict)

    # ------------------------------------------------------------------
    def learn_from_interaction(
        self,
        user_request: str,
        response: str,
        rating: int,
        context: Optional[Dict[str, Any]] = None,
    ) -> None:
        """Process interaction data and update internal state.

        Parameters
        ----------
        user_request:
            Original request from the user.
        response:
            System response produced.
        rating:
            User-provided rating where negative values indicate failure.
        context:
            Optional additional context for the interaction.
        """

        interaction = {
            "request": user_request,
            "response": response,
            "rating": rating,
            "context": context or {},
        }

        metrics = {
            "success": rating >= 0,
            "reaction_time": None,
            "error_type": None,
        }

        prev_failure = self.check_previous_failures(user_request)
        if prev_failure:
            interaction["context"]["warning"] = prev_failure.get("recommendation")
            print(f"Warning: previous failure detected. {prev_failure.get('recommendation')}")

        self.experience_buffer.append(interaction)

        if rating >= 0:
            self.success_metrics["positive"] += 1
            if context:
                user_id = context.get("user_id")
                tone = context.get("tone")
                examples = context.get("examples", [])
                if user_id and (tone or examples):
                    self.style_memory.save_preferences(user_id, tone, examples)
        else:
            self.success_metrics["negative"] += 1
            metrics["error_type"] = classify_error(interaction)
            self._analyze_failure(interaction, metrics["error_type"])

        # Capture reaction time if provided
        if context:
            start = context.get("start_time")
            end = context.get("end_time")
            if start is not None and end is not None:
                metrics["reaction_time"] = end - start

        interaction["metrics"] = metrics

        # Cache the response for future lookups
        self.response_cache[user_request] = response

        # Update adaptation weights based on success ratio
        total_pos = self.success_metrics["positive"]
        total_neg = self.success_metrics["negative"]
        total = total_pos + total_neg
        if total:
            ratio = total_pos / total
            self.adaptation_weights["success_rate"] = ratio
            for key in list(self.adaptation_weights.keys()):
                if key == "success_rate":
                    continue
                weight = self.adaptation_weights[key]
                self.adaptation_weights[key] = max(1, int(weight * ratio))

        if context and context.get("user_id"):
            FeedbackInterface.record(context["user_id"], interaction)

    # ------------------------------------------------------------------
    def _analyze_failure(self, interaction: Dict[str, Any], error_type: str) -> None:
        """Record failure details and recommended actions."""
        entry = {
            "request": interaction.get("request"),
            "response": interaction.get("response"),
            "context": interaction.get("context", {}),
            "model": interaction.get("context", {}).get("model"),
            "description": error_type,
            "recommendation": recommend_action(error_type),
            "error_type": error_type,
        }
        self.failure_analysis.append(entry)

    # ------------------------------------------------------------------
    def check_previous_failures(self, user_request: str) -> Optional[Dict[str, Any]]:
        """Return previous failure entry if the request was seen before."""

        for failure in self.failure_analysis:
            if failure.get("request") == user_request:
                return failure
        return None

    # ------------------------------------------------------------------
    def get_cached_response(self, user_request: str) -> Optional[str]:
        """Return cached response for ``user_request`` if available.

        The method also checks past failures and prints a warning when a
        matching failure is found.
        """

        prev_failure = self.check_previous_failures(user_request)
        if prev_failure:
            print(
                f"Warning: previous failure detected. {prev_failure.get('recommendation')}"
            )
        return self.response_cache.get(user_request)

    # ------------------------------------------------------------------
    def create_new_neuron_type(self) -> Optional[str]:
        """Create and register a new neuron type if needed.

        The decision is based on accumulated ``failure_analysis`` statistics. If
        a failure reason count exceeds its stored weight, a new neuron type is
        registered for that reason and its weight updated.
        """

        cfg = EvolutionConfig()

        error_counts: Dict[str, int] = {}
        for entry in self.failure_analysis:
            et = entry.get("error_type", "unknown")
            error_counts[et] = error_counts.get(et, 0) + 1

        for reason, count in error_counts.items():
            weight = self.adaptation_weights.get(reason, 0)
            source = Neuron(
                id=reason,
                type=reason,
                activation_count=count,
                strength=min(1.0, count / (weight + 1)),
            )
            result = evolve(source, cfg)
            if result:
                neuron_type, neuron_cls = result
                NeuronFactory.register(neuron_type, neuron_cls)
                self.adaptation_weights[reason] = count
                return neuron_type
        return None

    # ------------------------------------------------------------------
    def save_state(self, path: Path | str) -> None:
        """Serialize the learning state to ``path``."""

        data = {
            "experience_buffer": self.experience_buffer,
            "success_metrics": self.success_metrics,
            "failure_analysis": self.failure_analysis,
            "adaptation_weights": self.adaptation_weights,
        }
        Path(path).write_text(json.dumps(data))

    # ------------------------------------------------------------------
    @classmethod
    def load_state(cls, path: Path | str) -> "LearningSystem":
        """Load learning state from ``path``."""

        data = json.loads(Path(path).read_text())
        instance = cls()
        instance.experience_buffer = data.get("experience_buffer", [])
        instance.success_metrics = data.get("success_metrics", {})
        instance.failure_analysis = data.get("failure_analysis", [])
        instance.adaptation_weights = data.get("adaptation_weights", {})
        return instance


__all__ = ["LearningSystem"]

