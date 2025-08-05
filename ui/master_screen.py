"""Screen used by the game master to control the session."""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Dict


@dataclass
class MasterScreen:
    """Collection of tools available to the game master."""

    tools: Dict[str, Any] = field(default_factory=dict)

    def add_tool(self, name: str, tool: Any) -> None:
        """Register a tool on the master screen."""
        self.tools[name] = tool

    def serialize(self) -> Dict[str, Any]:
        """Return a serialisable mapping of the screen's tools."""
        serialised: Dict[str, Any] = {}
        for name, tool in self.tools.items():
            if hasattr(tool, "serialize"):
                serialised[name] = tool.serialize()  # type: ignore[call-arg]
            elif isinstance(tool, (dict, list, str, int, float, bool)) or tool is None:
                serialised[name] = tool
            else:
                serialised[name] = repr(tool)
        return serialised

    def render(self) -> Dict[str, Any]:
        """Expose the master screen in a form that the front-end can render."""
        return self.serialize()
