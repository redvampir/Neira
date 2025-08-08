"""Shared base class for simple text generation helpers."""
from __future__ import annotations

from typing import Optional

from src.search import Retriever

from src.llm import BaseLLM
from src.memory.style_memory import StylePattern


class BaseGenerator:
    """Utility class that wraps an optional LLM with a prompt template.

    Subclasses provide a ``template`` that is used to format incoming prompts.
    The :meth:`generate` method then handles calling the LLM or returning a
    fallback when the LLM is not available.
    """

    def __init__(
        self,
        llm: Optional[BaseLLM],
        template: str,
        retriever: Retriever | None = None,
    ) -> None:
        self.llm = llm
        self.template = template
        self.retriever = retriever

    # ------------------------------------------------------------------
    def retrieve_context(self, query: str) -> str:
        """Return additional context for ``query`` using the retriever."""

        if self.retriever is None:
            return ""
        try:
            results = self.retriever.retrieve(query)
        except Exception:
            return ""
        return "\n".join(results)

    def generate(
        self,
        prompt: str,
        fallback_text: str,
        max_tokens: int = 512,
        style: StylePattern | None = None,
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
        style:
            Optional :class:`StylePattern` influencing the prompt.
        """
        if self.llm is None:
            return fallback_text
        context = self.retrieve_context(prompt)
        try:
            formatted_prompt = self.template.format(prompt=prompt, context=context)
        except KeyError:
            formatted_prompt = self.template.format(prompt=prompt)
            if context:
                formatted_prompt = context + "\n" + formatted_prompt
        if style is not None:
            style_prompt = ""
            if style.description:
                style_prompt += f"Тон: {style.description}\n"
            if style.examples:
                style_prompt += "Примеры:\n" + "\n".join(style.examples) + "\n"
            formatted_prompt = style_prompt + formatted_prompt
        return self.llm.generate(formatted_prompt, max_tokens=max_tokens)
