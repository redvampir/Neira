from __future__ import annotations

"""Adapt response tone based on context and iteration count."""

from typing import Any

# Predefined response styles.
STYLES: dict[str, str] = {
    "confident_but_open": "I am confident in this information, but open to feedback.",
    "curious_investigator": "Let's explore this further with curiosity.",
    "respectful_collaboration": "Working together respectfully for the best answer.",
    "default_helpful": "Here's what I found.",
}


def adapt_response_style(context: Any, iteration_count: int) -> str:
    """Return a style label for a response.

    Parameters
    ----------
    context:
        Arbitrary context data that may include a ``tone`` hint.
    iteration_count:
        Number of refinement iterations performed during generation.
    """

    tone = getattr(context, "get", lambda key, default=None: default)("tone", None)

    if tone == "curious":
        return "curious_investigator"
    if tone == "collaborative":
        return "respectful_collaboration"
    if iteration_count > 0:
        return "confident_but_open"
    return "default_helpful"


__all__ = ["adapt_response_style", "STYLES"]
