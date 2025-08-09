"""Template plugin for the code editor.

This file demonstrates the minimal structure required for plugins.
Developers can copy this template to start building their own
extensions.
"""

from __future__ import annotations

from dataclasses import dataclass

from src.plugins import Plugin


@dataclass
class ExamplePlugin(Plugin):
    """Small example plugin used as a template."""

    name: str = "Example"

    def on_activate(self) -> str:  # pragma: no cover - placeholder
        return f"{self.name} activated"

    def on_deactivate(self) -> str:  # pragma: no cover - placeholder
        return f"{self.name} deactivated"
