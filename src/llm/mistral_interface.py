"""Mistral LLM interface using llama-cpp-python."""
from __future__ import annotations

# The real implementation relies on ``llama_cpp`` which may not be available
# in lightweight environments (like the test environment for this kata).
# Import the class lazily and provide a helpful fallback so that the module can
# be imported even when the dependency is missing.  Tests only require the
# class to exist – they don't actually instantiate the heavy model.
try:  # pragma: no cover - simple import guard
    from llama_cpp import Llama  # type: ignore
except ModuleNotFoundError:  # pragma: no cover
    Llama = None  # type: ignore


class MistralLLM:
    """Wrapper around a local Mistral GGUF model."""

    def __init__(self, model_path: str) -> None:
        if Llama is None:
            raise RuntimeError("llama_cpp is required to use MistralLLM")
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
        if Llama is None:
            raise RuntimeError("llama_cpp is required to use MistralLLM")
        result = self.model(prompt, max_tokens=max_tokens, stop=["</s>"])
        return result["choices"][0]["text"].strip()
