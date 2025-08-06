from __future__ import annotations

from src.memory.index import MemoryIndex


def test_deduplicates_exact_matches() -> None:
    index = MemoryIndex(dedup_threshold=1.0)
    index.set("a", "hello world")
    index.set("b", "hello world")

    assert "a" in index.cold_storage
    assert "b" not in index.cold_storage


def test_respects_similarity_threshold() -> None:
    text1 = "hello world"
    text2 = "hello there world"

    low_threshold_index = MemoryIndex(dedup_threshold=0.5)
    low_threshold_index.set("a", text1)
    low_threshold_index.set("b", text2)
    assert "b" not in low_threshold_index.cold_storage

    high_threshold_index = MemoryIndex(dedup_threshold=0.8)
    high_threshold_index.set("a", text1)
    high_threshold_index.set("b", text2)
    assert "b" in high_threshold_index.cold_storage
