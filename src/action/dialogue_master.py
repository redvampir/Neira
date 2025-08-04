"""Мастер диалогов - оживляю беседы между героями."""

from __future__ import annotations

from typing import Optional

from src.llm import BaseLLM
from .base_generator import BaseGenerator


class DialogueMaster(BaseGenerator):
    """Помогаю героям говорить живо."""

    def __init__(self, llm: Optional[BaseLLM]) -> None:
        super().__init__(llm, template="Сгенерируй диалог: {prompt}")

    def create(self, command: str, max_tokens: int = 512) -> str:
        """Создаю диалог с помощью LLM, если она доступна."""
        fallback = "💬 Модель недоступна для генерации диалога"
        return self.generate(command, fallback, max_tokens=max_tokens)
