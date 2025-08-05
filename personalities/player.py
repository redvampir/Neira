"""Player personality definitions."""

from __future__ import annotations

import json
from dataclasses import dataclass, field
from pathlib import Path
from typing import ClassVar, Dict, List

from src.core.ai_personality import AIPersonality

TRAITS_PATH = Path(__file__).with_name("default_traits.json")
DEFAULT_TRAITS: Dict[str, float] = json.loads(TRAITS_PATH.read_text())


@dataclass
class PlayerPersonality(AIPersonality):
    """AI personality for players with action choices."""

    traits: Dict[str, float] = field(default_factory=lambda: DEFAULT_TRAITS.copy())

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

    def __init__(self, name: str, archetype: str = "warrior") -> None:
        preset = self.archetypes.get(archetype, {})
        super().__init__(
            name=name,
            role="player",
            knowledge_focus=preset.get("knowledge_focus", []),
            personality_traits=preset.get("personality_traits", []),
            current_character=archetype.capitalize(),
            decision_style=preset.get("decision_style", "balanced"),
            communication_style=preset.get("communication_style", "conversational"),
        )
        self.traits = DEFAULT_TRAITS.copy()

    def choose_action(self, situation: str, options: List[str]) -> str:
        """Select an action from ``options`` given the ``situation``."""
        if not options:
            return "wait"
        return options[0]
