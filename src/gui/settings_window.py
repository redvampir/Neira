"""Window allowing the user to tweak application settings."""

from __future__ import annotations

from typing import Any

from .layout_editor import LayoutEditor


class SettingsWindow:  # pragma: no cover - GUI stub
    """Placeholder settings window with access to a layout editor."""

    def __init__(self, parent: Any | None = None) -> None:
        self.parent = parent
        self.setup_window()

    def setup_window(self) -> None:
        """Initialise child tools such as the layout editor."""

        self.layout_editor = LayoutEditor(self)

    def open_layout_editor(self) -> None:
        """Open the layout editor window."""

        self.layout_editor.open()
