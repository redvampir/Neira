from __future__ import annotations

"""Simple search panel providing filtering and preview of results."""

from dataclasses import dataclass
from typing import Iterable, List, Set

from ..indexer import Result, SearchIndexer


@dataclass
class PanelResult:
    """Representation of an item returned by :class:`SearchPanel`."""

    mode: str
    path: str
    preview: str


class SearchPanel:
    """High level wrapper around :class:`SearchIndexer` with filters."""

    def __init__(self, indexer: SearchIndexer) -> None:
        self.indexer = indexer
        self.filters: Set[str] = set()

    # ------------------------------------------------------------------
    def set_filters(self, modes: Iterable[str]) -> None:
        """Restrict searches to the provided modes."""
        self.filters = set(modes)

    # ------------------------------------------------------------------
    def search(self, query: str, limit: int = 5) -> List[PanelResult]:
        """Return results matching ``query`` respecting current filters."""
        results: List[PanelResult] = []
        for res in self.indexer.search(query, modes=self.filters or None, limit=limit):
            results.append(PanelResult(res.mode, str(res.path), res.snippet))
        return results


__all__ = ["SearchPanel", "PanelResult"]
