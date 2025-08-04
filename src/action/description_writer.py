"""Генератор описаний через локальную LLM."""
from __future__ import annotations

from typing import Optional

from src.llm.mistral_interface import MistralLLM


class DescriptionWriter:
    """Создает повествовательные описания с помощью LLM."""

    def __init__(self, llm: Optional[MistralLLM]) -> None:
        self.llm = llm

    def write(self, description: str, max_tokens: int = 512) -> str:
        """Сгенерировать описание для указанной идеи."""
        if self.llm is None:
            return f"📜 Описание: {description}"
        prompt = f"Опиши: {description}"
        return self.llm.generate(prompt, max_tokens=max_tokens)
