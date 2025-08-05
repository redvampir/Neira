from __future__ import annotations

"""Three-level caching index for memory records."""

from typing import Any, Dict


class MemoryIndex:
    """Manage data between hot, warm and cold storage levels.

    The index keeps track of how often each entry is accessed via
    ``usage_stats``. Frequently used entries migrate upwards through the
    caching tiers:

    * Cold storage – baseline persistence for rarely used data.
    * Warm cache   – items accessed occasionally.
    * Hot cache    – frequently accessed items.

    Periodic checks promote lightly used cold entries to the warm cache and
    heavily used warm entries to the hot cache.
    """

    def __init__(self, hot_threshold: int = 5, warm_threshold: int = 2) -> None:
        self.hot_cache: Dict[str, Any] = {}
        self.warm_cache: Dict[str, Any] = {}
        self.cold_storage: Dict[str, Any] = {}
        self.usage_stats: Dict[str, int] = {}
        self.hot_threshold = hot_threshold
        self.warm_threshold = warm_threshold

    def set(self, key: str, value: Any) -> None:
        """Store a new entry in cold storage."""
        self.cold_storage[key] = value

    def get(self, key: str) -> Any:
        """Retrieve an entry from the index."""
        if key in self.hot_cache:
            self._increment_usage(key)
            return self.hot_cache[key]
        if key in self.warm_cache:
            self._increment_usage(key)
            self._promote_to_hot(key)
            if key in self.hot_cache:
                return self.hot_cache[key]
            return self.warm_cache[key]
        value = self._search_cold_storage(key)
        if value is not None:
            self._increment_usage(key)
        self._periodic_check()
        return value

    def _increment_usage(self, key: str) -> None:
        self.usage_stats[key] = self.usage_stats.get(key, 0) + 1

    def _promote_to_hot(self, key: str) -> None:
        """Move an entry from the warm cache to the hot cache."""
        if (
            key in self.warm_cache
            and self.usage_stats.get(key, 0) >= self.hot_threshold
        ):
            self.hot_cache[key] = self.warm_cache.pop(key)

    def _add_to_warm(self, key: str, value: Any) -> None:
        """Insert an entry into the warm cache removing it from cold storage."""
        self.warm_cache[key] = value
        self.cold_storage.pop(key, None)

    def _search_cold_storage(self, key: str) -> Any:
        """Look up an entry in cold storage."""
        return self.cold_storage.get(key)

    def _periodic_check(self) -> None:
        """Promote lightly used cold entries to the warm cache."""
        for key in list(self.cold_storage.keys()):
            if self.usage_stats.get(key, 0) >= self.warm_threshold:
                value = self.cold_storage[key]
                self._add_to_warm(key, value)


__all__ = ["MemoryIndex"]
