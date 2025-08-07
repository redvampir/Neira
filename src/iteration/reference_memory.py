from __future__ import annotations

"""Utilities for storing reference links for internal and external sources."""

from dataclasses import dataclass
from typing import List, Dict, Any, TYPE_CHECKING

from src.utils.source_tracker import SourceTracker

if TYPE_CHECKING:  # pragma: no cover - only for type hints
    from .deep_searcher import DeepSearcher


@dataclass
class ReferenceEntry:
    """Information about a stored reference."""

    summary: str
    path: str
    confidence: float


class ReferenceMemory:
    """Store references and register them via :class:`SourceTracker`.

    The memory distinguishes between internal references (local file paths)
    and external references (web links). A :class:`DeepSearcher` instance is
    used to provide additional context for summaries and is invoked whenever a
    new reference is created.
    """

    def __init__(
        self,
        tracker: SourceTracker | None = None,
        searcher: "DeepSearcher" | None = None,
    ) -> None:
        self.tracker = tracker or SourceTracker()
        if searcher is None:
            from .deep_searcher import DeepSearcher

            searcher = DeepSearcher()
        self.searcher = searcher
        self.internal_sources: List[ReferenceEntry] = []
        self.external_sources: List[ReferenceEntry] = []

    # ------------------------------------------------------------------
    def _is_external(self, path: str) -> bool:
        return path.startswith("http://") or path.startswith("https://")

    # ------------------------------------------------------------------
    def create_reference_link(self, summary: str, full_path: str, confidence: float) -> str:
        """Create a markdown reference link and track the source.

        The link is registered with :class:`SourceTracker` and stored in either
        the internal or external list depending on its origin. The associated
        :class:`DeepSearcher` is queried with ``summary`` to integrate search
        capabilities.
        """

        # Register source
        self.tracker.add(summary, full_path, confidence)

        # Obtain additional context via deep searcher (ignore failures)
        try:
            self.searcher.search(summary)
        except Exception:
            pass

        entry = ReferenceEntry(summary=summary, path=full_path, confidence=confidence)
        if self._is_external(full_path):
            self.external_sources.append(entry)
        else:
            self.internal_sources.append(entry)

        return f"[{summary}]({full_path})"

    # ------------------------------------------------------------------
    def search(self, query: str, user_id: str | None = None, limit: int = 5) -> List[Dict[str, Any]]:
        """Proxy search requests to the underlying :class:`DeepSearcher`."""
        return self.searcher.search(query, user_id=user_id, limit=limit)

    # ------------------------------------------------------------------
    def report(self) -> str:
        """Return a report of all linked sources."""

        from .memory_inspector import MemoryInspector

        inspector = MemoryInspector(self)
        return inspector.generate_report()


__all__ = ["ReferenceMemory", "ReferenceEntry"]
