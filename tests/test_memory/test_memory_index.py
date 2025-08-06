from __future__ import annotations

from src.memory.index import MemoryIndex


def test_promotes_cold_to_warm() -> None:
    index = MemoryIndex(hot_threshold=10, warm_threshold=2, hot_limit=5, warm_limit=5)
    index.set("alpha", 1)

    assert index.get("alpha") == 1  # first access, stays in cold
    assert "alpha" in index.cold_storage

    # Second access triggers promotion to warm cache
    index.get("alpha")
    assert "alpha" in index.warm_cache
    assert "alpha" not in index.cold_storage


def test_promotes_warm_to_hot() -> None:
    index = MemoryIndex(hot_threshold=3, warm_threshold=1, hot_limit=5, warm_limit=5)
    index.set("beta", 2)

    # First access moves beta to warm cache
    index.get("beta")
    assert "beta" in index.warm_cache

    # Further accesses promote beta to hot cache
    index.get("beta")
    index.get("beta")
    assert "beta" in index.hot_cache
    assert "beta" not in index.warm_cache


def test_demotes_inactive_items() -> None:
    index = MemoryIndex(hot_threshold=3, warm_threshold=1, hot_limit=5, warm_limit=5)
    index.set("alpha", 1)
    # Promote alpha to hot
    index.get("alpha")
    index.get("alpha")
    index.get("alpha")
    assert "alpha" in index.hot_cache

    # Use beta repeatedly so that alpha ages out to cold
    index.set("beta", 2)
    index.get("beta")
    index.get("beta")
    index.get("beta")
    index.get("beta")
    assert "alpha" in index.cold_storage
    assert "alpha" not in index.hot_cache
    assert "alpha" not in index.warm_cache


def test_hot_limit_eviction() -> None:
    index = MemoryIndex(hot_threshold=1, warm_threshold=0, hot_limit=1, warm_limit=1)
    index.set("a", 1)
    index.get("a")
    index.get("a")  # a promoted to hot

    index.set("b", 2)
    index.get("b")
    index.get("b")  # b promoted to hot, a evicted due to LRU

    assert "b" in index.hot_cache
    assert "a" in index.cold_storage


def test_warm_limit_eviction() -> None:
    index = MemoryIndex(hot_threshold=100, warm_threshold=0, hot_limit=5, warm_limit=1)
    index.set("a", 1)
    index.get("a")  # a warmed
    index.set("b", 2)
    index.get("b")  # b warmed, a evicted

    assert "b" in index.warm_cache
    assert "a" in index.cold_storage


def test_search_cold_storage() -> None:
    index = MemoryIndex()
    index.set("gamma", 3)
    assert index._search_cold_storage("gamma") == 3
