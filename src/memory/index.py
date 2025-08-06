from __future__ import annotations

"""Three-level caching index for memory records."""

from collections import OrderedDict
import time
from typing import Any, Dict, Set
from pathlib import Path

try:
    import yaml
except Exception:  # pragma: no cover - optional dependency
    yaml = None  # type: ignore


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
        dedup_threshold: float | None = None,
        config_path: str | Path | None = None,
    ) -> None:
        self.hot_cache: "OrderedDict[str, Any]" = OrderedDict()
        self.warm_cache: "OrderedDict[str, Any]" = OrderedDict()
        self.cold_storage: Dict[str, Any] = {}
        self.usage_stats: Dict[str, int] = {}
        self.access_times: Dict[str, float] = {}
        # Track how trustworthy each source is perceived to be. 1.0 means
        # highly reliable while 0.0 indicates unknown or untrusted source.
        self.source_reliability: Dict[str, float] = {}
        self.hot_threshold = hot_threshold
        self.warm_threshold = warm_threshold
        self.hot_limit = hot_limit
        self.warm_limit = warm_limit
        self.dedup_threshold = (
            dedup_threshold
            if dedup_threshold is not None
            else self._load_dedup_threshold(config_path)
        )
        self._fingerprints: Dict[str, Set[str]] = {}

    # ------------------------------------------------------------------
    # public API
    def set(self, key: str, value: Any, reliability: float = 0.5) -> None:
        """Store ``key``/``value`` in cold storage if not a duplicate.

        Parameters
        ----------
        key:
            Identifier for the memory record.
        value:
            The data to store.
        reliability:
            A float between 0 and 1 representing how trustworthy the source
            of this record is. Defaults to ``0.5`` which means neutral
            reliability.
        """
        text = str(value)
        tokens = self._tokenize(text)
        if self._is_duplicate(tokens):
            return
        self._fingerprints[key] = tokens
        self.cold_storage[key] = value
        self.usage_stats[key] = 0
        self.access_times[key] = time.time()
        self.source_reliability[key] = max(0.0, min(1.0, reliability))
        self._age_items(key)
        self._check_demotions()
        self._enforce_limits()

    def update_reliability(self, key: str, reliability: float) -> None:
        """Update reliability for ``key`` if it exists."""
        if key in self.cold_storage or key in self.warm_cache or key in self.hot_cache:
            self.source_reliability[key] = max(0.0, min(1.0, reliability))

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

    def _load_dedup_threshold(self, path: str | Path | None) -> float:
        """Load deduplication threshold from config file."""
        config_path = (
            Path(path)
            if path is not None
            else Path(__file__).resolve().parents[2] / "config" / "memory.yml"
        )
        if yaml is None:
            return 1.0
        try:
            with config_path.open("r", encoding="utf-8") as f:
                data = yaml.safe_load(f) or {}
        except Exception:  # pragma: no cover - missing or invalid config
            data = {}
        return float(data.get("deduplication_threshold", 1.0))

    def _tokenize(self, text: str) -> Set[str]:
        """Tokenize text into a set of lowercase words."""
        return {tok for tok in text.lower().split() if tok}

    def _jaccard(self, a: Set[str], b: Set[str]) -> float:
        """Compute Jaccard similarity between two token sets."""
        if not a and not b:
            return 1.0
        union = len(a | b)
        if union == 0:
            return 0.0
        return len(a & b) / union

    def _is_duplicate(self, tokens: Set[str]) -> bool:
        """Check if tokens are similar to existing fingerprints."""
        for existing in self._fingerprints.values():
            if self._jaccard(tokens, existing) >= self.dedup_threshold:
                return True
        return False


__all__ = ["MemoryIndex"]

