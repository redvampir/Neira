"""Storage for writing style information and examples."""
from __future__ import annotations

from dataclasses import asdict, dataclass, field
import json
from pathlib import Path
from typing import Dict, List, Any


@dataclass
class StylePattern:
    """Describes a writing style for a particular author."""

    author: str
    description: str = ""
    examples: List[str] = field(default_factory=list)
    characteristics: List[str] = field(default_factory=list)

    # ------------------------------------------------------------------
    def to_dict(self) -> Dict[str, Any]:
        """Return a serialisable representation of the style pattern."""
        return asdict(self)

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "StylePattern":
        """Create a :class:`StylePattern` from a serialised form."""
        return cls(**data)


class StyleMemory:
    """Remember styles and their examples, persisted to disk."""

    def __init__(self, storage_path: str | Path | None = None) -> None:
        self.storage_path = Path(storage_path or "data/styles.json")
        self._data: Dict[str, StylePattern] = {}
        self.load()

    # ------------------------------------------------------------------
    def add(
        self,
        author: str,
        example: str | None = None,
        description: str | None = None,
        characteristics: List[str] | None = None,
    ) -> None:
        """Add or update information about an author's style."""
        pattern = self._data.setdefault(author, StylePattern(author=author))
        if description:
            pattern.description = description
        if example:
            pattern.examples.append(example)
        if characteristics:
            pattern.characteristics.extend(characteristics)
        return pattern

    def add_style_example(self, author: str, example: str) -> None:
        """Store a writing example linked to a particular author."""
        self.add(author, example=example)

    def get_style(self, author: str | None = None) -> StylePattern | Dict[str, StylePattern] | None:
        """Retrieve stored style information."""
        if author is None:
            return self._data
        return self._data.get(author)

    def get_examples(self, author: str | None = None) -> List[str]:
        """Return a list of style examples."""
        if author:
            pattern = self._data.get(author)
            return list(pattern.examples) if pattern else []
        examples: List[str] = []
        for pattern in self._data.values():
            examples.extend(pattern.examples)
        return examples

    # ------------------------------------------------------------------
    def save(self) -> None:
        """Persist memory to disk."""
        serialised = {author: pattern.to_dict() for author, pattern in self._data.items()}
        self.storage_path.parent.mkdir(parents=True, exist_ok=True)
        self.storage_path.write_text(
            json.dumps(serialised, ensure_ascii=False, indent=2),
            encoding="utf-8",
        )

    def load(self) -> None:
        """Load style information from disk."""
        if not self.storage_path.exists():
            return
        try:
            raw: Dict[str, Any] = json.loads(self.storage_path.read_text(encoding="utf-8"))
        except Exception:
            raw = {}
        self._data = {author: StylePattern.from_dict(info) for author, info in raw.items()}


__all__ = ["StyleMemory", "StylePattern"]
