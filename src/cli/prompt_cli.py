"""Interactive command line interface for Neyra using prompt_toolkit.

Provides tag and command autocompletion and handles basic slash
commands like ``/help`` and ``/exit``.  Suggestions are accepted with
``Tab``.
"""

from __future__ import annotations

import re
from typing import Iterable

from prompt_toolkit import PromptSession
from prompt_toolkit.completion import Completer, Completion
from prompt_toolkit.key_binding import KeyBindings
from rich.console import Console
from rich.panel import Panel

from src.interaction import TagProcessor, handle_command


class _NeyraCompleter(Completer):
    """Autocomplete tags, commands and entity names within tags."""

    def __init__(self, tags: Iterable[str], commands: Iterable[str], processor: TagProcessor) -> None:
        self._tags = list(tags)
        self._commands = list(commands)
        self._processor = processor

    def get_completions(self, document, complete_event):  # type: ignore[override]
        text = document.text_before_cursor

        # Detect if the cursor is inside a tag after a colon.  In this case we
        # provide entity name suggestions and automatically close the tag when a
        # completion is accepted.
        tag_match = re.search(r"@[^@:\s]+:\s*([^@]*)$", text)
        if tag_match:
            prefix = tag_match.group(1)
            for suggestion in self._processor.generate_hints(prefix):
                yield Completion(f"{suggestion}@ ", start_position=-len(prefix))
            return

        word = document.get_word_before_cursor(pattern=re.compile(r"[^\s]+"))
        if not word:
            return
        if word.startswith("@"):  # Tag completion
            prefix = word[1:].lower()
            for tag in self._tags:
                if tag.lower().startswith(prefix):
                    yield Completion(f"@{tag}: ", start_position=-len(word))
        elif word.startswith("/"):  # Slash command completion
            prefix = word[1:].lower()
            for cmd in self._commands:
                if cmd.lower().startswith(prefix):
                    yield Completion(f"/{cmd}", start_position=-len(word))


COMMANDS = TagProcessor.SLASH_COMMANDS


def run_cli(neyra, *, use_color: bool = True) -> None:
    """Run interactive CLI loop for the given :class:`Neyra` instance."""

    processor = TagProcessor()
    completer = _NeyraCompleter(TagProcessor.available_tags(), COMMANDS, processor)

    kb = KeyBindings()

    @kb.add("tab")
    def _(event) -> None:
        buffer = event.current_buffer
        if buffer.complete_state:
            buffer.complete_next()
        else:
            buffer.start_completion(select_first=False)

    session = PromptSession(completer=completer, key_bindings=kb)
    console = Console(no_color=not use_color)

    print("Введите команды. Используйте /help для помощи, /exit для выхода.")
    while True:
        try:
            text = session.prompt("> ")
            if hasattr(neyra, "history"):
                neyra.history.add(text)
        except (KeyboardInterrupt, EOFError):  # pragma: no cover - interactive
            print()
            break
        clean = text.strip()
        if not clean:
            continue
        result = handle_command(neyra, text, processor)
        if result.is_exit:
            break
        if not result.text and not result.waves:
            continue

        output_text = result.text
        if result.waves:
            console.print("Доступные волны:")
            for idx, wave in enumerate(result.waves, 1):
                console.print(f"{idx}. {wave}")
            try:
                choice = int(session.prompt("Выберите волну (1..n): ")) - 1
            except Exception:  # pragma: no cover - user input
                choice = 0
            if 0 <= choice < len(result.waves):
                output_text = result.waves[choice]
            else:  # pragma: no cover - defensive
                output_text = result.waves[0]

        if result.style:
            console.print(Panel(output_text, style=result.style))
        else:
            console.print(output_text)


__all__ = ["run_cli"]
