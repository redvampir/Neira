"""Lightweight tag parsing and command handling utilities."""

from __future__ import annotations

import json
from dataclasses import dataclass
from pathlib import Path
import re
from typing import List, Optional

from neira_rust import (
    parse as _parse_tags,
    suggest_entities as _suggest_entities,
    Tag as ProcessedTag,
)


class TagProcessor:
    """Helper for working with ``@тег: значение@`` constructs."""

    SLASH_COMMANDS = ["help", "exit", "сгенерировать"]

    @staticmethod
    def available_tags() -> List[str]:  # pragma: no cover - simple constant
        return ["Нейра", "Персонаж", "Сцена", "Эмоция", "Стиль", "Место"]

    def parse(self, text: str) -> List[ProcessedTag]:
        return _parse_tags(text)

    def suggest_entities(self, prefix: str) -> List[str]:
        return _suggest_entities(prefix)

    def generate_hints(self, prefix: str) -> List[str]:
        return self.suggest_entities(prefix)

    # ------------------------------------------------------------------
    def extract_style_examples(self, text: str) -> List[str]:
        """Extract style examples marked by special blocks and persist them."""

        from src.memory.knowledge_base import KB_ROOT

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
        """Execute ``plan`` handling ``ACT`` steps."""

        from src.analysis import PostProcessor, run_post_processors
        from src.analysis.reasoning_planner import ReasoningStep
        from src.memory.index import MemoryIndex
        from src.search.retriever import Retriever

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

