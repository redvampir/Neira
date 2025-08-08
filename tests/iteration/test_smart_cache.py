import pytest

from src.iteration.smart_cache import SmartCache


def test_tags_affect_key(tmp_path):
    cache = SmartCache(cache_dir=tmp_path)
    cache.set("hello", "A", tags=["t1"])
    cache.set("hello", "B", tags=["t2"])
    assert cache.get("hello", tags=["t1"]) == "A"
    assert cache.get("hello", tags=["t2"]) == "B"
    cache.set("foo", "bar", tags=["x", "y"])
    # Order of tags should not matter
    assert cache.get("foo", tags=["y", "x"]) == "bar"


def test_tier_promotion_and_cold_fetch(tmp_path):
    cache = SmartCache(
        cache_dir=tmp_path,
        hot_limit_mb=0.00005,
        warm_limit_mb=0.0001,
        warm_threshold=2,
        hot_threshold=3,
    )
    cache.set("q1", "v1")
    cache.get("q1")
    cache.set("q2", "v2")
    key2 = cache._hash_key("q2", None)
    size2 = cache.sizes[key2]
    cache.warm.pop(key2, None)
    cache.warm_size -= size2
    assert key2 not in cache.hot and key2 not in cache.warm
    assert cache.get("q2") == "v2"  # first access keeps it cold
    assert key2 not in cache.hot and key2 not in cache.warm
    cache.get("q2")  # second access -> warm
    assert key2 in cache.warm
    cache.get("q2")  # third access -> hot
    assert key2 in cache.hot


def test_persistence_across_instances(tmp_path):
    cache = SmartCache(cache_dir=tmp_path)
    cache.set("q", 123, tags=["a"])
    other = SmartCache(cache_dir=tmp_path)
    assert other.get("q", tags=["a"]) == 123
