from __future__ import annotations

"""Lazy loading utility for book chapters with cache management."""

from collections import OrderedDict
from pathlib import Path
from typing import Dict


class LazyMemoryLoader:
    """Load book chapter content on demand with basic caching.

    Parameters
    ----------
    book_dir:
        Directory containing chapter files. Each chapter should be a separate
        ``.txt`` file where the stem of the filename is treated as the chapter
        identifier.
    max_cache_size:
        Maximum number of chapters to keep in memory at once. When the limit is
        exceeded, the least recently used chapter is discarded.
    """

    def __init__(self, book_dir: str | Path, max_cache_size: int = 3) -> None:
        self.book_dir = Path(book_dir)
        self.max_cache_size = max_cache_size
        self._index: Dict[str, Path] = {}
        self._cache: "OrderedDict[str, str]" = OrderedDict()

    def load_book_index(self) -> Dict[str, Path]:
        """Build an index of available chapters without loading their content."""
        self._index = {
            file.stem: file for file in sorted(self.book_dir.glob("*.txt"))
        }
        return self._index

    def get_book_chapter(self, chapter: str) -> str:
        """Return the text of a chapter loading it from disk if necessary."""
        if not self._index:
            self.load_book_index()
        if chapter in self._cache:
            self._cache.move_to_end(chapter)
            return self._cache[chapter]
        if chapter not in self._index:
            raise KeyError(f"Unknown chapter: {chapter}")
        content = self._index[chapter].read_text(encoding="utf-8")
        self._cache[chapter] = content
        self._cache.move_to_end(chapter)
        if len(self._cache) > self.max_cache_size:
            self._cache.popitem(last=False)
        return content

    def clear_cache(self) -> None:
        """Remove all cached chapters to free memory."""
        self._cache.clear()


__all__ = ["LazyMemoryLoader"]
