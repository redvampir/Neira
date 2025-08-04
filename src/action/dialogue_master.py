"""Мастер диалогов - оживляю беседы между героями."""

from __future__ import annotations

from typing import Optional

from src.llm.mistral_interface import MistralLLM


class DialogueMaster:
    """Помогаю героям говорить живо."""

    def __init__(self, llm: Optional[MistralLLM]) -> None:
        self.llm = llm

    def create(self, command: str, max_tokens: int = 512) -> str:
        """Создаю диалог с помощью LLM, если она доступна."""
        if self.llm is None:
            return "💬 Модель недоступна для генерации диалога"
        prompt = f"Сгенерируй диалог: {command}"
        return self.llm.generate(prompt, max_tokens=max_tokens)
