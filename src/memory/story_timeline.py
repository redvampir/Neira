"""Timeline of events occurring in stories.

The timeline stores events keyed by an arbitrary string (for example a chapter
or scene name).  Each event can contain free‑form information.  Data is stored
as JSON in ``data/timeline.json`` and can be modified through :py:meth:`add`,
:py:meth:`get` and persisted with :py:meth:`save`.
"""

from __future__ import annotations

from pathlib import Path
import json
from typing import Any, Dict


class StoryTimeline:
    """Maintain a simple mapping of story events."""

    def __init__(self, storage_path: str | Path | None = None) -> None:
        self.storage_path = Path(storage_path or "data/timeline.json")
        self._data: Dict[str, Dict[str, Any]] = {}
        self._load()

    def _load(self) -> None:
        if self.storage_path.exists():
            try:
                self._data = json.loads(self.storage_path.read_text(encoding="utf-8"))
            except Exception:
                self._data = {}

    def add(self, key: str, event: Dict[str, Any]) -> None:
        self._data[key] = event

    def get(self, key: str | None = None) -> Any:
        if key is None:
            return self._data
        return self._data.get(key)

    def save(self) -> None:
        self.storage_path.parent.mkdir(parents=True, exist_ok=True)
        self.storage_path.write_text(
            json.dumps(self._data, ensure_ascii=False, indent=2),
            encoding="utf-8",
        )


__all__ = ["StoryTimeline"]

