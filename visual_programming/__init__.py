"""Utilities for the visual programming subsystem."""

from typing import List

from src.commands.autocomplete import get_suggestions

from .translation_sync import TranslationSync
from .node_palette import NodePalette, NodeTemplate
from .history import History
from .error_highlight import highlight_errors
from .nodes.html_css import HTMLElement, CSSRule, export_html_css

try:  # pragma: no cover - optional dependency
    from .collab_client import VisualCollabClient
except Exception:  # pragma: no cover - gracefully degrade when deps missing
    VisualCollabClient = None  # type: ignore[assignment]


def context_menu_autocomplete(prefix: str, file_type: str = "python") -> List[str]:
    """Return autocomplete suggestions for visual programming context menus."""

    return get_suggestions(prefix, file_type=file_type, mode="visual_programming")


def panel_autocomplete(prefix: str, file_type: str = "python") -> List[str]:
    """Return autocomplete entries for the visual editor panel."""

    return get_suggestions(prefix, file_type=file_type, mode="visual_programming")

__all__ = [
    "TranslationSync",
    "NodePalette",
    "NodeTemplate",
    "History",
    "highlight_errors",
    "HTMLElement",
    "CSSRule",
    "export_html_css",
    "VisualCollabClient",
    "context_menu_autocomplete",
    "panel_autocomplete",
]
