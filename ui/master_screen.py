"""Screen used by the game master to control the session."""

from __future__ import annotations

import asyncio
from dataclasses import dataclass, field
from typing import Any, Dict

from audio.engine import SoundEngine


@dataclass
class MasterScreen:
    """Collection of tools available to the game master.

    The master screen now exposes a :class:`~audio.engine.SoundEngine` instance
    via the :attr:`sound` attribute.  Games can use it to trigger sound effects
    during play.  The engine reads its configuration from ``config/audio.yaml``.

    Examples
    --------
    >>> screen = MasterScreen()
    >>> # Play a pre-recorded sound effect at half volume
    >>> asyncio.run(screen.sound.play("dice-roll", volume=0.5))
    """

    tools: Dict[str, Any] = field(default_factory=dict)
    sound: SoundEngine = field(default_factory=SoundEngine)

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
