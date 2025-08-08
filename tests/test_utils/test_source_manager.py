from __future__ import annotations

from src.utils.source_manager import SourceManager, calculate_source_limit


def test_register_merges_duplicates_by_reliability() -> None:
    manager = SourceManager()
    manager.register("old", "https://example.com", 0.6)
    manager.register("new", "https://example.com", 0.9)
    sources = manager.all()
    assert len(sources) == 1
    assert sources[0].summary == "new"
    assert sources[0].reliability == 0.9


def test_calculate_source_limit_scales_with_reliability() -> None:
    assert calculate_source_limit("high", base_limit=10) > calculate_source_limit(
        "low", base_limit=10
    )


def test_limit_sources_restricts_total() -> None:
    manager = SourceManager()
    # Register multiple sources with decreasing reliability
    for idx in range(5):
        manager.register(f"s{idx}", f"u{idx}", 1 - idx * 0.1)

    limit = calculate_source_limit("low")
    manager.limit_sources({"reliability_level": "low"})
    sources = manager.all()
    assert len(sources) == limit
    # Ensure the most reliable source remains
    assert sources[0].summary == "s0"
