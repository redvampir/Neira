from __future__ import annotations

"""Simple client for querying external search services and updating knowledge."""

from typing import Callable, Iterable, List, Dict
import requests
import re

from src.memory import MemoryIndex


class SearchAPIClient:
    """Interact with search APIs and maintain reliability metrics."""

    def __init__(
        self,
        memory: MemoryIndex | None = None,
        fetcher: Callable[[str, int], Iterable[Dict[str, str]]] | None = None,
    ) -> None:
        """
        Parameters
        ----------
        memory:
            Optional memory index used to store extracted facts and reliability
            information.
        fetcher:
            Callable performing the actual HTTP request. It receives the query and
            limit and must return an iterable of search result dictionaries. Each
            result dictionary should contain ``url`` and ``snippet`` keys. When not
            provided, a minimal DuckDuckGo JSON API is used.
        """
        self.memory = memory or MemoryIndex()
        self.fetcher = fetcher or self._duckduckgo_fetch
        self.session = requests.Session()

    # ------------------------------------------------------------------
    def _duckduckgo_fetch(self, query: str, limit: int) -> Iterable[Dict[str, str]]:
        """Fetch results from DuckDuckGo.

        This method is intentionally very small and only parses the parts used in
        unit tests. Network errors are swallowed and an empty list is returned.
        """
        try:
            resp = self.session.get(
                "https://duckduckgo.com/?q=" + query + "&format=json&no_redirect=1"
            )
            resp.raise_for_status()
            data = resp.json()
            results = []
            for item in data.get("RelatedTopics", [])[:limit]:
                if isinstance(item, dict) and item.get("Text") and item.get("FirstURL"):
                    results.append({"url": item["FirstURL"], "snippet": item["Text"]})
            return results
        except Exception:  # pragma: no cover - best effort network call
            return []

    # ------------------------------------------------------------------
    def search(self, query: str, limit: int = 5) -> List[Dict[str, str]]:
        """Return search results ranked by source reliability."""
        results = list(self.fetcher(query, limit))
        results.sort(
            key=lambda r: self.memory.source_reliability.get(r.get("url", ""), 0.5),
            reverse=True,
        )
        return results

    # ------------------------------------------------------------------
    def extract_facts(self, text: str) -> List[str]:
        """Naively extract facts by splitting text into sentences."""
        sentences = re.split(r"[\.!?]+", text)
        return [s.strip() for s in sentences if s.strip()]

    # ------------------------------------------------------------------
    def search_and_update(self, query: str, limit: int = 5) -> List[Dict[str, str]]:
        """Perform a search and update memory with extracted facts."""
        results = self.search(query, limit)
        for result in results:
            url = result.get("url", "")
            snippet = result.get("snippet", "")
            reliability = self.memory.source_reliability.get(url, 0.5)
            for fact in self.extract_facts(snippet):
                self.memory.set(fact, True, reliability=reliability)
            # ``MemoryIndex.update_reliability`` only updates existing keys, so we
            # modify the reliability mapping directly to ensure the source is
            # tracked even if it wasn't previously stored.
            self.memory.source_reliability[url] = min(1.0, reliability + 0.1)
        return results


__all__ = ["SearchAPIClient"]
