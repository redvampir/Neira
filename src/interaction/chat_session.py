from __future__ import annotations

"""Lightweight chat interface with context memory.

This module introduces :class:`ChatSession` which provides a very small
stateful layer on top of the existing tag based command system.  It allows
users to talk with Neyra in a conversational manner.  The session keeps track
of the last referenced character and automatically expands follow‑up questions
into proper tags.  The classic tag logic remains untouched – the chat simply
adds a convenience layer around it.
"""

from dataclasses import dataclass
import re
from typing import List, Optional

from .command_handler import handle_command
from .tag_processor import TagProcessor


@dataclass
class ChatEntry:
    """Single message in the conversation history."""

    speaker: str
    text: str


class ChatSession:
    """Interact with Neyra in a dialogue style.

    Parameters
    ----------
    neyra:
        Instance capable of processing commands via ``process_command``.
    processor:
        Optional shared :class:`TagProcessor`.  If not supplied, a new instance
        is created.  Keeping a single processor allows autocompletion data and
        history to be reused between calls.
    """

    def __init__(self, neyra, processor: Optional[TagProcessor] = None) -> None:
        self.neyra = neyra
        self.processor = processor or TagProcessor()
        self.history: List[ChatEntry] = []
        self._last_character: Optional[str] = None

    # ------------------------------------------------------------------
    # Public API
    def ask(self, message: str) -> str:
        """Send ``message`` to Neyra and return her response."""

        prepared = self._prepare_message(message)
        result = handle_command(self.neyra, prepared, self.processor)

        # Track conversation history
        self.history.append(ChatEntry("user", message))
        self.history.append(ChatEntry("neyra", result.text))

        # Update context based on tags in the prepared message
        tags = self.processor.parse(prepared)
        for tag in tags:
            if tag.type == "character_work" and tag.subject:
                self._last_character = tag.subject

        return result.text

    def chat_loop(self, *, input_func=input, output_func=print) -> None:  # pragma: no cover - interactive
        """Run an interactive loop until the user enters ``/exit``."""

        while True:
            user_text = input_func("\n> ")
            if not user_text.strip():
                continue
            if user_text.strip() == "/exit":
                break
            response = self.ask(user_text)
            if response:
                output_func(response)

    # ------------------------------------------------------------------
    # Helpers
    def _prepare_message(self, message: str) -> str:
        """Return a command string suitable for ``handle_command``.

        The function first checks whether the message already contains tags.  If
        it does, it is returned as‑is.  Otherwise a few simple heuristics are
        applied to map natural language questions to tags.  Currently supported
        patterns are intentionally small but showcase how context can be
        extended without modifying the underlying tag system.
        """

        # If message already uses tags – nothing to do
        if self.processor.parse(message):
            return message

        # Pattern: "как выглядел <имя>" or "расскажи как выглядит <имя>"
        m = re.search(r"(?:как\s+)?выглядел[аи]?\s+(?P<name>[\w-]+)", message, re.IGNORECASE)
        if m:
            name = m.group("name")
            self._last_character = name
            return f"@Персонаж: {name} - внешность@"

        # Follow‑up question about the last character: "как он говорит" etc.
        if self._last_character is not None:
            if re.search(r"как\s+он\s+говорит", message, re.IGNORECASE):
                return f"@Персонаж: {self._last_character} - манера речи@"
            if re.search(r"как\s+он\s+выгляд", message, re.IGNORECASE):
                return f"@Персонаж: {self._last_character} - внешность@"

        # Default: treat as a direct command to Neyra
        return f"@Нейра: {message}@"


__all__ = ["ChatSession", "ChatEntry"]
