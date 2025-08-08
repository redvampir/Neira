"""Lightweight tag parsing and command handling utilities."""

from __future__ import annotations

from dataclasses import dataclass
import json
from pathlib import Path
import re
from typing import List, Optional

from src.analysis import PostProcessor, run_post_processors
from src.analysis.reasoning_planner import ReasoningStep
from src.memory.index import MemoryIndex
from src.memory.knowledge_base import KB_ROOT
from src.search.retriever import Retriever


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
        """Parse ``text`` for ``@тег: значение@`` constructs.

        The parser supports nested tags and escaping of the ``@`` symbol via a
        double ``@@`` sequence.  Commands can be supplied using either an em-dash
        (``—``) or a regular hyphen after the subject and additional commands can
        follow separated by ``/``.
        """

        def render(tag: ProcessedTag) -> str:
            base = f"@{tag.type.capitalize()}: {tag.subject}"
            if tag.commands:
                base += " " + " ".join(f"/{c}" for c in tag.commands)
            return base + "@"

        def parse_tag(segment: str, start: int) -> tuple[ProcessedTag, List[ProcessedTag], int]:
            i = start + 1
            colon = segment.find(":", i)
            if colon == -1:
                return ProcessedTag("", "", []), [], 1
            tag_type = segment[i:colon].strip().lower()
            i = colon + 1
            content_chars: List[str] = []
            inner: List[ProcessedTag] = []
            while i < len(segment):
                ch = segment[i]
                if ch == "@":
                    if i + 1 < len(segment) and segment[i + 1] == "@":
                        content_chars.append("@")
                        i += 2
                        continue
                    next_at = segment.find("@", i + 1)
                    next_colon = segment.find(":", i + 1)
                    if next_colon != -1 and (next_at == -1 or next_colon < next_at):
                        nested_tag, nested_inner, consumed = parse_tag(segment, i)
                        inner.extend(nested_inner)
                        inner.append(nested_tag)
                        content_chars.append(render(nested_tag))
                        i += consumed
                        continue
                    i += 1
                    break
                else:
                    content_chars.append(ch)
                    i += 1
            content = "".join(content_chars).replace("@@", "@")
            parts = [p.strip() for p in content.split("/") if p.strip()]
            first = parts[0] if parts else ""
            commands: List[str] = []
            subject = first
            dash_split = "—" if "—" in first else "-" if "-" in first else None
            if dash_split:
                subject, cmd = [p.strip() for p in first.split(dash_split, 1)]
                if cmd:
                    commands.append(cmd)
            for p in parts[1:]:
                commands.append(p)
            tag = ProcessedTag(tag_type, subject, commands)
            self.register_entity(subject)
            return tag, inner, i - start

        tags: List[ProcessedTag] = []
        i = 0
        while i < len(text):
            if text[i] == "@":
                tag, inner_tags, consumed = parse_tag(text, i)
                tags.extend(inner_tags)
                if tag.type:
                    tags.append(tag)
                i += consumed
            else:
                i += 1
        return tags

    # Hint helpers -------------------------------------------------------
    def register_entity(self, name: str) -> None:
        if name and name not in self._entities:
            self._entities.append(name)

    def suggest_entities(self, prefix: str) -> List[str]:
        """Return known entities starting with ``prefix``.

        Entities are collected from previously parsed tags as well as the
        knowledge base stored in :mod:`src.memory.knowledge_base`.
        """

        suggestions = [
            e for e in self._entities if e.lower().startswith(prefix.lower())
        ]
        kb_file = KB_ROOT / "characters.json"
        try:
            data = json.loads(kb_file.read_text(encoding="utf-8"))
            for info in data.values():
                name = info.get("name", "")
                if name.lower().startswith(prefix.lower()):
                    suggestions.append(name)
        except Exception:
            pass
        seen: set[str] = set()
        result: List[str] = []
        for name in suggestions:
            if name not in seen:
                seen.add(name)
                result.append(name)
        return result

    def generate_hints(self, prefix: str) -> List[str]:
        return self.suggest_entities(prefix)

    # ------------------------------------------------------------------
    def extract_style_examples(self, text: str) -> List[str]:
        """Extract style examples marked by special blocks and persist them."""

        pattern = re.compile(
            r"\[Пример стиля автора,.*?\](.*?)\[Пример окончен\]",
            re.DOTALL,
        )
        examples = [m.strip() for m in pattern.findall(text)]
        if examples:
            KB_ROOT.mkdir(parents=True, exist_ok=True)
            path = KB_ROOT / "style.json"
            try:
                data = json.loads(path.read_text(encoding="utf-8"))
            except Exception:
                data = {"examples": []}
            for ex in examples:
                if ex not in data["examples"]:
                    data["examples"].append(ex)
            path.write_text(
                json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8"
            )
        return examples

    # ------------------------------------------------------------------
    def run_reasoning_plan(
        self,
        plan: List[ReasoningStep],
        memory: MemoryIndex | None = None,
        retriever: Retriever | None = None,
        post_processors: List[PostProcessor] | None = None,
    ) -> str:
        """Execute ``plan`` handling ``ACT`` steps.

        ``ACT`` steps with ``source='memory'`` query :class:`MemoryIndex` while
        ``source='rag'`` steps leverage :class:`src.search.retriever.Retriever`.
        After all actions are performed the aggregated text is run through the
        provided ``post_processors`` using :func:`run_post_processors`.
        """

        outputs: List[str] = []
        mem = memory or MemoryIndex()
        rag = retriever or Retriever()
        for step in plan:
            if step.marker != "ACT":
                continue
            if step.source == "memory":
                result = mem.get(step.content)
                if result is not None:
                    outputs.append(str(result))
            elif step.source == "rag":
                snippets = rag.retrieve(step.content)
                if snippets:
                    outputs.extend(snippets)
        text = "\n".join(outputs)
        processors = post_processors or []
        text, _ = run_post_processors(text, processors)
        return text

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
