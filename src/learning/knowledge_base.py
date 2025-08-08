from __future__ import annotations

import json
from pathlib import Path
from typing import Any, Dict, List, Optional


class KnowledgeBase:
    """Simple JSON-backed storage for failure records."""

    def __init__(self, path: Path | str | None = None) -> None:
        self.path = Path(path) if path else None
        self._entries: List[Dict[str, Any]] = []
        if self.path and self.path.exists():
            try:
                self._entries = json.loads(
                    self.path.read_text(encoding="utf-8")
                )
            except json.JSONDecodeError:
                self._entries = []

    # ------------------------------------------------------------------
    def add(self, entry: Dict[str, Any]) -> None:
        """Add a new failure ``entry`` to the knowledge base."""
        self._entries.append(entry)

    # ------------------------------------------------------------------
    def query(self, request: str) -> Optional[Dict[str, Any]]:
        """Return the first entry matching ``request`` if present."""
        for item in self._entries:
            if item.get("request") == request:
                return item
        return None

    # ------------------------------------------------------------------
    def save(self) -> None:
        """Persist entries to disk when a path is configured."""
        if not self.path:
            return
        self.path.write_text(
            json.dumps(self._entries), encoding="utf-8"
        )


__all__ = ["KnowledgeBase"]
