"""Utilities for context-sensitive help and walkthroughs.

This module provides a small registry that maps UI element identifiers to
short hint texts.  It also exposes helpers to register walkthrough functions
for different modes and installs a global ``F1`` key binding that pops up a
mini chat with the relevant hint.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Callable, Dict, Optional

try:  # pragma: no cover - optional dependency
    from prompt_toolkit.key_binding import KeyBindings
except Exception:  # pragma: no cover - library may not be installed
    KeyBindings = None  # type: ignore


# ---------------------------------------------------------------------------
# Helper implementation


def _default_open_chat(message: str) -> None:
    """Fallback mini chat implementation.

    In environments where the real chat UI is unavailable the helper simply
    prints the message.  The callable can be replaced with a custom
    implementation by assigning to :attr:`ContextHelper.open_chat`.
    """

    print(f"[mini-chat] {message}")


@dataclass
class ContextHelper:
    """Central registry for context hints and walkthrough callbacks."""

    hints: Dict[str, str] = field(default_factory=dict)
    walkthroughs: Dict[str, Callable[[], None]] = field(default_factory=dict)
    open_chat: Callable[[str], None] = _default_open_chat

    # ------------------------------------------------------------------ hints
    def register_hint(self, element_id: str, text: str) -> None:
        """Associate ``text`` with ``element_id``."""

        self.hints[element_id] = text

    def get_hint(self, element_id: str) -> Optional[str]:
        """Return the hint text for ``element_id`` if available."""

        return self.hints.get(element_id)

    # --------------------------------------------------------------- walkthrough
    def register_walkthrough(self, mode: str, func: Callable[[], None]) -> None:
        """Register a walkthrough callback for ``mode``."""

        self.walkthroughs[mode] = func

    def start_walkthrough(self, mode: str) -> None:
        """Execute the walkthrough associated with ``mode`` if present."""

        walkthrough = self.walkthroughs.get(mode)
        if walkthrough:
            walkthrough()

    # -------------------------------------------------------------- key binding
    def install_f1(
        self, bindings: KeyBindings, current_element: Callable[[], str]
    ) -> None:
        """Install an ``F1`` binding on ``bindings`` that opens mini chat.

        Parameters
        ----------
        bindings:
            ``prompt_toolkit`` :class:`KeyBindings` instance to extend.
        current_element:
            Callable returning the identifier of the currently focused UI
            element.
        """

        if KeyBindings is None:
            return

        @bindings.add("f1")
        def _show_help(event) -> None:  # pragma: no cover - interactive helper
            element_id = current_element()
            hint = self.get_hint(element_id)
            if hint:
                self.open_chat(hint)


# Global helper instance -------------------------------------------------------
helper = ContextHelper()

# Convenience re-exports -------------------------------------------------------
register_hint = helper.register_hint
register_walkthrough = helper.register_walkthrough
start_walkthrough = helper.start_walkthrough
install_f1 = helper.install_f1
get_hint = helper.get_hint
