"""Lightweight facade for code editor components.

The module attempts to import optional subcomponents but tolerates missing
dependencies.  This allows unit tests to import ``code_editor`` without pulling
in heavy requirements such as ``PyYAML`` which some panels depend on.
"""

from __future__ import annotations

try:  # pragma: no cover - optional dependency
    from .lsp_client import LSPClient
except Exception:  # pragma: no cover - gracefully degrade when deps missing
    LSPClient = None  # type: ignore[assignment]

try:  # pragma: no cover - optional dependency
    from .profiler_panel import ProfilerPanel
except Exception:  # pragma: no cover - gracefully degrade when deps missing
    ProfilerPanel = None  # type: ignore[assignment]

from .translation_panel import TranslationPanel

try:  # pragma: no cover - optional dependency
    from .git_ui import GitUI, ConflictWindow
except Exception:  # pragma: no cover - gracefully degrade when deps missing
    GitUI = None  # type: ignore[assignment]
    ConflictWindow = None  # type: ignore[assignment]

__all__ = [
    "LSPClient",
    "ProfilerPanel",
    "TranslationPanel",
    "GitUI",
    "ConflictWindow",
]
