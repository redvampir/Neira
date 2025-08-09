"""Simple UI for managing saved layouts."""

from __future__ import annotations

from typing import Any

from ui.layout_manager import LayoutManager


class LayoutEditor:  # pragma: no cover - GUI stub
    """Placeholder layout editor window.

    The real project would present controls to rearrange panels and persist
    them through :class:`LayoutManager`.  For the purposes of the tests this
    class merely wires up a manager instance.
    """

    def __init__(self, parent: Any | None = None, manager: LayoutManager | None = None) -> None:
        self.parent = parent
        self.manager = manager or LayoutManager()

    def open(self) -> None:
        """Open the editor window."""

        # Real implementation would spawn a GUI; this is intentionally left as
        # a stub to keep the test environment lightweight.
        pass


__all__ = ["LayoutEditor"]
