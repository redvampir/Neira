from __future__ import annotations

from dataclasses import dataclass
from typing import Dict, Any


@dataclass
class Scene:
    """Representation of a narrative scene."""

    description: str
    content: str
    emotion: str = ""
    style: str = ""

    def to_dict(self) -> Dict[str, Any]:
        return {
            "description": self.description,
            "content": self.content,
            "emotion": self.emotion,
            "style": self.style,
        }

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "Scene":
        return cls(
            description=data.get("description", ""),
            content=data.get("content", ""),
            emotion=data.get("emotion", ""),
            style=data.get("style", ""),
        )


__all__ = ["Scene"]
