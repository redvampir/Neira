from __future__ import annotations

from dataclasses import dataclass, field
from typing import List, Dict, Any

from .scene import Scene


@dataclass
class Chapter:
    """Representation of a book chapter."""

    title: str
    summary: str = ""
    scenes: List[Scene] = field(default_factory=list)

    def to_dict(self) -> Dict[str, Any]:
        return {
            "title": self.title,
            "summary": self.summary,
            "scenes": [scene.to_dict() for scene in self.scenes],
        }

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "Chapter":
        scenes_data = data.get("scenes", [])
        scenes = [Scene.from_dict(s) for s in scenes_data]
        return cls(
            title=data.get("title", ""),
            summary=data.get("summary", ""),
            scenes=scenes,
        )


__all__ = ["Chapter"]
