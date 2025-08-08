"""Utility functions for evaluating iteration progress."""

from __future__ import annotations

from difflib import SequenceMatcher
from typing import Dict

from src.core.config import get_logger

logger = get_logger(__name__)


def similarity(original: str, revised: str) -> float:
    """Return similarity ratio between two strings.

    The value is between ``0`` and ``1`` where ``1`` means the strings are
    identical. Comparison is performed on a character basis.
    """

    if not original and not revised:
        return 1.0
    return SequenceMatcher(None, original, revised).ratio()


def length(text: str) -> int:
    """Return the number of words in ``text``."""

    return len(text.split())


def corrected_errors(original: str, revised: str) -> int:
    """Estimate how many tokens were changed from ``original`` to ``revised``.

    Differences are computed on word level and include replacements, insertions
    and deletions.
    """

    matcher = SequenceMatcher(None, original.split(), revised.split())
    count = 0
    for tag, i1, i2, j1, j2 in matcher.get_opcodes():
        if tag == "replace":
            count += max(i2 - i1, j2 - j1)
        elif tag == "delete":
            count += i2 - i1
        elif tag == "insert":
            count += j2 - j1
    return count


def log_metrics(iteration: int, original: str, revised: str) -> Dict[str, float]:
    """Compute and log metrics for an iteration step."""

    metrics = {
        "iteration": iteration,
        "similarity": similarity(original, revised),
        "length": length(revised),
        "errors_corrected": corrected_errors(original, revised),
    }
    logger.info("Iteration %s metrics: %s", iteration, metrics)
    return metrics


__all__ = ["similarity", "length", "corrected_errors", "log_metrics"]
