"""Lightweight HTML preview widget used by the tests.

The real project exposes a rich browser component capable of hot reloading
files.  For the unit tests we merely keep track of the rendered HTML so that
other components can verify that previews are generated and updated correctly.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from html import escape
from pathlib import Path
from typing import Iterable, Set


@dataclass
class IframeView:
    """Render a HTML file inside an ``<iframe>`` snippet.

    Parameters
    ----------
    path:
        Path to the HTML document that should be displayed.
    auto_refresh:
        When ``True`` the view automatically reloads the file whenever
        :meth:`save` is called.  This mirrors the "refresh on save" behaviour of
        the desktop application.
    hotkeys:
        Iterable of accepted hotkey strings that can trigger :meth:`open`.
    """

    path: Path
    auto_refresh: bool = True
    hotkeys: Iterable[str] = field(
        default_factory=lambda: {"ctrl_shift_p", "ctrl_shift_o", "open_preview"}
    )

    _content: str = field(init=False, default="")
    _last_mtime: float = field(init=False, default=0.0)

    # ------------------------------------------------------------------
    def open(self, trigger: str | None = None) -> str:
        """Return iframe HTML when the correct hotkey is used.

        If ``trigger`` is provided it must match one of :attr:`hotkeys`,
        otherwise an empty string is returned.  This mirrors how the real GUI
        only opens the preview when the dedicated hotkey is pressed.
        """

        if trigger is not None:
            valid: Set[str] = set(self.hotkeys)
            if trigger not in valid:
                return ""
        self.refresh()
        return self.render()

    # ------------------------------------------------------------------
    def refresh(self) -> None:
        """Reload the file content if it changed on disk."""

        if self.path.exists():
            mtime = self.path.stat().st_mtime
            if mtime != self._last_mtime:
                self._content = self.path.read_text(encoding="utf-8")
                self._last_mtime = mtime

    # ------------------------------------------------------------------
    def save(self, html: str) -> None:
        """Write ``html`` to :attr:`path` and refresh the view if enabled."""

        self.path.write_text(html, encoding="utf-8")
        if self.auto_refresh:
            # Update cached content immediately to ensure consumers see the
            # changes even when the filesystem timestamp resolution is coarse.
            self._content = html
            try:
                self._last_mtime = self.path.stat().st_mtime
            except FileNotFoundError:  # pragma: no cover - extremely unlikely
                self._last_mtime = 0.0

    # ------------------------------------------------------------------
    def render(self) -> str:
        """Return an ``<iframe>`` snippet for the current content."""

        return f'<iframe srcdoc="{escape(self._content)}"></iframe>'
