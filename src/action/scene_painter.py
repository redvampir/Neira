"""Художник сцен - рисую картины словами."""

from __future__ import annotations

from typing import Optional

from src.llm import BaseLLM
from src.models import Scene
from src.memory.style_memory import StylePattern
from .base_generator import BaseGenerator


class ScenePainter(BaseGenerator):
    """Создаю яркие описания сцен."""

    def __init__(self, llm: Optional[BaseLLM]) -> None:
        super().__init__(llm, template="Опиши сцену: {prompt}")

    def paint(
        self,
        description: str,
        max_tokens: int = 512,
        style: StylePattern | None = None,
    ) -> Scene:
        """Генерирую сцену через LLM."""
        fallback = "🎨 Модель недоступна для генерации сцены"
        content = self.generate(description, fallback, max_tokens=max_tokens, style=style)
        return Scene(description=description, content=content)
