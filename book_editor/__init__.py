from __future__ import annotations

"""Autocomplete integration helpers for the book editor."""

from typing import List

from src.commands.autocomplete import get_suggestions


def context_menu_autocomplete(prefix: str, file_type: str = "markdown") -> List[str]:
    """Return autocomplete suggestions for the context menu.

    ``file_type`` defaults to ``"markdown"`` to mirror the typical content
    handled by the book editor.
    """

    return get_suggestions(prefix, file_type=file_type, mode="book_editor")


def panel_autocomplete(prefix: str, file_type: str = "markdown") -> List[str]:
    """Return autocomplete suggestions for the main editing panel."""

    return get_suggestions(prefix, file_type=file_type, mode="book_editor")


__all__ = ["context_menu_autocomplete", "panel_autocomplete"]
