"""Utility helpers for language parser implementations."""
from __future__ import annotations

from typing import Dict, Tuple


def lookup_localization(key: str, mapping: Dict[str, str]) -> str:
    """Return a localized string for ``key`` from ``mapping``.

    The function falls back to ``key`` if no corresponding entry is found.
    """

    return mapping.get(key, key)


def offset_to_position(text: str, offset: int) -> Tuple[int, int]:
    """Convert character ``offset`` into a 1-based ``(line, column)`` pair."""

    if offset < 0 or offset > len(text):  # pragma: no cover - defensive
        raise ValueError("offset out of range")
    line = text.count("\n", 0, offset) + 1
    col = offset - text.rfind("\n", 0, offset)
    return line, col


def position_to_offset(text: str, line: int, column: int) -> int:
    """Convert a ``(line, column)`` pair into a character offset."""

    if line < 1:  # pragma: no cover - defensive
        raise ValueError("line numbers start at 1")
    lines = text.splitlines(True)
    if line > len(lines):  # pragma: no cover - defensive
        raise ValueError("line out of range")
    return sum(len(lines[i]) for i in range(line - 1)) + (column - 1)
