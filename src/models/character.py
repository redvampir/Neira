from __future__ import annotations

from dataclasses import dataclass, field
from typing import List, Dict, Any


@dataclass
class Character:
    """Representation of a story character."""

    name: str
    personality_traits: List[str] = field(default_factory=list)
    emotional_moments: List[str] = field(default_factory=list)
    relationships: Dict[str, str] = field(default_factory=dict)
    growth_arc: List[str] = field(default_factory=list)
    first_mention: bool = False

    def to_dict(self) -> Dict[str, Any]:
        """Serialize the character to a JSON-serializable dict."""
        return {
            "name": self.name,
            "personality_traits": list(self.personality_traits),
            "emotional_moments": list(self.emotional_moments),
            "relationships": dict(self.relationships),
            "growth_arc": list(self.growth_arc),
            "first_mention": self.first_mention,
        }

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "Character":
        """Deserialize a :class:`Character` from a dictionary."""
        return cls(
            name=data.get("name", ""),
            personality_traits=list(data.get("personality_traits", [])),
            emotional_moments=list(data.get("emotional_moments", [])),
            relationships=dict(data.get("relationships", {})),
            growth_arc=list(data.get("growth_arc", [])),
            first_mention=bool(data.get("first_mention", False)),
        )


__all__ = ["Character"]
