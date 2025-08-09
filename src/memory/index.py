from __future__ import annotations

"""Minimal in-memory index used for tests.

The real project uses a Rust backed implementation, but for unit tests we only
need a very small subset of the functionality.  This lightweight replacement
stores values in dictionaries and tracks simple reliability scores.
"""

from typing import Dict, Any


class MemoryIndex:
    def __init__(self) -> None:
        self.cold_storage: Dict[str, Any] = {}
        self.source_reliability: Dict[str, float] = {}

    def set(self, key: str, value: Any, reliability: float = 0.5) -> None:
        self.cold_storage[key] = value
        self.source_reliability[key] = reliability


__all__ = ["MemoryIndex"]
