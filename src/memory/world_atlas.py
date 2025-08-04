"""Storage for imaginary locations and worlds.

The atlas keeps a mapping of world names to arbitrary descriptions.  Like the
other memory classes it supports :py:meth:`add`, :py:meth:`get` and
:py:meth:`save` and persists data in ``data/worlds.json``.
"""

from __future__ import annotations

from pathlib import Path
import json
from typing import Any, Dict


class WorldAtlas:
    """Remember worlds and their details."""

    def __init__(self, storage_path: str | Path | None = None) -> None:
        self.storage_path = Path(storage_path or "data/worlds.json")
        self._data: Dict[str, Dict[str, Any]] = {}
        self._load()

    def _load(self) -> None:
        if self.storage_path.exists():
            try:
                self._data = json.loads(self.storage_path.read_text(encoding="utf-8"))
            except Exception:
                self._data = {}

    def add(self, name: str, info: Dict[str, Any]) -> None:
        self._data[name] = info

    def get(self, name: str | None = None) -> Any:
        if name is None:
            return self._data
        return self._data.get(name)

    def save(self) -> None:
        self.storage_path.parent.mkdir(parents=True, exist_ok=True)
        self.storage_path.write_text(
            json.dumps(self._data, ensure_ascii=False, indent=2),
            encoding="utf-8",
        )


__all__ = ["WorldAtlas"]

