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

from prompt_toolkit import PromptSession
from prompt_toolkit.history import InMemoryHistory
from rich.console import Console
from rich.markdown import Markdown
from rich.panel import Panel

from .tag_processor import TagProcessor, handle_command


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

    def __init__(
        self,
        neyra,
        processor: Optional[TagProcessor] = None,
        *,
        max_history: int = 50,
        console: Optional[Console] = None,
    ) -> None:
        self.neyra = neyra
        self.processor = processor or TagProcessor()
        self.history: List[ChatEntry] = []
        self._last_character: Optional[str] = None
        self.max_history = max_history
        self.console = console or Console()

    # ------------------------------------------------------------------
    # Public API
    def ask(self, message: str) -> str:
        """Send ``message`` to Neyra and return her response."""

        try:
            prepared = self._prepare_message(message)
            result = handle_command(self.neyra, prepared, self.processor)
        except Exception as exc:  # pragma: no cover - defensive
            self.console.print(Panel(f"[red]{exc}[/]", title="Error"))
            return ""

        # Track conversation history
        self.history.append(ChatEntry("user", message))
        self.history.append(ChatEntry("neyra", result.text))
        self._trim_history()

        # Update context based on tags in the prepared message
        tags = self.processor.parse(prepared)
        for tag in tags:
            if tag.type == "character_work" and tag.subject:
                self._last_character = tag.subject

        return result.text

    def chat_loop(self) -> None:  # pragma: no cover - interactive
        """Run an interactive loop until the user enters ``/exit``."""

        session = PromptSession(history=InMemoryHistory())

        while True:
            try:
                user_text = session.prompt("\n> ")
            except (KeyboardInterrupt, EOFError):
                break
            if not user_text.strip():
                continue
            if user_text.strip() == "/exit":
                break

            if user_text.startswith("/"):
                service = self._handle_service_command(user_text.strip())
                if service:
                    self.console.print(Panel(Markdown(service), title="System"))
                continue

            response = self.ask(user_text)
            if response:
                self.console.print(Panel(Markdown(response), title="Neyra"))

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

    def _trim_history(self) -> None:
        """Ensure stored history does not exceed ``max_history`` entries."""

        if len(self.history) > self.max_history:
            del self.history[: len(self.history) - self.max_history]

    def _handle_service_command(self, command: str) -> Optional[str]:
        """Handle internal service commands.

        Parameters
        ----------
        command:
            The command string beginning with ``/``.
        Returns
        -------
        Optional[str]
            Textual response for the user. ``None`` if nothing should be printed.
        """

        cmd = command.strip()
        if cmd == "/help":
            return (
                "Доступные команды:\n"
                "/help — показать это сообщение\n"
                "/clear — очистить историю\n"
                "/status — состояние сессии\n"
                "/memory — вывести историю"
            )
        if cmd == "/clear":
            self.history.clear()
            return "История очищена"
        if cmd == "/status":
            status = [f"Записей в истории: {len(self.history)}"]
            if self._last_character:
                status.append(f"Последний персонаж: {self._last_character}")
            return "\n".join(status)
        if cmd == "/memory":
            if not self.history:
                return "История пуста"
            lines = [f"{entry.speaker}: {entry.text}" for entry in self.history]
            return "\n".join(lines)
        return f"Неизвестная команда: {cmd}"


__all__ = ["ChatSession", "ChatEntry"]
