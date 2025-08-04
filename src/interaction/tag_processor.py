"""Advanced tag processing and command handling.

The classic tag system used by the project understands simple markers like
``@Персонаж: Имя@``.  This module extends the concept by supporting commands
inside tags, autocompletion and style extraction.  The goal is not to provide
an ultimate implementation, but a lightweight layer that demonstrates how such
features could be wired together.
"""

from __future__ import annotations

import json
import re
from collections import deque
from dataclasses import dataclass, field
from pathlib import Path
from typing import Deque, Dict, List, Optional


@dataclass
class ProcessedTag:
    """A normalised representation of a tag."""

    type: str
    subject: str
    commands: List[str] = field(default_factory=list)
    raw: str = ""


class TagProcessor:
    """Parse tags and provide helper utilities.

    Parameters
    ----------
    kb_path:
        Path to the knowledge base directory.  It is used for autocompletion
        and for resolving commands that require reading stored information.
    llm:
        Optional language model used when generation is requested.
    """

    def __init__(self, kb_path: str | Path = "data/knowledge_base", llm: Optional[object] = None) -> None:
        self.kb_path = Path(kb_path)
        self.llm = llm
        self.entity_history: Deque[str] = deque(maxlen=100)

        index_file = self.kb_path / "index.json"
        if index_file.exists():
            self.index: Dict[str, List[str]] = json.loads(index_file.read_text(encoding="utf-8"))
        else:
            self.index = {"characters": [], "locations": []}

    # ------------------------------------------------------------------
    # Parsing
    def parse(self, text: str) -> List[ProcessedTag]:
        """Extract all tags from the provided text."""

        pattern = re.compile(r"@([^:]+):\s*([^@]+)@")
        tags: List[ProcessedTag] = []
        for match in pattern.finditer(text):
            tag_type = match.group(1).strip().lower()
            content = match.group(2).strip()
            subject, commands = self._split_content(content)
            tag = ProcessedTag(type=tag_type, subject=subject, commands=commands, raw=match.group(0))
            tags.append(tag)
            self.register_entity(subject)
        return tags

    @staticmethod
    def _split_content(content: str) -> tuple[str, List[str]]:
        """Split tag content into subject and a list of commands."""

        parts = [p.strip() for p in content.split('/') if p.strip()]
        first = parts[0] if parts else ""
        rest = parts[1:]
        subject_part, _, cmd_part = first.partition('—')
        commands: List[str] = []
        if cmd_part:
            commands.append(cmd_part.strip())
        commands.extend(rest)
        return subject_part.strip(), commands

    # ------------------------------------------------------------------
    # Autocomplete helpers
    def register_entity(self, name: str) -> None:
        """Remember entity names for later suggestions."""

        if name:
            self.entity_history.append(name)

    def suggest_entities(self, prefix: str) -> List[str]:
        """Suggest known entities starting with ``prefix``."""

        prefix_low = prefix.lower()
        pool: List[str] = []
        for item in list(self.entity_history) + self.index.get("characters", []) + self.index.get("locations", []):
            if item not in pool and item.lower().startswith(prefix_low):
                pool.append(item)
        return pool

    # ------------------------------------------------------------------
    # Style example extraction
    def extract_style_examples(self, text: str) -> List[str]:
        """Extract style examples wrapped in special markers and store them."""

        pattern = re.compile(
            r"\[Пример стиля автора,.*?\](.*?)\[Пример окончен\]",
            re.DOTALL,
        )
        examples = [ex.strip() for ex in pattern.findall(text)]
        if not examples:
            return []

        style_file = self.kb_path / "style.json"
        data = {"examples": []}
        if style_file.exists():
            data = json.loads(style_file.read_text(encoding="utf-8"))
        data.setdefault("examples", []).extend(examples)
        style_file.write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8")
        return examples

    # ------------------------------------------------------------------
    # Command execution helpers
    def execute(self, tag: ProcessedTag) -> Optional[str]:
        """Execute simple built-in commands."""

        if tag.type == "персонаж" and tag.commands:
            return self._character_command(tag.subject, tag.commands[0])
        if "сгенерировать" in " ".join(tag.commands) or "сгенерировать" in tag.subject.lower():
            return self._generate_text(tag.subject)
        return None

    def _character_command(self, name: str, command: str) -> Optional[str]:
        char_file = self.kb_path / "characters.json"
        if not char_file.exists():
            return None
        data = json.loads(char_file.read_text(encoding="utf-8"))
        record = data.get(name.lower())
        if not record:
            return None
        if command == "внешность":
            return record.get("appearance") or ""
        if command == "стиль":
            return record.get("speech_style") or ""
        if command == "сцена":
            # In a full implementation this would return stored scenes.  For now
            # we simply acknowledge the request.
            return f"Сцены с участием {name} пока не сохранены"
        return None

    def _generate_text(self, description: str) -> str:
        """Generate a scene using the optional language model."""

        prompt = (
            "Сгенерируй сцену: {desc}\n"
            "Стиль: неизвестный автор\n"
            "Формат: краткие фразы, от лица рассказчика".format(desc=description)
        )
        if self.llm is None:
            # During tests we don't have a real model – return the prompt so
            # callers can verify the template.
            return prompt
        return self.llm.generate(prompt)


__all__ = ["TagProcessor", "ProcessedTag"]

