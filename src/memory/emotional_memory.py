"""Simple storage for emotional impressions.

The class keeps track of emotions experienced during interaction with
characters or events.  It exposes :py:meth:`add`, :py:meth:`get` and
:py:meth:`save` methods and persists its data in ``data/emotions.json``.
"""

from __future__ import annotations

from pathlib import Path
import json
from typing import Any, Dict, List

from src.core.config import get_logger

logger = get_logger(__name__)


class EmotionalMemory:
    """Remember emotional reactions and persist them on disk."""

    def __init__(self, storage_path: str | Path | None = None) -> None:
        self.storage_path = Path(storage_path or "data/emotions.json")
        self._data: Dict[str, List[str]] = {}
        self._load()

    def _load(self) -> None:
        if self.storage_path.exists():
            try:
                self._data = json.loads(self.storage_path.read_text(encoding="utf-8"))
            except json.JSONDecodeError as exc:
                logger.warning(
                    "Failed to decode emotions file %s: %s", self.storage_path, exc
                )
                self._data = {}

    def add(self, key: str, emotion: str) -> None:
        self._data.setdefault(key, []).append(emotion)

    def get(self, key: str | None = None) -> Any:
        if key is None:
            return self._data
        return self._data.get(key, [])

    def save(self) -> None:
        self.storage_path.parent.mkdir(parents=True, exist_ok=True)
        self.storage_path.write_text(
            json.dumps(self._data, ensure_ascii=False, indent=2),
            encoding="utf-8",
        )


__all__ = ["EmotionalMemory"]

