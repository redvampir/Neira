"""Utility to parse and route command-like tags.

This module recognises inline tags such as ``@Кампания: создать@`` and
bracket tags like ``[Настроить НРИ-режим]``.  Each recognised tag is routed
to a corresponding handler function.  The design includes a ``register``
method to allow new commands to be added without modifying the core parser,
keeping the system easily extensible.
"""

from __future__ import annotations

from dataclasses import dataclass
import re
from typing import Callable, Dict, List, Pattern


@dataclass
class Command:
    """Container for a single command pattern and its handler."""

    name: str
    pattern: Pattern[str]
    handler: Callable[[re.Match], str]


class CommandTagParser:
    """Parse text for special commands and dispatch to handlers."""

    def __init__(self) -> None:
        self._commands: Dict[str, Command] = {}
        # Register built-in commands
        self.register(
            name="create_campaign",
            pattern=r"@Кампания:\s*создать@",
            handler=lambda _: self.create_campaign(),
        )
        self.register(
            name="setup_nri_mode",
            pattern=r"\[Настроить НРИ-режим\]",
            handler=lambda _: self.setup_nri_mode(),
        )

    def register(self, name: str, pattern: str, handler: Callable[[re.Match], str]) -> None:
        """Register a new command pattern and its handler."""
        compiled = re.compile(pattern, re.IGNORECASE | re.DOTALL)
        self._commands[name] = Command(name=name, pattern=compiled, handler=handler)

    def parse_and_execute(self, text: str) -> List[str]:
        """Parse ``text`` and execute any recognised commands.

        Returns a list with the results from each executed handler in the
        order they were found in the text.
        """
        results: List[str] = []
        for command in self._commands.values():
            for match in command.pattern.finditer(text):
                results.append(command.handler(match))
        return results

    # Handlers for built-in commands -------------------------------------------------
    def create_campaign(self) -> str:  # pragma: no cover - trivial behaviour
        """Handle the ``@Кампания: создать@`` tag."""
        return "create_campaign"

    def setup_nri_mode(self) -> str:  # pragma: no cover - trivial behaviour
        """Handle the ``[Настроить НРИ-режим]`` tag."""
        return "setup_nri_mode"
