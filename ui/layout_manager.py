from __future__ import annotations

"""Manage docking layouts for UI panels with JSON persistence.

This module offers a very small API to dock and undock panels and to
persist layouts under ``userdata/layouts``.  Layouts are stored as JSON
mappings from panel identifiers to a dictionary describing their docked
state.

Example
-------
>>> from ui.layout_manager import LayoutManager
>>> lm = LayoutManager()
>>> lm.dock_panel("chat", "left")
>>> lm.save_layout("my_layout")
"""

from dataclasses import dataclass, field
from pathlib import Path
import json
from typing import Dict, Any, List

# --------------------------------------------------------------------------- Paths
ROOT_DIR = Path(__file__).resolve().parents[1]
LAYOUTS_DIR = ROOT_DIR / "userdata" / "layouts"


@dataclass
class LayoutManager:
    """Keep track of panel docking information and persist layouts."""

    layout: Dict[str, Dict[str, Any]] = field(default_factory=dict)

    # ------------------------------------------------------------------ docking
    def dock_panel(self, panel_id: str, position: str) -> None:
        """Dock ``panel_id`` at ``position``.

        Parameters
        ----------
        panel_id:
            Identifier of the panel to dock.
        position:
            Arbitrary string describing the target area (e.g. ``"left"``).
        """

        self.layout[panel_id] = {"docked": True, "position": position}

    def undock_panel(self, panel_id: str) -> None:
        """Mark ``panel_id`` as floating (undocked)."""

        info = self.layout.get(panel_id, {"position": None})
        info.update({"docked": False, "position": None})
        self.layout[panel_id] = info

    # ------------------------------------------------------------------ persistence
    def save_layout(self, name: str) -> Path:
        """Save the current layout under ``name``.

        Returns the path of the written JSON file.
        """

        LAYOUTS_DIR.mkdir(parents=True, exist_ok=True)
        path = LAYOUTS_DIR / f"{name}.json"
        with path.open("w", encoding="utf-8") as fh:
            json.dump(self.layout, fh, ensure_ascii=False, indent=2)
        return path

    def load_layout(self, name: str) -> None:
        """Load layout ``name`` replacing the current one."""

        path = LAYOUTS_DIR / f"{name}.json"
        with path.open("r", encoding="utf-8") as fh:
            self.layout = json.load(fh)

    def list_layouts(self) -> List[str]:
        """Return identifiers of available saved layouts."""

        if not LAYOUTS_DIR.exists():
            return []
        return [p.stem for p in LAYOUTS_DIR.glob("*.json")]


__all__ = [
    "LayoutManager",
    "LAYOUTS_DIR",
]
