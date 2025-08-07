from __future__ import annotations

from collections import OrderedDict
from typing import Any, Iterable
import hashlib
import time

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
        default_ttl: float | None = None,
    ) -> None:
        super().__init__(cache_dir)
        self.hot_limit = hot_limit
        self.warm_limit = warm_limit
        self.default_ttl = default_ttl
        self.hot: OrderedDict[str, Any] = OrderedDict()
        self.warm: OrderedDict[str, Any] = OrderedDict()
        self.expires_at: dict[str, float] = {}

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
    def set(
        self,
        query: str,
        value: Any,
        tags: Iterable[str] | None = None,
        ttl: float | None = None,
    ) -> None:
        key = self._hash_key(query, tags)
        ttl_value = ttl if ttl is not None else self.default_ttl
        if ttl_value is not None:
            exp = time.time() + ttl_value
            self.expires_at[key] = exp
        else:
            self.expires_at.pop(key, None)
            exp = None
        data = {"value": value, "tags": list(tags) if tags else []}
        if exp is not None:
            data["expires_at"] = exp
        super().set(key, data)
        self._promote_to_hot(key, value)

    def get(self, query: str, tags: Iterable[str] | None = None) -> Any | None:
        key = self._hash_key(query, tags)
        if self._is_expired(key):
            return None
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
        exp = data.get("expires_at") if isinstance(data, dict) else None
        if exp is not None:
            self.expires_at[key] = exp
            if exp < time.time():
                self.invalidate(query, tags)
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
            self.expires_at.clear()
            super().invalidate()
            return
        key = self._hash_key(query, tags)
        self.hot.pop(key, None)
        self.warm.pop(key, None)
        self.expires_at.pop(key, None)
        super().invalidate(key)

    # ------------------------------------------------------------------
    # expiration helpers
    def _is_expired(self, key: str) -> bool:
        exp = self.expires_at.get(key)
        if exp is not None and exp < time.time():
            self.hot.pop(key, None)
            self.warm.pop(key, None)
            self.expires_at.pop(key, None)
            super().invalidate(key)
            return True
        return False

    def cleanup(self) -> None:
        now = time.time()
        expired = [k for k, v in self.expires_at.items() if v < now]
        for key in expired:
            self.hot.pop(key, None)
            self.warm.pop(key, None)
            self.expires_at.pop(key, None)
            super().invalidate(key)
