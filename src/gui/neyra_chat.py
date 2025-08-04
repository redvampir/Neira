"""Chat panel used to communicate with the Neyra assistant."""

from __future__ import annotations

from typing import Any


class NeyraChatPanel:  # pragma: no cover - GUI stub
    """Simple placeholder for the chat interface."""

    def __init__(self, parent: Any | None = None) -> None:
        self.parent = parent
        self.setup_chat_area()
        self.setup_input_area()
        self.setup_neyra_avatar()
        self.load_personality()

    def setup_chat_area(self) -> None:
        pass

    def setup_input_area(self) -> None:
        pass

    def setup_neyra_avatar(self) -> None:
        pass

    def load_personality(self) -> None:
        pass

    def send_message_to_neyra(self, message: str) -> None:
        pass

    def receive_neyra_response(self, response: str) -> None:
        pass

    def show_neyra_thinking(self) -> None:
        pass
