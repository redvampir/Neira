"""Retrieve context from local files and web search."""
from __future__ import annotations

from pathlib import Path
from typing import List

from .api_client import SearchAPIClient


class Retriever:
    """Search local data files and an external API for additional context."""

    def __init__(
        self,
        data_path: str | Path | None = None,
        api_client: SearchAPIClient | None = None,
    ) -> None:
        self.data_path = Path(data_path) if data_path else None
        self.api_client = api_client or SearchAPIClient()

    # ------------------------------------------------------------------
    def _search_files(self, query: str) -> List[str]:
        """Return matching snippets from local files."""
        if not self.data_path or not self.data_path.exists():
            return []
        matches: List[str] = []
        q = query.lower()
        for file in self.data_path.rglob("*"):
            if not file.is_file():
                continue
            try:
                content = file.read_text(encoding="utf-8")
            except Exception:
                continue
            if q in content.lower():
                matches.append(content)
        return matches

    # ------------------------------------------------------------------
    def _search_api(self, query: str, limit: int) -> List[str]:
        """Return snippets from the external search API."""
        try:
            results = self.api_client.search(query, limit)
        except Exception:
            return []
        snippets: List[str] = []
        for item in results:
            snippet = item.get("snippet") if isinstance(item, dict) else None
            if snippet:
                snippets.append(snippet)
        return snippets

    # ------------------------------------------------------------------
    def retrieve(self, query: str, limit: int = 5) -> List[str]:
        """Aggregate context from local files and the external API."""
        context: List[str] = []
        context.extend(self._search_files(query))
        context.extend(self._search_api(query, limit))
        return context


__all__ = ["Retriever"]
