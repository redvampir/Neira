from __future__ import annotations

from dataclasses import dataclass, field
import json
from pathlib import Path
from typing import Any, Dict, List, Optional, Type

from src.neurons import Neuron, NeuronFactory
from src.memory import StyleMemory


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
        Counts of failure reasons gathered during analysis.
    adaptation_weights:
        Threshold values used to decide when to create new neuron types.
    """

    experience_buffer: List[Dict[str, Any]] = field(default_factory=list)
    success_metrics: Dict[str, int] = field(
        default_factory=lambda: {"positive": 0, "negative": 0}
    )
    failure_analysis: Dict[str, int] = field(default_factory=dict)
    adaptation_weights: Dict[str, int] = field(default_factory=dict)
    style_memory: StyleMemory = field(default_factory=StyleMemory)

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
        self.experience_buffer.append(interaction)

        if rating >= 0:
            self.success_metrics["positive"] += 1
            if context:
                user_id = context.get("user_id", "default")
                tone = context.get("tone")
                examples = context.get("examples", [])
                if tone or examples:
                    self.style_memory.add(user_id, "preferred", description=tone)
                    for ex in examples:
                        self.style_memory.add_style_example(user_id, "preferred", ex)
                    self.style_memory.save()
        else:
            self.success_metrics["negative"] += 1
            self._analyze_failure(interaction)

    # ------------------------------------------------------------------
    def _analyze_failure(self, interaction: Dict[str, Any]) -> None:
        """Record failure reason statistics."""

        reason = interaction["context"].get("topic", "unknown")
        self.failure_analysis[reason] = self.failure_analysis.get(reason, 0) + 1

    # ------------------------------------------------------------------
    def create_new_neuron_type(self) -> Optional[str]:
        """Create and register a new neuron type if needed.

        The decision is based on accumulated ``failure_analysis`` statistics. If
        a failure reason count exceeds its stored weight, a new neuron type is
        registered for that reason and its weight updated.
        """

        for reason, count in self.failure_analysis.items():
            weight = self.adaptation_weights.get(reason, 0)
            if count > weight:
                neuron_type = f"{reason}_neuron"

                def _process(self: Neuron, *args: Any, **kwargs: Any) -> Any:  # pragma: no cover - placeholder
                    return None

                new_cls: Type[Neuron] = type(
                    neuron_type,
                    (Neuron,),
                    {"process": _process},
                )
                NeuronFactory.register(neuron_type, new_cls)
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
        instance.failure_analysis = data.get("failure_analysis", {})
        instance.adaptation_weights = data.get("adaptation_weights", {})
        return instance


__all__ = ["LearningSystem"]

