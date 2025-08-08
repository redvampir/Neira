from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime
from typing import Iterable, Sequence, Set


@dataclass
class Source:
    """Simple representation of an information source."""

    reliability: float
    date: datetime
    context: Set[str]


@dataclass
class Resolution:
    """Result of conflict resolution between two sets of sources."""

    winner: str | None
    explanation: str
    action: str


def _score_sources(sources: Sequence[Source]) -> tuple[float, datetime, int]:
    """Return aggregate metrics for ``sources``.

    Metrics are average reliability, most recent date and context coverage
    (number of unique context tokens).
    """

    if not sources:
        return 0.0, datetime.min, 0
    avg_reliability = sum(s.reliability for s in sources) / len(sources)
    latest_date = max(s.date for s in sources)
    context_tokens: Set[str] = set()
    for src in sources:
        context_tokens.update(src.context)
    return avg_reliability, latest_date, len(context_tokens)


def resolve_conflict(sources_a: Sequence[Source], sources_b: Sequence[Source]) -> Resolution:
    """Compare two lists of sources and select the stronger one.

    The comparison is performed lexicographically by the following metrics:

    1. Average reliability (higher is better)
    2. Most recent publication date (more recent is better)
    3. Context coverage – number of unique context tokens (higher is better)

    Returns a :class:`Resolution` describing which side won, an explanation and
    the required action.
    """

    score_a = _score_sources(sources_a)
    score_b = _score_sources(sources_b)

    explanation_parts = []
    winner: str | None = None

    if score_a[0] > score_b[0]:
        winner = "A"
        explanation_parts.append(
            f"higher average reliability ({score_a[0]:.2f} > {score_b[0]:.2f})"
        )
    elif score_b[0] > score_a[0]:
        winner = "B"
        explanation_parts.append(
            f"higher average reliability ({score_b[0]:.2f} > {score_a[0]:.2f})"
        )
    elif score_a[1] > score_b[1]:
        winner = "A"
        explanation_parts.append(
            f"more recent sources ({score_a[1].date():%Y-%m-%d} > {score_b[1].date():%Y-%m-%d})"
        )
    elif score_b[1] > score_a[1]:
        winner = "B"
        explanation_parts.append(
            f"more recent sources ({score_b[1].date():%Y-%m-%d} > {score_a[1].date():%Y-%m-%d})"
        )
    elif score_a[2] > score_b[2]:
        winner = "A"
        explanation_parts.append(
            f"broader context coverage ({score_a[2]} > {score_b[2]})"
        )
    elif score_b[2] > score_a[2]:
        winner = "B"
        explanation_parts.append(
            f"broader context coverage ({score_b[2]} > {score_a[2]})"
        )
    else:
        explanation_parts.append("no decisive advantage")

    if winner == "A":
        action = "use_sources_a"
    elif winner == "B":
        action = "use_sources_b"
    else:
        action = "needs_manual_review"

    explanation = "; ".join(explanation_parts)
    return Resolution(winner=winner, explanation=explanation, action=action)


__all__ = ["Source", "Resolution", "resolve_conflict"]
