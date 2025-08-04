from __future__ import annotations

import hashlib
import json
from pathlib import Path
from typing import Any


class CacheManager:
    """Simple JSON file-based cache with optional in-memory layer.

    Entries are stored as JSON files in ``cache_dir`` and mirrored in memory
    for the lifetime of the process.  Call :py:meth:`invalidate` to remove
    specific entries or to clear the whole cache.
    """

    def __init__(self, cache_dir: str | Path = ".cache") -> None:
        self.cache_dir = Path(cache_dir)
        self.cache_dir.mkdir(exist_ok=True)
        self._memory: dict[str, Any] = {}

    def _path_for(self, key: str) -> Path:
        hashed = hashlib.sha256(key.encode("utf-8")).hexdigest()
        return self.cache_dir / f"{hashed}.json"

    def get(self, key: str) -> Any | None:
        if key in self._memory:
            return self._memory[key]
        path = self._path_for(key)
        if path.exists():
            try:
                data = json.loads(path.read_text(encoding="utf-8"))
            except Exception:
                return None
            self._memory[key] = data
            return data
        return None

    def set(self, key: str, value: Any) -> None:
        self._memory[key] = value
        path = self._path_for(key)
        path.write_text(json.dumps(value, ensure_ascii=False), encoding="utf-8")

    def invalidate(self, key: str | None = None) -> None:
        """Remove a single cached entry or clear the whole cache."""
        if key is None:
            self._memory.clear()
            for file in self.cache_dir.glob("*.json"):
                try:
                    file.unlink()
                except FileNotFoundError:
                    pass
        else:
            self._memory.pop(key, None)
            path = self._path_for(key)
            if path.exists():
                path.unlink()
