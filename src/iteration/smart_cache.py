from __future__ import annotations

from collections import OrderedDict
from typing import Any, Iterable
import hashlib
import json
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
        hot_limit_mb: float = 32,
        warm_limit_mb: float = 128,
        cold_limit_mb: float = 1024,
        warm_threshold: int = 2,
        hot_threshold: int = 5,
        default_ttl: float | None = None,
    ) -> None:
        super().__init__(cache_dir)
        self.hot_limit = int(hot_limit_mb * 1024 * 1024)
        self.warm_limit = int(warm_limit_mb * 1024 * 1024)
        self.cold_limit = int(cold_limit_mb * 1024 * 1024)
        self.warm_threshold = warm_threshold
        self.hot_threshold = hot_threshold
        self.default_ttl = default_ttl
        self.hot: OrderedDict[str, Any] = OrderedDict()
        self.warm: OrderedDict[str, Any] = OrderedDict()
        self.expires_at: dict[str, float] = {}
        self.access_counts: dict[str, int] = {}
        self.sizes: dict[str, int] = {}
        self.hot_size = 0
        self.warm_size = 0
        self.cold_size = 0

    # ------------------------------------------------------------------
    # key helpers
    def _hash_key(self, query: str, tags: Iterable[str] | None) -> str:
        tags_part = "|".join(sorted(tags)) if tags else ""
        raw = f"{query}|{tags_part}".encode("utf-8")
        return hashlib.sha256(raw).hexdigest()

    # ------------------------------------------------------------------
    # tier maintenance
    def _promote_to_hot(self, key: str, value: Any) -> None:
        if key in self.warm:
            self.warm.pop(key, None)
            self.warm_size -= self.sizes.get(key, 0)
        if key not in self.hot:
            self.hot_size += self.sizes.get(key, 0)
        self.hot[key] = value
        self.hot.move_to_end(key)
        self._enforce_hot_limit()

    def _promote_to_warm(self, key: str, value: Any) -> None:
        if key in self.hot:
            self.hot.pop(key, None)
            self.hot_size -= self.sizes.get(key, 0)
        if key not in self.warm:
            self.warm_size += self.sizes.get(key, 0)
        self.warm[key] = value
        self.warm.move_to_end(key)
        self._enforce_warm_limit()

    def _enforce_hot_limit(self) -> None:
        while self.hot_size > self.hot_limit and self.hot:
            key = min(self.hot, key=lambda k: self.access_counts.get(k, 0))
            value = self.hot.pop(key)
            self.hot_size -= self.sizes.get(key, 0)
            self._promote_to_warm(key, value)

    def _enforce_warm_limit(self) -> None:
        while self.warm_size > self.warm_limit and self.warm:
            key = min(self.warm, key=lambda k: self.access_counts.get(k, 0))
            self.warm.pop(key, None)
            self.warm_size -= self.sizes.get(key, 0)

    def _enforce_cold_limit(self) -> None:
        while self.cold_size > self.cold_limit and self.sizes:
            key = min(self.sizes, key=lambda k: self.access_counts.get(k, 0))
            self._invalidate_key(key)

    def _invalidate_key(self, key: str) -> None:
        if key in self.hot:
            self.hot.pop(key, None)
            self.hot_size -= self.sizes.get(key, 0)
        if key in self.warm:
            self.warm.pop(key, None)
            self.warm_size -= self.sizes.get(key, 0)
        self.expires_at.pop(key, None)
        self.access_counts.pop(key, None)
        size = self.sizes.pop(key, 0)
        self.cold_size -= size
        super().invalidate(key)

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
        size = len(json.dumps(data, ensure_ascii=False).encode("utf-8"))
        prev = self.sizes.get(key, 0)
        self.cold_size += size - prev
        self.sizes[key] = size
        self.access_counts.setdefault(key, 0)
        super().set(key, data)
        self._promote_to_hot(key, value)
        self._enforce_cold_limit()

    def get(self, query: str, tags: Iterable[str] | None = None) -> Any | None:
        key = self._hash_key(query, tags)
        if self._is_expired(key):
            return None
        if key in self.hot:
            self.access_counts[key] = self.access_counts.get(key, 0) + 1
            self.hot.move_to_end(key)
            return self.hot[key]
        if key in self.warm:
            value = self.warm[key]
            self.access_counts[key] = self.access_counts.get(key, 0) + 1
            if self.access_counts[key] >= self.hot_threshold:
                self._promote_to_hot(key, value)
            else:
                self.warm.move_to_end(key)
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
        self.access_counts[key] = self.access_counts.get(key, 0) + 1
        if self.access_counts[key] >= self.hot_threshold:
            self._promote_to_hot(key, value)
        elif self.access_counts[key] >= self.warm_threshold:
            self._promote_to_warm(key, value)
        return value

    def invalidate(self, query: str | None = None, tags: Iterable[str] | None = None) -> None:
        if query is None:
            self.hot.clear()
            self.warm.clear()
            self.expires_at.clear()
            self.access_counts.clear()
            self.sizes.clear()
            self.hot_size = self.warm_size = self.cold_size = 0
            super().invalidate()
            return
        key = self._hash_key(query, tags)
        self._invalidate_key(key)

    # ------------------------------------------------------------------
    # expiration helpers
    def _is_expired(self, key: str) -> bool:
        exp = self.expires_at.get(key)
        if exp is not None and exp < time.time():
            self._invalidate_key(key)
            return True
        return False

    def cleanup(self) -> None:
        now = time.time()
        expired = [k for k, v in self.expires_at.items() if v < now]
        for key in expired:
            self._invalidate_key(key)
        self._enforce_cold_limit()
