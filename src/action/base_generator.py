"""Shared base class for simple text generation helpers."""
from __future__ import annotations

from typing import Optional

from src.llm.mistral_interface import MistralLLM


class BaseGenerator:
    """Utility class that wraps an optional LLM with a prompt template.

    Subclasses provide a ``template`` that is used to format incoming prompts.
    The :meth:`generate` method then handles calling the LLM or returning a
    fallback when the LLM is not available.
    """

    def __init__(self, llm: Optional[MistralLLM], template: str) -> None:
        self.llm = llm
        self.template = template

    def generate(
        self, prompt: str, fallback_text: str, max_tokens: int = 512
    ) -> str:
        """Generate text using the LLM if available.

        Parameters
        ----------
        prompt:
            The core idea or command to feed into the template.
        fallback_text:
            Text returned when ``llm`` is ``None``.
        max_tokens:
            Maximum amount of tokens for the generation.
        """
        if self.llm is None:
            return fallback_text
        formatted_prompt = self.template.format(prompt=prompt)
        return self.llm.generate(formatted_prompt, max_tokens=max_tokens)
