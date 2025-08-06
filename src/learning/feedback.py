"""Interface for recording user feedback during learning."""
from __future__ import annotations

from pathlib import Path
import json
from typing import Any, Dict, List


class FeedbackInterface:
    """Simple persistence layer for interaction feedback."""

    storage_path = Path("data/feedback.json")
    _data: Dict[str, List[Dict[str, Any]]] = {}

    @classmethod
    def record(cls, user_id: str, interaction: Dict[str, Any]) -> None:
        """Record ``interaction`` feedback for ``user_id``."""
        cls._data.setdefault(user_id, []).append(interaction)
        cls.save()

    @classmethod
    def save(cls) -> None:
        """Persist feedback to disk."""
        cls.storage_path.parent.mkdir(parents=True, exist_ok=True)
        cls.storage_path.write_text(
            json.dumps(cls._data, ensure_ascii=False, indent=2),
            encoding="utf-8",
        )

    @classmethod
    def load(cls) -> None:
        """Load previously stored feedback if available."""
        if not cls.storage_path.exists():
            return
        try:
            raw = json.loads(cls.storage_path.read_text(encoding="utf-8"))
        except Exception:
            raw = {}
        cls._data = {k: list(v) for k, v in raw.items()}


__all__ = ["FeedbackInterface"]
