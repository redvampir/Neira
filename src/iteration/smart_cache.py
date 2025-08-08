from __future__ import annotations

from collections import OrderedDict
from pathlib import Path
from typing import Any, Iterable
import gzip
import hashlib
import json
import shutil
import threading
import time

from src.core.cache_manager import CacheManager


class SmartCache(CacheManager):
    """Cache with hot/warm/cold tiers and predictive prefetching.

    * ``hot``  – in-memory ``OrderedDict`` with LRU eviction.
    * ``warm`` – plain JSON files on disk handled by :class:`CacheManager`.
    * ``cold`` – compressed archives stored under ``cache_dir/archive``.

    Basic access statistics are tracked in ``access_history`` and an
    exponentially smoothed value is used to forecast repeated requests.  When
    a key is predicted to be accessed again soon it is proactively promoted to
    the hot tier.
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
        smoothing_alpha: float = 0.5,
        prefetch_threshold: float = 0.7,
        history_limit: int = 20,
        stale_after: float | None = None,
        cleanup_interval: float | None = None,
    ) -> None:
        super().__init__(cache_dir)
        self.hot_limit = int(hot_limit_mb * 1024 * 1024)
        self.warm_limit = int(warm_limit_mb * 1024 * 1024)
        self.cold_limit = int(cold_limit_mb * 1024 * 1024)
        self.warm_threshold = warm_threshold
        self.hot_threshold = hot_threshold
        self.default_ttl = default_ttl

        self.smoothing_alpha = smoothing_alpha
        self.prefetch_threshold = prefetch_threshold
        self.history_limit = history_limit
        self.stale_after = stale_after
        self.cleanup_interval = cleanup_interval

        self.hot: OrderedDict[str, Any] = OrderedDict()
        self.warm_keys: OrderedDict[str, None] = OrderedDict()
        self.cold_dir = Path(self.cache_dir) / "archive"
        self.cold_dir.mkdir(exist_ok=True)

        self.expires_at: dict[str, float] = {}
        self.access_counts: dict[str, int] = {}
        self.sizes: dict[str, int] = {}
        self.cold_sizes: dict[str, int] = {}
        self.last_access: dict[str, float] = {}
        self.access_history: dict[str, list[float]] = {}
        self.smoothed_access: dict[str, float] = {}

        self.hot_size = 0
        self.warm_size = 0
        self.cold_size = 0

        if cleanup_interval:
            self._schedule_cleanup()

    # ------------------------------------------------------------------
    # key helpers
    def _hash_key(self, query: str, tags: Iterable[str] | None) -> str:
        tags_part = "|".join(sorted(tags)) if tags else ""
        raw = f"{query}|{tags_part}".encode("utf-8")
        return hashlib.sha256(raw).hexdigest()

    # ------------------------------------------------------------------
    # tier maintenance
    def _promote_to_hot(self, key: str, value: Any) -> None:
        if key not in self.hot:
            self.hot_size += self.sizes.get(key, 0)
        self.hot[key] = value
        self.hot.move_to_end(key)
        self._enforce_hot_limit()

    def _move_to_cold(self, key: str) -> None:
        warm_path = self._path_for(key)
        if not warm_path.exists():
            return
        cold_path = self.cold_dir / f"{warm_path.name}.gz"
        with open(warm_path, "rb") as f_in, gzip.open(cold_path, "wb") as f_out:
            shutil.copyfileobj(f_in, f_out)
        warm_size = warm_path.stat().st_size
        cold_size = cold_path.stat().st_size
        warm_path.unlink()
        self.warm_keys.pop(key, None)
        self.warm_size -= warm_size
        self.cold_size += cold_size
        self.cold_sizes[key] = cold_size
        if key in self.hot:
            self.hot_size -= self.sizes.get(key, 0)
            self.hot.pop(key, None)

    def _load_from_cold(self, key: str) -> Any | None:
        cold_path = self.cold_dir / f"{self._path_for(key).name}.gz"
        if not cold_path.exists():
            return None
        with gzip.open(cold_path, "rt", encoding="utf-8") as fh:
            data = json.load(fh)
        cold_size = self.cold_sizes.pop(key, cold_path.stat().st_size)
        self.cold_size -= cold_size
        size = len(json.dumps(data, ensure_ascii=False).encode("utf-8"))
        self.sizes[key] = size
        self.warm_size += size
        super().set(key, data)
        self.warm_keys[key] = None
        return data

    def _enforce_hot_limit(self) -> None:
        while self.hot_size > self.hot_limit and self.hot:
            key = min(self.hot, key=lambda k: self.access_counts.get(k, 0))
            self.hot_size -= self.sizes.get(key, 0)
            self.hot.pop(key, None)

    def _enforce_warm_limit(self) -> None:
        while self.warm_size > self.warm_limit and self.warm_keys:
            candidates = [k for k in self.warm_keys if k not in self.hot]
            if not candidates:
                break
            key = min(candidates, key=lambda k: self.access_counts.get(k, 0))
            self._move_to_cold(key)

    def _enforce_cold_limit(self) -> None:
        while self.cold_size > self.cold_limit and self.cold_sizes:
            key = min(self.cold_sizes, key=lambda k: self.access_counts.get(k, 0))
            self._invalidate_key(key)

    def _record_access(self, key: str, value: Any | None) -> None:
        now = time.time()
        hist = self.access_history.setdefault(key, [])
        hist.append(now)
        if len(hist) > self.history_limit:
            hist.pop(0)
        prev = self.smoothed_access.get(key, 0.0)
        self.smoothed_access[key] = self.smoothing_alpha + (1 - self.smoothing_alpha) * prev
        self.access_counts[key] = self.access_counts.get(key, 0) + 1
        self.last_access[key] = now
        if value is not None and self.smoothed_access[key] >= self.prefetch_threshold and key not in self.hot:
            self._promote_to_hot(key, value)

    def _invalidate_key(self, key: str) -> None:
        if key in self.hot:
            self.hot_size -= self.sizes.get(key, 0)
            self.hot.pop(key, None)
        if key in self.warm_keys:
            warm_path = self._path_for(key)
            if warm_path.exists():
                size = warm_path.stat().st_size
                self.warm_size -= size
                warm_path.unlink()
            self.warm_keys.pop(key, None)
        if key in self.cold_sizes:
            cold_path = self.cold_dir / f"{self._path_for(key).name}.gz"
            if cold_path.exists():
                size = self.cold_sizes.get(key, cold_path.stat().st_size)
                self.cold_size -= size
                cold_path.unlink()
            self.cold_sizes.pop(key, None)
        self.expires_at.pop(key, None)
        self.access_counts.pop(key, None)
        self.sizes.pop(key, None)
        self.last_access.pop(key, None)
        self.access_history.pop(key, None)
        self.smoothed_access.pop(key, None)
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
        now = time.time()
        ttl_value = ttl if ttl is not None else self.default_ttl
        if ttl_value is not None:
            self.expires_at[key] = now + ttl_value
        else:
            self.expires_at.pop(key, None)
        data = {"value": value, "tags": list(tags) if tags else []}
        exp = self.expires_at.get(key)
        if exp is not None:
            data["expires_at"] = exp
        size = len(json.dumps(data, ensure_ascii=False).encode("utf-8"))
        prev = self.sizes.get(key, 0)
        self.warm_size += size - prev
        self.sizes[key] = size
        self.access_counts.setdefault(key, 0)
        self.last_access[key] = now
        self.access_history.setdefault(key, [])
        self.smoothed_access.setdefault(key, 0.0)
        super().set(key, data)
        self.warm_keys[key] = None
        self.warm_keys.move_to_end(key)
        self._enforce_warm_limit()
        self._enforce_cold_limit()

    def get(self, query: str, tags: Iterable[str] | None = None) -> Any | None:
        key = self._hash_key(query, tags)
        if self._is_expired(key):
            return None
        if key in self.hot:
            value = self.hot[key]
            self.hot.move_to_end(key)
            self._record_access(key, value)
            return value
        if key in self.warm_keys:
            data = super().get(key)
            if data is None:
                self.warm_keys.pop(key, None)
                return None
            value = data["value"] if isinstance(data, dict) and "value" in data else data
            self.warm_keys.move_to_end(key)
            self._record_access(key, value)
            if self.access_counts[key] >= self.hot_threshold:
                self._promote_to_hot(key, value)
            return value
        warm_path = self._path_for(key)
        if warm_path.exists():
            data = super().get(key)
            if data is None:
                return None
            size = len(json.dumps(data, ensure_ascii=False).encode("utf-8"))
            self.sizes[key] = size
            self.warm_size += size
            self.warm_keys[key] = None
            value = data["value"] if isinstance(data, dict) and "value" in data else data
            self._record_access(key, value)
            if self.access_counts[key] >= self.hot_threshold:
                self._promote_to_hot(key, value)
            return value
        data = self._load_from_cold(key)
        if data is None:
            return None
        value = data["value"] if isinstance(data, dict) and "value" in data else data
        self._record_access(key, value)
        if self.access_counts[key] >= self.hot_threshold:
            self._promote_to_hot(key, value)
        return value

    def invalidate(self, query: str | None = None, tags: Iterable[str] | None = None) -> None:
        if query is None:
            for key in list(set(self.sizes) | set(self.cold_sizes)):
                self._invalidate_key(key)
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
        if self.stale_after is not None:
            stale = [k for k, v in self.last_access.items() if now - v > self.stale_after]
            for key in stale:
                self._invalidate_key(key)
        self._enforce_warm_limit()
        self._enforce_cold_limit()

    # ------------------------------------------------------------------
    # automatic cleanup
    def _schedule_cleanup(self) -> None:
        timer = threading.Timer(self.cleanup_interval, self._auto_cleanup)
        timer.daemon = True
        timer.start()
        self._cleanup_timer = timer

    def _auto_cleanup(self) -> None:
        try:
            self.cleanup()
        finally:
            self._schedule_cleanup()

