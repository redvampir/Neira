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
    """Remember user specific styles and their examples, persisted to disk."""

    def __init__(self, storage_path: str | Path | None = None) -> None:
        self.storage_path = Path(storage_path or "data/styles.json")
        # user_id -> {author -> StylePattern}
        self._data: Dict[str, Dict[str, StylePattern]] = {}
        self.load()

    # ------------------------------------------------------------------
    def add(
        self,
        user_id: str,
        author: str,
        example: str | None = None,
        description: str | None = None,
        characteristics: List[str] | None = None,
    ) -> StylePattern:
        """Add or update information about an author's style for ``user_id``."""
        user_styles = self._data.setdefault(user_id, {})
        pattern = user_styles.setdefault(author, StylePattern(author=author))
        if description:
            pattern.description = description
        if example:
            pattern.examples.append(example)
        if characteristics:
            pattern.characteristics.extend(characteristics)
        return pattern

    def add_style_example(self, user_id: str, author: str, example: str) -> None:
        """Store a writing example linked to a particular author for ``user_id``."""
        self.add(user_id, author, example=example)

    def get_style(
        self, user_id: str, author: str | None = None
    ) -> StylePattern | Dict[str, StylePattern] | None:
        """Retrieve stored style information."""
        user_styles = self._data.get(user_id, {})
        if author is None:
            return user_styles
        return user_styles.get(author)

    def get_examples(self, user_id: str, author: str | None = None) -> List[str]:
        """Return a list of style examples for ``user_id``."""
        user_styles = self._data.get(user_id, {})
        if author:
            pattern = user_styles.get(author)
            return list(pattern.examples) if pattern else []
        examples: List[str] = []
        for pattern in user_styles.values():
            examples.extend(pattern.examples)
        return examples

    # ------------------------------------------------------------------
    def save(self) -> None:
        """Persist memory to disk."""
        serialised = {
            user_id: {author: pattern.to_dict() for author, pattern in styles.items()}
            for user_id, styles in self._data.items()
        }
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
        self._data = {
            user_id: {
                author: StylePattern.from_dict(info)
                for author, info in styles.items()
            }
            for user_id, styles in raw.items()
        }


__all__ = ["StyleMemory", "StylePattern"]
