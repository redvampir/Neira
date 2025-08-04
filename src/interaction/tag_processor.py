from __future__ import annotations

"""Lightweight tag parsing and command handling utilities."""

from dataclasses import dataclass
import re
from typing import List, Optional


# ---------------------------------------------------------------------------
# Tag parsing


@dataclass
class ProcessedTag:
    type: str
    subject: str
    commands: List[str]


class TagProcessor:
    """Very small helper for working with ``@тег: значение@`` конструкциями."""

    SLASH_COMMANDS = ["help", "exit", "сгенерировать"]

    @staticmethod
    def available_tags() -> List[str]:  # pragma: no cover - simple constant
        return ["Нейра", "Персонаж", "Сцена", "Эмоция", "Стиль", "Место"]

    # Basic entity storage for hint generation
    def __init__(self) -> None:
        self._entities: List[str] = []

    # Parsing ------------------------------------------------------------
    def parse(self, text: str) -> List[ProcessedTag]:
        pattern = re.compile(r"@([^:@]+):([^@]+)@")
        tags: List[ProcessedTag] = []
        for m in pattern.finditer(text):
            tag_type = m.group(1).strip().lower()
            content = m.group(2).strip()
            parts = [p.strip() for p in content.split('/')] if '/' in content else [content]
            first = parts[0]
            commands: List[str] = []
            subject = first
            if '-' in first:
                subject, cmd = [p.strip() for p in first.split('-', 1)]
                if cmd:
                    commands.append(cmd)
            for p in parts[1:]:
                if p:
                    commands.append(p)
            tags.append(ProcessedTag(tag_type, subject, commands))
            self.register_entity(subject)
        return tags

    # Hint helpers -------------------------------------------------------
    def register_entity(self, name: str) -> None:
        if name and name not in self._entities:
            self._entities.append(name)

    def suggest_entities(self, prefix: str) -> List[str]:
        return [e for e in self._entities if e.lower().startswith(prefix.lower())]

    def generate_hints(self, prefix: str) -> List[str]:
        return self.suggest_entities(prefix)

    # Slash command execution -------------------------------------------
    def execute_slash(self, command: str) -> Optional[str]:
        clean = command.strip()
        if clean.startswith('/'):
            clean = clean[1:]
        name, _, arg = clean.partition(' ')
        name = name.lower()
        if name == 'help':
            tags = ', '.join(self.available_tags())
            cmds = ', '.join(f"/{c}" for c in self.SLASH_COMMANDS)
            return f"Доступные теги: {tags}\nДоступные команды: {cmds}"
        if name == 'exit':
            return "__exit__"
        if name == 'сгенерировать':
            return f"Сгенерируй сцену: {arg}" if arg else ""
        return None

    def execute(self, tag: ProcessedTag) -> Optional[str]:
        if "сгенерировать" in [c.lower() for c in tag.commands]:
            return f"Сгенерируй сцену: {tag.subject}"
        return None


# ---------------------------------------------------------------------------
# Command handling

from dataclasses import dataclass


@dataclass
class CommandResult:
    text: str = ""
    style: Optional[str] = None
    is_exit: bool = False


def handle_command(neyra, text: str, processor: TagProcessor) -> CommandResult:
    """Process a single user command."""

    clean = text.strip()
    if not clean:
        return CommandResult()
    if clean.startswith('/'):
        result = processor.execute_slash(clean)
        if result == "__exit__":
            return CommandResult(is_exit=True)
        return CommandResult(text=result or "", style="cyan")
    result = neyra.process_command(text)
    lower = result.lower()
    style = None
    if "@" in result:
        style = "cyan"
    elif "эмоци" in lower:
        style = "magenta"
    elif any(word in lower for word in ["опис", "сцена"]):
        style = "green"
    return CommandResult(text=result, style=style)


__all__ = ["TagProcessor", "ProcessedTag", "handle_command", "CommandResult"]
