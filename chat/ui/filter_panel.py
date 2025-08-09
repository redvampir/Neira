from __future__ import annotations

"""Filtering helpers for notes based on tags and dates."""

from datetime import datetime
from typing import Iterable, List

from chat.memory_store import Note


class FilterPanel:
    """Simple filter controller for note collections."""

    def __init__(self) -> None:
        self.tags: List[str] | None = None
        self.start: datetime | None = None
        self.end: datetime | None = None

    def set_filters(
        self,
        tags: Iterable[str] | None = None,
        start: datetime | None = None,
        end: datetime | None = None,
    ) -> None:
        self.tags = list(tags) if tags else None
        self.start = start
        self.end = end

    def apply(self, notes: Iterable[Note]) -> List[Note]:
        """Return ``notes`` matching configured filters."""

        result: List[Note] = []
        for note in notes:
            if self.tags and not set(self.tags).issubset(note.tags):
                continue
            if self.start and note.created < self.start:
                continue
            if self.end and note.created > self.end:
                continue
            result.append(note)
        return result
