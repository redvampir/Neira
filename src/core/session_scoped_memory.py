from __future__ import annotations

"""Utilities for per-user memory isolation.

This module exposes :class:`SessionScopedMemory` which lazily creates
``CharacterMemory``, ``WorldMemory`` and ``StyleMemory`` instances for each
``user_id``.  The memory for a user is stored inside a dedicated directory under
``base_path / user_id`` ensuring complete isolation between users.
"""

from pathlib import Path
from typing import Dict, Tuple

from src.memory import CharacterMemory, WorldMemory, StyleMemory


MemoryTuple = Tuple[CharacterMemory, WorldMemory, StyleMemory]


class SessionScopedMemory:
    """Manage separate memory objects for each user."""

    def __init__(self, base_path: str | Path = "data") -> None:
        self.base_path = Path(base_path)
        self._cache: Dict[str, MemoryTuple] = {}

    # ------------------------------------------------------------------
    def get(self, user_id: str) -> MemoryTuple:
        """Return memory objects associated with ``user_id``.

        Memory instances are created on first access and cached for subsequent
        calls.  Each user's data is stored under ``base_path / user_id``.
        """

        if user_id not in self._cache:
            user_dir = self.base_path / user_id
            char_mem = CharacterMemory(user_dir / "characters.json")
            world_mem = WorldMemory(user_dir / "world.json")
            style_mem = StyleMemory(user_dir / "styles.json")
            self._cache[user_id] = (char_mem, world_mem, style_mem)
        return self._cache[user_id]


__all__ = ["SessionScopedMemory"]
