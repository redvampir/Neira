"""Player personality definitions."""

from __future__ import annotations

import json
from dataclasses import dataclass, field
from pathlib import Path
from typing import ClassVar, Dict, List

from src.core.ai_personality import AIPersonality

TRAITS_PATH = Path(__file__).with_name("default_traits.json")
DEFAULT_TRAITS: Dict[str, float] = json.loads(
    TRAITS_PATH.read_text(encoding="utf-8")
)


@dataclass
class PlayerPersonality(AIPersonality):
    """AI personality for players with action choices."""

    traits: Dict[str, float] = field(default_factory=lambda: DEFAULT_TRAITS.copy())
    lora_adapters: List[str] = field(default_factory=list)

    archetypes: ClassVar[Dict[str, Dict[str, List[str]]]] = {
        "warrior": {
            "knowledge_focus": ["combat"],
            "personality_traits": ["brave", "resilient"],
            "decision_style": "bold",
        },
        "scholar": {
            "knowledge_focus": ["arcana", "history"],
            "personality_traits": ["curious"],
            "decision_style": "thoughtful",
        },
    }

    def __init__(
        self,
        name: str,
        archetype: str = "warrior",
        lora_adapters: List[str] | None = None,
    ) -> None:
        """Initialize a player personality.

        Parameters
        ----------
        name:
            Display name of the personality.
        archetype:
            Optional archetype preset such as ``warrior``.
        lora_adapters:
            Identifiers for LoRA adapters associated with this personality.
        """

        preset = self.archetypes.get(archetype, {})
        super().__init__(
            name=name,
            role="player",
            knowledge_focus=preset.get("knowledge_focus", []),
            personality_traits=preset.get("personality_traits", []),
            current_character=archetype.capitalize(),
            decision_style=preset.get("decision_style", "balanced"),
            communication_style=preset.get(
                "communication_style", "conversational"
            ),
        )
        self.traits = DEFAULT_TRAITS.copy()
        self.lora_adapters = lora_adapters or []

    def choose_action(self, situation: str, options: List[str]) -> str:
        """Select an action from ``options`` given the ``situation``."""
        if not options:
            return "wait"
        return options[0]
