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
    cache = SmartCache(cache_dir=tmp_path, hot_limit=1, warm_limit=1)
    cache.set("q1", "v1")
    cache.set("q2", "v2")
    cache.get("q1")  # promote q1 -> hot, q2 -> warm
    cache.set("q3", "v3")  # q2 should move to cold storage
    key2 = cache._hash_key("q2", None)
    assert key2 not in cache.hot and key2 not in cache.warm
    # retrieving should bring it back from cold
    assert cache.get("q2") == "v2"
    assert key2 in cache.warm or key2 in cache.hot


def test_persistence_across_instances(tmp_path):
    cache = SmartCache(cache_dir=tmp_path)
    cache.set("q", 123, tags=["a"])
    other = SmartCache(cache_dir=tmp_path)
    assert other.get("q", tags=["a"]) == 123
