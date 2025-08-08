from __future__ import annotations

"""Simple persistent catalog for storing reusable ideas or snippets.

The catalog acts as a key/value store backed by ``data/idea_catalog.json``.
Entries are stored as ``{"id": "text"}`` mappings and can be
created, retrieved, updated and deleted.  All operations modify the in-memory
representation; :meth:`save` persists the catalog to disk.
"""

from pathlib import Path
import json
from typing import Dict, Any


class IdeaCatalog:
    """Persistent key/value catalog for inspiration pieces."""

    def __init__(self, storage_path: str | Path | None = None) -> None:
        base = Path(__file__).resolve().parents[2]
        self.storage_path = Path(storage_path) if storage_path else base / "data" / "idea_catalog.json"
        self._data: Dict[str, Any] = {}
        self._load()

    # ------------------------------------------------------------------
    def _load(self) -> None:
        """Load catalog contents from :attr:`storage_path`."""
        if self.storage_path.exists():
            try:
                self._data = json.loads(self.storage_path.read_text(encoding="utf-8"))
            except Exception:
                self._data = {}

    # CRUD operations ---------------------------------------------------
    def add(self, key: str, value: Any) -> None:
        """Store ``value`` under ``key``."""
        self._data[key] = value

    def get(self, key: str | None = None) -> Any:
        """Retrieve an entry by ``key`` or return the whole catalog."""
        if key is None:
            return self._data
        return self._data.get(key)

    def update(self, key: str, value: Any) -> None:
        """Update ``key`` with ``value``.  Missing keys create new entries."""
        self._data[key] = value

    def delete(self, key: str) -> None:
        """Remove ``key`` from the catalog if present."""
        self._data.pop(key, None)

    # ------------------------------------------------------------------
    def save(self) -> None:
        """Persist current catalog state to disk."""
        self.storage_path.parent.mkdir(parents=True, exist_ok=True)
        self.storage_path.write_text(
            json.dumps(self._data, ensure_ascii=False, indent=2), encoding="utf-8"
        )


__all__ = ["IdeaCatalog"]
