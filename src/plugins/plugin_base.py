from __future__ import annotations

"""Base class for simple hook-based plugins."""

from typing import Any, List


class Plugin:
    """Minimal plugin interface used by :class:`PluginManager`.

    Subclasses can override any of the hook methods to participate in
    the different stages of :class:`IterativeGenerator`.
    """

    def on_draft(self, draft: str, context: Any) -> None:  # pragma: no cover - default noop
        """Called after the initial draft has been generated."""

    def on_gap_analysis(self, draft: str, gaps: List[Any]) -> None:  # pragma: no cover
        """Called after gaps have been analysed for ``draft``."""

    def on_finalize(self, response: str) -> None:  # pragma: no cover
        """Called with the final response before it is returned."""
