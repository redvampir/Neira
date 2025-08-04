"""Storage for information about fictional worlds."""
from __future__ import annotations

import json
from pathlib import Path
from typing import Any, Dict


class WorldMemory:
    """Remember details about worlds and persist them to disk."""

    def __init__(self, storage_path: str | Path | None = None) -> None:
        self.storage_path = Path(storage_path or "data/world_memory.json")
        self._data: Dict[str, Dict[str, Any]] = {}
        self._load()

    def _load(self) -> None:
        if self.storage_path.exists():
            try:
                self._data = json.loads(self.storage_path.read_text(encoding="utf-8"))
            except Exception:
                self._data = {}

    def add(self, name: str, info: Dict[str, Any]) -> None:
        """Add or update information about a world."""
        self._data[name] = info

    def get(self, name: str | None = None) -> Any:
        """Retrieve information about a world or all worlds."""
        if name is None:
            return self._data
        return self._data.get(name)

    def save(self) -> None:
        """Persist current memory to the storage file."""
        self.storage_path.parent.mkdir(parents=True, exist_ok=True)
        self.storage_path.write_text(
            json.dumps(self._data, ensure_ascii=False, indent=2),
            encoding="utf-8",
        )


__all__ = ["WorldMemory"]
