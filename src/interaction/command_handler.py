from __future__ import annotations

"""Utility helpers for processing user commands.

This module provides :func:`handle_command` which routes user input either
through the :class:`TagProcessor` slash command system or to the main
``Neyra`` instance.  The logic was previously embedded in the CLI and is
now shared with the web interface as well.
"""

from dataclasses import dataclass
from typing import Optional

from .tag_processor import TagProcessor


@dataclass
class CommandResult:
    """Result returned by :func:`handle_command`.

    Attributes
    ----------
    text:
        Response generated for the user.
    style:
        Optional style hint (``cyan``, ``magenta``, ``green``) used by the
        CLI and the web client for colouring the output.
    is_exit:
        Flag indicating that the application should terminate.
    """

    text: str = ""
    style: Optional[str] = None
    is_exit: bool = False


def handle_command(neyra, text: str, processor: TagProcessor) -> CommandResult:
    """Process a single user command."""

    clean = text.strip()
    if not clean:
        return CommandResult()

    if clean.startswith("/"):
        result = processor.execute_slash(clean)
        if result == "__exit__":
            return CommandResult(is_exit=True)
        return CommandResult(text=result or "", style="cyan")

    result = neyra.process_command(text)
    lower = result.lower()
    style = None
    if "@" in result:
        style = "cyan"
    elif "эмоци" in lower:
        style = "magenta"
    elif any(word in lower for word in ["опис", "сцена"]):
        style = "green"
    return CommandResult(text=result, style=style)


__all__ = ["CommandResult", "handle_command"]
