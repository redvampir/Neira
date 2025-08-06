"""Utilities for classifying errors and recommending corrective actions."""
from __future__ import annotations

from typing import Any, Dict


def classify_error(interaction: Dict[str, Any]) -> str:
    """Classify the type of error that occurred.

    The classification is heuristic and based on the content of the response
    and optional context metadata. It returns one of ``"logical"``,
    ``"linguistic"`` or ``"system"``.
    """

    response = (interaction.get("response") or "").lower()
    context = interaction.get("context") or {}

    # Basic heuristics: system errors contain typical error keywords
    if any(keyword in response for keyword in ["traceback", "exception", "error"]):
        return "system"

    # Linguistic issues flagged explicitly via context
    if context.get("lang_issue"):
        return "linguistic"

    # Default to logical errors (incorrect or nonsensical answers)
    return "logical"


def recommend_action(error_type: str) -> str:
    """Provide a recommendation for handling an error type."""

    if error_type == "system":
        return "Consider switching the model or checking system configuration."
    if error_type == "linguistic":
        return "Try rephrasing or adjusting the prompt wording."
    return "Provide clearer instructions or use a more capable model."


__all__ = ["classify_error", "recommend_action"]
