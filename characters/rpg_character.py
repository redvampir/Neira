"""RPG character model with serialization helpers."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Dict, List


@dataclass
class RPGCharacter:
    """Representation of a role-playing game character."""

    name: str
    attributes: Dict[str, Any] = field(default_factory=dict)
    skills: List[str] = field(default_factory=list)
    equipment: List[str] = field(default_factory=list)
    status_effects: List[str] = field(default_factory=list)
    ai_personality_type: str = ""
    decision_patterns: List[str] = field(default_factory=list)
    roleplay_style: str = ""

    def to_dict(self) -> Dict[str, Any]:
        """Serialize the character to a JSON-serializable dict."""
        return {
            "name": self.name,
            "attributes": dict(self.attributes),
            "skills": list(self.skills),
            "equipment": list(self.equipment),
            "status_effects": list(self.status_effects),
            "ai_personality_type": self.ai_personality_type,
            "decision_patterns": list(self.decision_patterns),
            "roleplay_style": self.roleplay_style,
        }

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "RPGCharacter":
        """Deserialize an :class:`RPGCharacter` from a dictionary."""
        return cls(
            name=data.get("name", ""),
            attributes=dict(data.get("attributes", {})),
            skills=list(data.get("skills", [])),
            equipment=list(data.get("equipment", [])),
            status_effects=list(data.get("status_effects", [])),
            ai_personality_type=data.get("ai_personality_type", ""),
            decision_patterns=list(data.get("decision_patterns", [])),
            roleplay_style=data.get("roleplay_style", ""),
        )


__all__ = ["RPGCharacter"]
