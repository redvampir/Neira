from __future__ import annotations

from datetime import datetime, timedelta

from src.interaction.conflict_resolver import Resolution, Source, resolve_conflict


BASE_DATE = datetime(2024, 1, 1)


def _make_source(rel: float, days: int, ctx: set[str]) -> Source:
    return Source(reliability=rel, date=BASE_DATE + timedelta(days=days), context=ctx)


def test_reliability_determines_winner() -> None:
    a = [_make_source(0.9, 0, {"x"})]
    b = [_make_source(0.5, 10, {"x", "y"})]
    resolution = resolve_conflict(a, b)
    assert resolution.winner == "A"
    assert "higher average reliability" in resolution.explanation
    assert resolution.action == "use_sources_a"


def test_date_used_when_reliability_equal() -> None:
    a = [_make_source(0.7, 0, {"x"})]
    b = [_make_source(0.7, 5, {"x"})]
    resolution = resolve_conflict(a, b)
    assert resolution.winner == "B"
    assert "more recent sources" in resolution.explanation
    assert resolution.action == "use_sources_b"


def test_context_breaks_remaining_tie() -> None:
    a = [_make_source(0.6, 0, {"x", "y"})]
    b = [_make_source(0.6, 0, {"x"})]
    resolution = resolve_conflict(a, b)
    assert resolution.winner == "A"
    assert "broader context coverage" in resolution.explanation


def test_no_clear_winner_triggers_manual_review() -> None:
    a = [_make_source(0.6, 0, {"x"})]
    b = [_make_source(0.6, 0, {"x"})]
    resolution = resolve_conflict(a, b)
    assert resolution.winner is None
    assert resolution.action == "needs_manual_review"
