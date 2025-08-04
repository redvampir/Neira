"""Mistral LLM interface using llama-cpp-python."""
from __future__ import annotations

from llama_cpp import Llama


class MistralLLM:
    """Wrapper around a local Mistral GGUF model."""

    def __init__(self, model_path: str) -> None:
        self.model = Llama(
            model_path=model_path,
            n_ctx=2048,
            n_gpu_layers=32,  # Можно уменьшить до 20–24 при нехватке памяти
            n_threads=6,
            use_mlock=True,
            verbose=False,
        )

    def generate(self, prompt: str, max_tokens: int = 512) -> str:
        """Generate text from the given prompt."""
        result = self.model(prompt, max_tokens=max_tokens, stop=["</s>"])
        return result["choices"][0]["text"].strip()
