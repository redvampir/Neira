"""Character neuron plugin for generating responses in a specific voice.

This plugin demonstrates how to build a neuron that speaks as a
particular character.  Upon initialisation it retrieves character
information from :mod:`src.memory.character_memory` and uses that to
shape responses in :meth:`process`.
"""

from __future__ import annotations

from src.neurons import Neuron
from src.memory import CharacterMemory
from src.models import Character


class CharacterNeuron(Neuron):
    """Neuron that replies using a character's individual voice."""

    type = "character"

    def __init__(
        self,
        id: str,
        name: str,
        *,
        style: str | None = None,
        theme: str | None = None,
        memory: CharacterMemory | None = None,
    ) -> None:
        super().__init__(id=id, type=self.type)
        self.name = name
        self.memory = memory or CharacterMemory()
        char = self.memory.get(name)
        if char is None:
            char = Character(name=name)
            self.memory.add(char)
        self.character = char
        self.style = style or ", ".join(self.character.personality_traits)
        self.theme = theme or self.character.appearance

    # ------------------------------------------------------------------
    def process(self, text: str) -> str:
        """Return ``text`` phrased in the character's unique voice."""

        self.activate()
        parts = [self.character.name]
        if self.style:
            parts.append(self.style)
        if self.theme:
            parts.append(self.theme)
        prefix = " | ".join(parts)
        return f"{prefix}: {text}"
