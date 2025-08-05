from __future__ import annotations

from src.memory.index import MemoryIndex


def test_promotes_cold_to_warm() -> None:
    index = MemoryIndex(hot_threshold=10, warm_threshold=2)
    index.set("alpha", 1)

    assert index.get("alpha") == 1  # first access, stays in cold
    assert "alpha" in index.cold_storage

    # Second access triggers promotion to warm cache
    index.get("alpha")
    assert "alpha" in index.warm_cache
    assert "alpha" not in index.cold_storage
    assert index.usage_stats["alpha"] == 2


def test_promotes_warm_to_hot() -> None:
    index = MemoryIndex(hot_threshold=3, warm_threshold=1)
    index.set("beta", 2)

    # First access moves beta to warm cache
    index.get("beta")
    assert "beta" in index.warm_cache

    # Further accesses promote beta to hot cache
    index.get("beta")
    index.get("beta")
    assert "beta" in index.hot_cache
    assert "beta" not in index.warm_cache
    assert index.usage_stats["beta"] == 3


def test_search_cold_storage() -> None:
    index = MemoryIndex()
    index.set("gamma", 3)
    assert index._search_cold_storage("gamma") == 3
