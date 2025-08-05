"""Action neuron."""

from __future__ import annotations

from dataclasses import dataclass

from .base import Neuron


@dataclass
class ActionNeuron(Neuron):
    """Neuron capable of triggering actions."""

    type: str = "action"

    # ------------------------------------------------------------------
    def process(self, action: str) -> str:
        """Return a textual representation of an action being executed."""

        self.activate()
        return f"action:{action}"
