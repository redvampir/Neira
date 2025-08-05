"""Storage for writing style information and examples."""
from __future__ import annotations

import json
from pathlib import Path
from typing import Any, Dict, List


class StyleMemory:
    """Remember styles and their examples, persisted to disk."""

    def __init__(self, storage_path: str | Path | None = None) -> None:
        self.storage_path = Path(storage_path or "data/styles.json")
        self._data: Dict[str, Dict[str, Any]] = {}
        self._load()

    def _load(self) -> None:
        if self.storage_path.exists():
            try:
                self._data = json.loads(self.storage_path.read_text(encoding="utf-8"))
            except Exception:
                self._data = {}

    def add(
        self,
        style: str,
        example: str | None = None,
        description: str | None = None,
    ) -> None:
        """Add a style description or example."""
        entry = self._data.setdefault(style, {"description": "", "examples": []})
        if description:
            entry["description"] = description
        if example:
            entry["examples"].append(example)

    # ------------------------------------------------------------------
    def add_style_example(self, author: str, example: str) -> None:
        """Store a writing example linked to a particular author."""
        entry = self._data.setdefault(author, {"description": "", "examples": []})
        entry["examples"].append(example)

    def get(self, style: str | None = None) -> Dict[str, Any] | Dict[str, Dict[str, Any]]:
        """Retrieve stored style information."""
        if style is None:
            return self._data
        return self._data.get(style, {"description": "", "examples": []})

    def get_examples(self, style: str | None = None) -> List[str]:
        """Return a list of style examples."""
        if style:
            return list(self._data.get(style, {}).get("examples", []))
        examples: List[str] = []
        for info in self._data.values():
            examples.extend(info.get("examples", []))
        return examples

    def save(self) -> None:
        """Persist memory to disk."""
        self.storage_path.parent.mkdir(parents=True, exist_ok=True)
        self.storage_path.write_text(
            json.dumps(self._data, ensure_ascii=False, indent=2),
            encoding="utf-8",
        )


__all__ = ["StyleMemory"]
