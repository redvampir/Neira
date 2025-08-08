import pytest
import sys
import types
from pathlib import Path

# stub packages to avoid heavy imports
root = Path(__file__).resolve().parents[2]
src_pkg = types.ModuleType("src")
src_pkg.__path__ = [str(root / "src")]
sys.modules.setdefault("src", src_pkg)

iteration_pkg = types.ModuleType("src.iteration")
iteration_pkg.__path__ = [str(root / "src" / "iteration")]
sys.modules.setdefault("src.iteration", iteration_pkg)

core_pkg = types.ModuleType("src.core")
core_pkg.__path__ = [str(root / "src" / "core")]
sys.modules.setdefault("src.core", core_pkg)

neira_rust_stub = types.ModuleType("neira_rust")
neira_rust_stub.KnowledgeGraph = object
neira_rust_stub.MemoryIndex = object
neira_rust_stub.ping = lambda: "pong"
neira_rust_stub.VerificationResult = object
neira_rust_stub.verify_claim = lambda *a, **k: neira_rust_stub.VerificationResult
sys.modules.setdefault("neira_rust", neira_rust_stub)

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


def test_archive_and_restore(tmp_path):
    cache = SmartCache(
        cache_dir=tmp_path,
        hot_limit_mb=1,
        warm_limit_mb=0.00005,  # force archive after second item
        cold_limit_mb=1,
    )
    cache.set("q1", "v1")
    key1 = cache._hash_key("q1", None)
    warm_path = cache._path_for(key1)
    cache.set("q2", "v2")  # triggers move of q1 to cold
    assert not warm_path.exists()
    archive_path = cache.cold_dir / f"{warm_path.name}.gz"
    assert archive_path.exists()
    assert cache.get("q1") == "v1"
    assert warm_path.exists()


def test_prefetch_promotes_to_hot(tmp_path):
    cache = SmartCache(
        cache_dir=tmp_path,
        prefetch_threshold=0.6,
        hot_threshold=100,
    )
    cache.set("q", "v")
    key = cache._hash_key("q", None)
    assert key not in cache.hot
    cache.get("q")
    assert key not in cache.hot
    cache.get("q")  # second access triggers prefetch
    assert key in cache.hot


def test_persistence_across_instances(tmp_path):
    cache = SmartCache(cache_dir=tmp_path)
    cache.set("q", 123, tags=["a"])
    other = SmartCache(cache_dir=tmp_path)
    assert other.get("q", tags=["a"]) == 123
