from __future__ import annotations

"""Persistent request history with search and context helpers."""

from dataclasses import dataclass, asdict
from datetime import datetime
import json
from pathlib import Path
from typing import List, Optional


@dataclass
class HistoryEntry:
    """Single request record."""

    timestamp: datetime
    text: str
    rating: Optional[int] = None


class RequestHistory:
    """Store and retrieve user requests.

    Parameters
    ----------
    path:
        Optional path to the history file.  If the file exists it is loaded on
        start and every modification is immediately persisted.
    """

    def __init__(
        self,
        path: str | Path = "logs/request_history.json",
        load_existing: bool = True,
    ) -> None:
        self.path = Path(path)
        self._entries: List[HistoryEntry] = []
        if load_existing and self.path.exists():
            try:
                data = json.loads(self.path.read_text(encoding="utf-8"))
                for item in data:
                    ts = datetime.fromisoformat(item["timestamp"])
                    rating = item.get("rating")
                    self._entries.append(HistoryEntry(ts, item["text"], rating))
            except Exception:  # pragma: no cover - corrupted history
                self._entries = []

    # ------------------------------------------------------------------
    def add(self, text: str, rating: Optional[int] = None) -> None:
        """Add ``text`` (with optional ``rating``) to the history and persist it."""

        entry = HistoryEntry(datetime.now(), text, rating)
        self._entries.append(entry)
        self._save()

    def search(self, query: str) -> List[HistoryEntry]:
        """Return all entries containing ``query`` (case insensitive)."""

        q = query.lower()
        return [e for e in self._entries if q in e.text.lower()]

    def get_context(self, limit: int = 5) -> str:
        """Return the last ``limit`` requests joined by newlines."""

        return "\n".join(e.text for e in self._entries[-limit:])

    def stats(self) -> str:
        """Return basic statistics for stored ratings."""

        ratings = [e.rating for e in self._entries if e.rating is not None]
        if not ratings:
            return "Оценок пока нет"
        total = len(ratings)
        avg = sum(ratings) / total
        lines = [f"Всего оценок: {total}", f"Средняя оценка: {avg:.2f}"]
        return "\n".join(lines)

    # ------------------------------------------------------------------
    def _save(self) -> None:
        self.path.parent.mkdir(parents=True, exist_ok=True)
        data = [asdict(e) | {"timestamp": e.timestamp.isoformat()} for e in self._entries]
        self.path.write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8")

    # Convenience helpers ------------------------------------------------
    def __len__(self) -> int:  # pragma: no cover - trivial
        return len(self._entries)

    def __iter__(self):  # pragma: no cover - trivial
        return iter(self._entries)
