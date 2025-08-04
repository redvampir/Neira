"""Художник сцен - рисую картины словами."""

from __future__ import annotations

from typing import Optional

from src.llm.mistral_interface import MistralLLM


class ScenePainter:
    """Создаю яркие описания сцен."""

    def __init__(self, llm: Optional[MistralLLM]) -> None:
        self.llm = llm

    def paint(self, description: str, max_tokens: int = 512) -> str:
        """Генерирую сцену через LLM."""
        if self.llm is None:
            return "🎨 Модель недоступна для генерации сцены"
        prompt = f"Опиши сцену: {description}"
        return self.llm.generate(prompt, max_tokens=max_tokens)
