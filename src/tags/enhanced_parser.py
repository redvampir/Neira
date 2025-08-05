"""Enhanced tag parser with support for inline and block patterns."""
from __future__ import annotations

from dataclasses import dataclass, field
import re
from typing import Dict, List

from src.core.neyra_config import TagSystemConfig
from src.tags.manager import register_pattern


@dataclass
class Tag:
    """A single parsed tag command."""

    type: str
    content: str
    position: tuple
    priority: int = 1
    params: Dict[str, str] = field(default_factory=dict)


class EnhancedTagParser:
    """Parse user text for inline ``@tag: value@`` and block tags."""

    #: inline patterns available by default (``@Тег: значение@``)
    INLINE_PATTERNS: Dict[str, str] = {
        **TagSystemConfig.CORE_TAGS,
        **TagSystemConfig.EXTENDED_TAGS,
        # Additional inline commands
        "character_reminder": r"@Напомни:\s*(?P<content>[^@]+)@",
        # ``generate_content`` also exposes the topic via params
        "generate_content": r"@Сгенерируй:\s*(?P<topic>[^@]+)@",
    }

    #: block patterns like ``[Пример стиля автора, X]\n...\n[Пример окончен]``
    BLOCK_PATTERNS: Dict[str, str] = {
        "style_example": r"\[Пример стиля автора,\s*(?P<author>[^\]]+)\]\s*(?P<content>.*?)\s*\[Пример окончен\]",
        "world_rule": r"\[Правило мира,\s*(?P<name>[^\]]+)\]\s*(?P<content>.*?)\s*\[Правило окончено\]",
        "character_reminder": r"\[Напоминание персонажа,\s*(?P<name>[^\]]+)\]\s*(?P<content>.*?)\s*\[Напоминание окончено\]",
        "generate_content": r"\[Сгенерируй,\s*(?P<topic>[^\]]+)\]\s*(?P<content>.*?)\s*\[Генерация окончена\]",
    }

    def __init__(self) -> None:
        for tag_dict in (self.INLINE_PATTERNS, self.BLOCK_PATTERNS):
            for tag, pattern in tag_dict.items():
                register_pattern(tag, pattern)

    def parse_user_input(self, text: str) -> List[Tag]:
        tags: List[Tag] = []

        for tag_type, pattern in self.INLINE_PATTERNS.items():
            for match in re.finditer(pattern, text, re.IGNORECASE | re.DOTALL):
                groups = match.groupdict()
                content = groups.pop("content", match.group(1)).strip()
                params = {k: v.strip() for k, v in groups.items()}
                tags.append(
                    Tag(
                        type=tag_type,
                        content=content,
                        position=match.span(),
                        params=params,
                    )
                )

        for tag_type, pattern in self.BLOCK_PATTERNS.items():
            block_re = re.compile(pattern, re.IGNORECASE | re.DOTALL)
            for match in block_re.finditer(text):
                groups = match.groupdict()
                content = groups.pop("content", match.group(1) if match.groups() else "").strip()
                params = {k: v.strip() for k, v in groups.items()}
                tags.append(
                    Tag(
                        type=tag_type,
                        content=content,
                        position=match.span(),
                        params=params,
                    )
                )

        return sorted(tags, key=lambda t: t.position[0])

    def suggest_tags(self, context: str) -> List[str]:
        suggestions: List[str] = []
        if any(word.istitle() for word in context.split()):
            suggestions.append("@Персонаж: имя - действие@")
        emotion_words = ["грустно", "радостно", "страшно", "весело"]
        if any(word in context.lower() for word in emotion_words):
            suggestions.append("@Эмоция: чувство@")
        return suggestions
