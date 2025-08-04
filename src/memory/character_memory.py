"""Character memory storage.

This module provides a tiny persistence layer for characters that Neyra
encounters.  The data is stored as JSON inside the ``data`` directory so that
it survives between sessions.  The class offers a minimal dictionary‑like
interface with :py:meth:`add`, :py:meth:`get` and :py:meth:`save` methods which
are used throughout the project.
"""

from __future__ import annotations

from pathlib import Path
import json
from typing import Any, Dict


class CharacterMemory:
    """Store information about characters and persist it on disk."""

    def __init__(self, storage_path: str | Path | None = None) -> None:
        self.storage_path = Path(storage_path or "data/characters.json")
        self._data: Dict[str, Dict[str, Any]] = {}
        self._load()

    # ------------------------------------------------------------------
    def _load(self) -> None:
        """Load previously saved memory from disk if available."""
        if self.storage_path.exists():
            try:
                self._data = json.loads(self.storage_path.read_text(encoding="utf-8"))
            except Exception:
                self._data = {}

    # ------------------------------------------------------------------
    def add(self, name: str, info: Dict[str, Any]) -> None:
        """Add or update information about a character."""
        self._data[name] = info

    def get(self, name: str | None = None) -> Dict[str, Any] | None:
        """Retrieve information about a character or all characters."""
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

    # Convenience methods to behave a bit like a dictionary -----------------
    def __contains__(self, name: str) -> bool:  # pragma: no cover - trivial
        return name in self._data

    def keys(self):  # pragma: no cover - simple delegation
        return self._data.keys()

    def __len__(self) -> int:  # pragma: no cover - simple delegation
        return len(self._data)


__all__ = ["CharacterMemory"]

