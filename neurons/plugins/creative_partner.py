"""Creative partner neuron plugin."""

from __future__ import annotations

from src.neurons import Neuron


class CreativePartnerNeuron(Neuron):
    """Neuron that decorates input text with a creative flair."""

    type = "creative_partner"

    def __init__(self, id: str) -> None:
        super().__init__(id=id, type=self.type)

    def process(self, text: str) -> str:
        """Return ``text`` with a creative prefix."""

        self.activate()
        return f"[Creative] {text}"
