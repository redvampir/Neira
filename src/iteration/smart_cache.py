from __future__ import annotations

from collections import OrderedDict
from typing import Any, Iterable
import hashlib

from src.core.cache_manager import CacheManager


class SmartCache(CacheManager):
    """Cache with hot/warm/cold tiers built on top of :class:`CacheManager`.

    ``hot`` and ``warm`` tiers are in-memory :class:`OrderedDict` structures that
    implement a simple LRU replacement strategy.  ``cold`` storage uses the
    underlying :class:`CacheManager` persistence layer.

    Keys are derived from the hash of the ``query`` and optional ``tags`` which
    makes the cache context aware.
    """

    def __init__(
        self,
        cache_dir: str | None = ".cache",
        *,
        hot_limit: int = 32,
        warm_limit: int = 128,
    ) -> None:
        super().__init__(cache_dir)
        self.hot_limit = hot_limit
        self.warm_limit = warm_limit
        self.hot: OrderedDict[str, Any] = OrderedDict()
        self.warm: OrderedDict[str, Any] = OrderedDict()

    # ------------------------------------------------------------------
    # key helpers
    def _hash_key(self, query: str, tags: Iterable[str] | None) -> str:
        tags_part = "|".join(sorted(tags)) if tags else ""
        raw = f"{query}|{tags_part}".encode("utf-8")
        return hashlib.sha256(raw).hexdigest()

    # ------------------------------------------------------------------
    # tier maintenance
    def _trim_hot(self) -> None:
        while len(self.hot) > self.hot_limit:
            key, value = self.hot.popitem(last=False)
            self.warm[key] = value
            self._trim_warm()

    def _trim_warm(self) -> None:
        while len(self.warm) > self.warm_limit:
            # dropping from warm leaves it only in cold storage (disk)
            self.warm.popitem(last=False)

    def _promote_to_hot(self, key: str, value: Any) -> None:
        self.hot[key] = value
        self.hot.move_to_end(key)
        self._trim_hot()

    # ------------------------------------------------------------------
    # public API
    def set(self, query: str, value: Any, tags: Iterable[str] | None = None) -> None:
        key = self._hash_key(query, tags)
        super().set(key, {"value": value, "tags": list(tags) if tags else []})
        self._promote_to_hot(key, value)

    def get(self, query: str, tags: Iterable[str] | None = None) -> Any | None:
        key = self._hash_key(query, tags)
        if key in self.hot:
            self.hot.move_to_end(key)
            return self.hot[key]
        if key in self.warm:
            value = self.warm.pop(key)
            self._promote_to_hot(key, value)
            return value
        data = super().get(key)
        if data is None:
            return None
        if isinstance(data, dict) and "value" in data:
            value = data["value"]
        else:
            value = data
        self.warm[key] = value
        self._trim_warm()
        return value

    def invalidate(self, query: str | None = None, tags: Iterable[str] | None = None) -> None:
        if query is None:
            self.hot.clear()
            self.warm.clear()
            super().invalidate()
            return
        key = self._hash_key(query, tags)
        self.hot.pop(key, None)
        self.warm.pop(key, None)
        super().invalidate(key)
