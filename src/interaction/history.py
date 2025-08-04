from __future__ import annotations

"""Simple request history tracking with timestamped entries."""

from dataclasses import dataclass
from datetime import datetime
from typing import List


@dataclass
class HistoryEntry:
    """A single history record."""

    timestamp: datetime
    text: str


class RequestHistory:
    """Store user requests with timestamps and provide lookup utilities."""

    def __init__(self) -> None:
        self._entries: List[HistoryEntry] = []

    def add(self, text: str) -> None:
        """Add a new request to the history."""
        self._entries.append(HistoryEntry(datetime.now(), text))

    def search(self, query: str) -> str:
        """Search by index (1-based) or by time substring.

        Parameters
        ----------
        query:
            If digits, treated as 1-based index; otherwise matched against
            timestamp formatted as ``YYYY-mm-dd HH:MM:SS``.
        """

        query = query.strip()
        if not query:
            return "📝 История пуста или запрос не указан." if not self._entries else "🤔 Нужен запрос для поиска."

        if query.isdigit():
            idx = int(query) - 1
            if 0 <= idx < len(self._entries):
                entry = self._entries[idx]
                stamp = entry.timestamp.strftime("%Y-%m-%d %H:%M:%S")
                return f"[{idx + 1}] {stamp} — {entry.text}"
            return "🤔 Не удалось найти запись с таким номером."

        for i, entry in enumerate(self._entries, 1):
            stamp = entry.timestamp.strftime("%Y-%m-%d %H:%M:%S")
            if query in stamp:
                return f"[{i}] {stamp} — {entry.text}"
        return "🤔 Не нашла запись по указанному времени."
