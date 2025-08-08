"""Manage information sources with reliability ranking."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Dict, List

from .source_tracker import SourceTracker

# Base reliability weights for different source types
SOURCE_RELIABILITY = {
    "low": 0.3,
    "medium": 0.6,
    "high": 1.0,
}


def calculate_source_limit(level: str, base_limit: int = 5) -> int:
    """Return the allowed number of sources for a given reliability level."""
    weight = SOURCE_RELIABILITY.get(level, SOURCE_RELIABILITY["low"])
    return max(1, int(base_limit * weight))


@dataclass
class ManagedSource:
    """Information about a registered source."""

    summary: str
    path: str
    reliability: float


class SourceManager:
    """Register sources while merging duplicates by reliability."""

    def __init__(self, tracker: SourceTracker | None = None) -> None:
        self.tracker = tracker or SourceTracker()
        self._sources: Dict[str, ManagedSource] = {}

    def register(self, summary: str, path: str, reliability: float) -> ManagedSource:
        """Register a source and return the stored entry.

        If the source already exists, the entry with the highest reliability is
        kept.
        """
        self.tracker.add(summary, path, reliability)
        existing = self._sources.get(path)
        new_entry = ManagedSource(summary=summary, path=path, reliability=reliability)
        if existing is None or reliability > existing.reliability:
            self._sources[path] = new_entry
            return new_entry
        return existing

    def get(self, path: str) -> ManagedSource | None:
        """Return a stored source by its path if present."""
        return self._sources.get(path)

    def all(self) -> List[ManagedSource]:
        """Return all registered sources sorted by reliability."""
        return sorted(self._sources.values(), key=lambda s: s.reliability, reverse=True)

    def limit_sources(self, query_context: Dict[str, Any]) -> List[ManagedSource]:
        """Limit stored sources according to ``query_context``.

        The ``query_context`` may define a ``reliability_level`` key indicating
        the desired trustworthiness (``"low"``, ``"medium"`` or ``"high"``).
        Based on that level, :func:`calculate_source_limit` determines how many
        sources should be retained. Only the most reliable sources are kept.
        """

        level = "medium"
        if isinstance(query_context, dict):
            level = query_context.get("reliability_level", level)
        else:  # pragma: no cover - fallback for attr-style contexts
            level = getattr(query_context, "reliability_level", level)

        limit = calculate_source_limit(level)
        sources = self.all()[:limit]
        self._sources = {s.path: s for s in sources}
        return sources


__all__ = [
    "SourceManager",
    "ManagedSource",
    "SOURCE_RELIABILITY",
    "calculate_source_limit",
]
