from __future__ import annotations

"""Simple file system indexer used for local search across modes.

The implementation is intentionally lightweight.  It builds an in-memory
index for a set of directories and provides a ``search`` method returning
snippets containing the query.  The index can be refreshed to pick up file
system changes.
"""

from dataclasses import dataclass
from pathlib import Path
from typing import Dict, Iterable, List, Optional, Set


@dataclass
class Result:
    """Search result returned by :class:`SearchIndexer`.

    Attributes
    ----------
    mode:
        Name of the mode (``book``, ``code`` …) the result belongs to.
    path:
        Path to the matched file.
    snippet:
        Extract from the file around the match, suitable for previews.
    """

    mode: str
    path: Path
    snippet: str


class SearchIndexer:
    """Index text files for multiple application modes.

    Parameters
    ----------
    paths:
        Optional mapping from mode name to directory path.  When ``None`` a
        default set of directories within the repository is used.
    """

    def __init__(self, paths: Optional[Dict[str, Path]] = None) -> None:
        self.paths: Dict[str, Path] = paths or self._default_paths()
        self.index: Dict[str, Dict[Path, str]] = {}
        self.mtimes: Dict[Path, float] = {}
        self.index_all()

    # ------------------------------------------------------------------
    def _default_paths(self) -> Dict[str, Path]:
        root = Path(__file__).resolve().parents[2]
        return {
            "book": root / "data" / "books",
            "code": root / "code_editor",
            "chat": root / "chat",
            "resources": root / "modes" / "resource_manager",
        }

    # ------------------------------------------------------------------
    def index_all(self) -> None:
        """(Re)build the index for all known modes."""
        for mode, path in self.paths.items():
            self._index_mode(mode, path)

    # ------------------------------------------------------------------
    def _index_mode(self, mode: str, path: Path) -> None:
        if not path.exists():
            return
        store = self.index.setdefault(mode, {})
        for file in path.rglob("*"):
            if not file.is_file():
                continue
            try:
                text = file.read_text(encoding="utf-8")
            except Exception:
                continue
            store[file] = text
            self.mtimes[file] = file.stat().st_mtime
        # Remove entries for files that no longer exist
        for f in list(store.keys()):
            if not f.exists():
                store.pop(f, None)
                self.mtimes.pop(f, None)

    # ------------------------------------------------------------------
    def update(self) -> None:
        """Update the index to reflect file system changes.

        New and modified files are re-read.  Removed files are dropped from the
        index.  This method is inexpensive and can be called frequently.
        """

        for mode, path in self.paths.items():
            store = self.index.setdefault(mode, {})
            seen: Set[Path] = set()
            if path.exists():
                for file in path.rglob("*"):
                    if not file.is_file():
                        continue
                    try:
                        text = file.read_text(encoding="utf-8")
                    except Exception:
                        continue
                    mtime = file.stat().st_mtime
                    if (
                        file not in self.mtimes
                        or self.mtimes[file] != mtime
                        or store.get(file) != text
                    ):
                        store[file] = text
                        self.mtimes[file] = mtime
                    seen.add(file)
            # Drop files that vanished
            for f in list(store.keys()):
                if f not in seen:
                    store.pop(f, None)
                    self.mtimes.pop(f, None)

    # ------------------------------------------------------------------
    def search(
        self,
        query: str,
        modes: Optional[Iterable[str]] = None,
        limit: int = 5,
    ) -> List[Result]:
        """Search the index and return matching snippets.

        Parameters
        ----------
        query:
            Text to search for.
        modes:
            Optional iterable of mode names to restrict the search to.
        limit:
            Maximum number of results to return.
        """

        q = query.lower()
        results: List[Result] = []
        modes_to_search = list(modes) if modes else list(self.index.keys())
        for mode in modes_to_search:
            store = self.index.get(mode, {})
            for path, text in store.items():
                idx = text.lower().find(q)
                if idx == -1:
                    continue
                snippet = self._make_snippet(text, idx, len(q))
                results.append(Result(mode, path, snippet))
                if len(results) >= limit:
                    return results
        return results

    # ------------------------------------------------------------------
    @staticmethod
    def _make_snippet(text: str, idx: int, qlen: int, window: int = 40) -> str:
        start = max(0, idx - window // 2)
        end = min(len(text), idx + qlen + window // 2)
        return text[start:end].replace("\n", " ")


__all__ = ["SearchIndexer", "Result"]
