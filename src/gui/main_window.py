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
from ..core.neyra_config import NEYRA_GREETING, NeyraPersonality


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
        if ctk is None:
            return

        # Configure grid weights for responsive resizing
        self.root.grid_rowconfigure(0, weight=1)
        self.root.grid_rowconfigure(1, weight=1)
        self.root.grid_columnconfigure(0, weight=1)
        self.root.grid_columnconfigure(1, weight=2)
        self.root.grid_columnconfigure(2, weight=1)

        # Create frames for each major component
        self.book_manager_frame = ctk.CTkFrame(self.root)
        self.book_manager_frame.grid(row=0, column=0, sticky="nsew")

        self.text_editor_frame = ctk.CTkFrame(self.root)
        self.text_editor_frame.grid(row=0, column=1, sticky="nsew")

        self.memory_viewer_frame = ctk.CTkFrame(self.root)
        self.memory_viewer_frame.grid(row=0, column=2, sticky="nsew")

        self.neyra_chat_frame = ctk.CTkFrame(self.root)
        self.neyra_chat_frame.grid(row=1, column=0, columnspan=2, sticky="nsew")

        self.tag_assistant_frame = ctk.CTkFrame(self.root)
        self.tag_assistant_frame.grid(row=1, column=2, sticky="nsew")

    def setup_components(self) -> None:
        """Instantiate child widgets."""
        self.book_manager = BookManager(self)
        self.text_editor = NeyraTextEditor(self)
        self.neyra_chat = NeyraChatPanel(self)
        self.memory_viewer = MemoryViewer(self)
        self.tag_assistant = TagAssistant(self)

    def setup_neyra_personality(self) -> None:
        """Load personality configuration for Neyra."""
        self.personality = NeyraPersonality()
        self.greeting_text = NEYRA_GREETING

        # Bind greeting text to a GUI widget if available
        if ctk is not None:
            self.greeting_label = ctk.CTkLabel(
                self.neyra_chat_frame, text=self.greeting_text
            )
            self.greeting_label.pack(pady=5)

        # Provide personality to chat component
        self.neyra_chat.personality = self.personality

    # In a real application ``run`` would likely start ``ctk``'s main loop.
    def run(self) -> None:  # pragma: no cover - GUI loop
        if ctk is not None:
            self.root.mainloop()
