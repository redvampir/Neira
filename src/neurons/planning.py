"""Planning neuron."""

from __future__ import annotations

from dataclasses import dataclass

from .base import Neuron


@dataclass
class PlanningNeuron(Neuron):
    """Neuron capable of simple planning tasks."""

    type: str = "planning"

    # ------------------------------------------------------------------
    def process(self, goal: str) -> str:
        """Return a placeholder plan for the given goal."""

        self.activate()
        return f"plan:{goal}"
