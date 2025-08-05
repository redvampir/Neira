"""Master personality definitions."""

from __future__ import annotations

import json
from dataclasses import dataclass, field
from pathlib import Path
from typing import ClassVar, Dict, List

from src.core.ai_personality import AIPersonality

TRAITS_PATH = Path(__file__).with_name("default_traits.json")
DEFAULT_TRAITS: Dict[str, float] = json.loads(TRAITS_PATH.read_text())


@dataclass
class MasterPersonality(AIPersonality):
    """AI personality for game masters with narrative abilities."""

    traits: Dict[str, float] = field(default_factory=lambda: DEFAULT_TRAITS.copy())

    specializations: ClassVar[Dict[str, Dict[str, List[str]]]] = {
        "lorekeeper": {
            "knowledge_focus": ["history", "mythology"],
            "personality_traits": ["wise"],
            "decision_style": "narrative",
        },
        "tactician": {
            "knowledge_focus": ["strategy"],
            "personality_traits": ["calculating"],
            "decision_style": "analytical",
        },
    }

    def __init__(self, name: str, specialization: str = "lorekeeper") -> None:
        preset = self.specializations.get(specialization, {})
        super().__init__(
            name=name,
            role="master",
            knowledge_focus=preset.get("knowledge_focus", []),
            personality_traits=preset.get("personality_traits", []),
            current_character="Game Master",
            decision_style=preset.get("decision_style", "balanced"),
            communication_style=preset.get("communication_style", "narrative"),
        )
        self.traits = DEFAULT_TRAITS.copy()

    def describe_scene(self, elements: List[str]) -> str:
        """Return a narrative description of the given ``elements``."""
        return f"As the master, {self.name} sees " + ", ".join(elements) + "."
