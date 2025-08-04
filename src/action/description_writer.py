"""Генератор описаний через локальную LLM."""
from __future__ import annotations

from typing import Optional

from src.llm import BaseLLM
from .base_generator import BaseGenerator


class DescriptionWriter(BaseGenerator):
    """Создает повествовательные описания с помощью LLM."""

    def __init__(self, llm: Optional[BaseLLM]) -> None:
        super().__init__(llm, template="Опиши: {prompt}")

    def write(self, description: str, max_tokens: int = 512) -> str:
        """Сгенерировать описание для указанной идеи."""
        fallback = f"📜 Описание: {description}"
        return self.generate(description, fallback, max_tokens=max_tokens)
