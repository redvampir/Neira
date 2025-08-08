"""Logical partner neuron plugin."""

from __future__ import annotations

from src.neurons import Neuron


class LogicalPartnerNeuron(Neuron):
    """Neuron that applies logical reasoning to text."""

    type = "logical_partner"

    def __init__(self, id: str) -> None:
        super().__init__(id=id, type=self.type)

    def process(self, text: str) -> str:
        """Return ``text`` with a logical prefix."""

        self.activate()
        return f"[Logical] {text}"
