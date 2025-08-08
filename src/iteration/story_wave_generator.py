from __future__ import annotations

"""Utility for creating alternative "waves" of a draft."""

from typing import List


def generate_waves(draft: str, num_waves: int = 3) -> List[str]:
    """Return ``num_waves`` simple variations of ``draft``.

    The first wave is the original draft, subsequent waves are labelled
    variations.  This lightweight implementation acts as a placeholder for
    more sophisticated rewriting models.
    """

    waves: List[str] = []
    for i in range(num_waves):
        if i == 0:
            waves.append(draft)
        else:
            waves.append(f"{draft} [вариант {i + 1}]")
    return waves


__all__ = ["generate_waves"]
