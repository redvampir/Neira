"""Tests for the SourceTracker utility."""

import logging
import pytest

from src.utils.source_tracker import SourceTracker


def test_add_source_and_format(caplog) -> None:
    tracker = SourceTracker()
    with caplog.at_level(logging.INFO, logger="src.utils.source_tracker"):
        tracker.add("fact", "http://example.com", 0.9)
    assert tracker.get_sources() == ["http://example.com"]
    assert "http://example.com" in tracker.format_citations()
    assert "Источник принят" in caplog.text


def test_block_unreliable_source() -> None:
    tracker = SourceTracker(reliability_threshold=0.8)
    with pytest.raises(ValueError):
        tracker.add("fact", "http://bad.com", 0.5)
