"""Graphical user interface components for the desktop edition of Neira.

The real project ships a fairly feature rich GUI.  Importing all of its
submodules pulls in heavy optional dependencies (``PyYAML``, GUI toolkits,
etc.).  The tests in this kata only need access to a couple of lightweight
helpers, therefore we attempt to import components lazily and fall back to
``None`` when prerequisites are missing.  This mirrors the behaviour of
``code_editor.__init__``.
"""

from __future__ import annotations

try:  # pragma: no cover - optional dependency
    from .main_window import NeyraMainWindow
except Exception:  # pragma: no cover - allow running without GUI stack
    NeyraMainWindow = None  # type: ignore[assignment]

try:  # pragma: no cover - optional dependency
    from .book_manager import BookManager
except Exception:  # pragma: no cover
    BookManager = None  # type: ignore[assignment]

from .text_editor import NeyraTextEditor

try:  # pragma: no cover - optional dependency
    from .neyra_chat import NeyraChatPanel
except Exception:  # pragma: no cover
    NeyraChatPanel = None  # type: ignore[assignment]

try:  # pragma: no cover - optional dependency
    from .memory_viewer import MemoryViewer
except Exception:  # pragma: no cover
    MemoryViewer = None  # type: ignore[assignment]

try:  # pragma: no cover - optional dependency
    from .tag_assistant import TagAssistant
except Exception:  # pragma: no cover
    TagAssistant = None  # type: ignore[assignment]

try:  # pragma: no cover - optional dependency
    from .settings_window import SettingsWindow
except Exception:  # pragma: no cover
    SettingsWindow = None  # type: ignore[assignment]

__all__ = [
    "NeyraMainWindow",
    "BookManager",
    "NeyraTextEditor",
    "NeyraChatPanel",
    "MemoryViewer",
    "TagAssistant",
    "SettingsWindow",
]
