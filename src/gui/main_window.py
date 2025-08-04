"""Main application window for the desktop interface."""

from __future__ import annotations

try:  # pragma: no-cover - GUI libraries might be missing in test env
    import customtkinter as ctk
except Exception:  # pragma: no-cover
    ctk = None  # type: ignore

from .book_manager import BookManager
from .text_editor import NeyraTextEditor
from .neyra_chat import NeyraChatPanel
from .memory_viewer import MemoryViewer
from .tag_assistant import TagAssistant


class NeyraMainWindow:  # pragma: no cover - mostly GUI wiring
    """Main window tying together all major components."""

    def __init__(self) -> None:
        self.setup_window()
        self.setup_layout()
        self.setup_components()
        self.setup_neyra_personality()

    def setup_window(self) -> None:
        """Configure window appearance."""
        # Detailed implementation will be added in later iterations.
        if ctk is not None:
            self.root = ctk.CTk()
            self.root.title("Neyra")

    def setup_layout(self) -> None:
        """Create main layout containers."""
        # Placeholder for layout logic.
        pass

    def setup_components(self) -> None:
        """Instantiate child widgets."""
        self.book_manager = BookManager(self)
        self.text_editor = NeyraTextEditor(self)
        self.neyra_chat = NeyraChatPanel(self)
        self.memory_viewer = MemoryViewer(self)
        self.tag_assistant = TagAssistant(self)

    def setup_neyra_personality(self) -> None:
        """Load personality configuration for Neyra."""
        pass

    # In a real application ``run`` would likely start ``ctk``'s main loop.
    def run(self) -> None:  # pragma: no cover - GUI loop
        if ctk is not None:
            self.root.mainloop()
