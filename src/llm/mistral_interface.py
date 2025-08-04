"""Mistral LLM interface using llama-cpp-python."""
from __future__ import annotations

from typing import Optional

from llama_cpp import Llama


class MistralLLM:
    """Wrapper around a local Mistral GGUF model."""

    def __init__(self, model_path: str, n_ctx: int = 2048, n_threads: int = 6) -> None:
        self.model = Llama(
            model_path=model_path,
            n_ctx=n_ctx,
            n_threads=n_threads,
            use_mlock=True,
            verbose=False,
        )

    def generate(self, prompt: str, max_tokens: int = 512) -> str:
        """Generate text from the given prompt."""
        response = self.model(prompt, max_tokens=max_tokens)
        return response["choices"][0]["text"].strip()
