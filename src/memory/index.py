from __future__ import annotations

"""Three-level caching index for memory records."""

from collections import OrderedDict
import time
from typing import Any, Dict


class MemoryIndex:
    """Manage data between hot, warm and cold storage levels.

    The index tracks how often and how recently each entry is accessed. Items
    migrate between ``cold_storage`` (baseline persistence), ``warm_cache`` and
    ``hot_cache`` based on these usage statistics. Each tier applies an LRU
    policy with configurable size limits.
    """

    def __init__(
        self,
        hot_threshold: int = 5,
        warm_threshold: int = 2,
        hot_limit: int = 128,
        warm_limit: int = 256,
    ) -> None:
        self.hot_cache: "OrderedDict[str, Any]" = OrderedDict()
        self.warm_cache: "OrderedDict[str, Any]" = OrderedDict()
        self.cold_storage: Dict[str, Any] = {}
        self.usage_stats: Dict[str, int] = {}
        self.access_times: Dict[str, float] = {}
        self.hot_threshold = hot_threshold
        self.warm_threshold = warm_threshold
        self.hot_limit = hot_limit
        self.warm_limit = warm_limit

    # ------------------------------------------------------------------
    # public API
    def set(self, key: str, value: Any) -> None:
        """Store ``key``/``value`` in cold storage."""
        self.cold_storage[key] = value
        self.usage_stats[key] = 0
        self.access_times[key] = time.time()
        self._age_items(key)
        self._check_demotions()
        self._enforce_limits()

    def get(self, key: str) -> Any:
        """Retrieve ``key`` from whichever tier currently holds it."""
        if key in self.hot_cache:
            self._record_access(key, tier="hot")
            self._check_demotions()
            self._enforce_limits()
            return self.hot_cache[key]

        if key in self.warm_cache:
            self._record_access(key, tier="warm")
            self._promote_to_hot(key)
            self._check_demotions()
            self._enforce_limits()
            if key in self.hot_cache:
                return self.hot_cache[key]
            return self.warm_cache[key]

        value = self._search_cold_storage(key)
        if value is not None:
            self._record_access(key, tier="cold")
            if self.usage_stats.get(key, 0) >= self.warm_threshold:
                self._add_to_warm(key, value)
        self._check_demotions()
        self._enforce_limits()
        return value

    # ------------------------------------------------------------------
    # internal helpers
    def _record_access(self, key: str, tier: str) -> None:
        """Update bookkeeping for ``key`` being accessed."""
        self._age_items(key)
        self.usage_stats[key] = self.usage_stats.get(key, 0) + 1
        self.access_times[key] = time.time()
        if tier == "hot":
            self.hot_cache.move_to_end(key)
        elif tier == "warm":
            self.warm_cache.move_to_end(key)

    def _age_items(self, accessed_key: str) -> None:
        """Decrement usage counts for items other than ``accessed_key``."""
        for key in list(self.usage_stats.keys()):
            if key != accessed_key and self.usage_stats[key] > 0:
                self.usage_stats[key] -= 1

    def _check_demotions(self) -> None:
        """Demote entries whose usage falls below the tier thresholds."""
        for key in list(self.hot_cache.keys()):
            if self.usage_stats.get(key, 0) < self.hot_threshold:
                value = self.hot_cache.pop(key)
                self.warm_cache[key] = value
                self.usage_stats[key] = self.warm_threshold
        for key in list(self.warm_cache.keys()):
            if self.usage_stats.get(key, 0) < self.warm_threshold:
                value = self.warm_cache.pop(key)
                self.cold_storage[key] = value
                self.usage_stats[key] = 0

    def _enforce_limits(self) -> None:
        """Apply LRU eviction when cache size limits are exceeded."""
        while len(self.hot_cache) > self.hot_limit:
            key, value = self.hot_cache.popitem(last=False)
            self.warm_cache[key] = value
            self.usage_stats[key] = self.warm_threshold
        while len(self.warm_cache) > self.warm_limit:
            key, value = self.warm_cache.popitem(last=False)
            self.cold_storage[key] = value
            self.usage_stats[key] = 0

    def _promote_to_hot(self, key: str) -> None:
        """Move an entry from the warm cache to the hot cache."""
        if (
            key in self.warm_cache
            and self.usage_stats.get(key, 0) >= self.hot_threshold
        ):
            self.hot_cache[key] = self.warm_cache.pop(key)
            self.usage_stats[key] = self.hot_threshold
            self.access_times[key] = time.time()

    def _add_to_warm(self, key: str, value: Any) -> None:
        """Insert an entry into the warm cache removing it from cold storage."""
        self.warm_cache[key] = value
        self.cold_storage.pop(key, None)
        self.usage_stats[key] = self.warm_threshold
        self.access_times[key] = time.time()

    def _search_cold_storage(self, key: str) -> Any:
        """Look up an entry in cold storage."""
        return self.cold_storage.get(key)


__all__ = ["MemoryIndex"]

