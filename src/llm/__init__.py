"""LLM interfaces for Neyra."""

from .base_llm import BaseLLM, LLMFactory
from .mistral_interface import MistralLLM
from .qwen_coder_interface import QwenCoderLLM

# ``LLMManager`` and the prompt helpers pull in a large dependency graph.
# They are optional for most unit tests, so import them lazily and fall back to
# stubs when dependencies are missing.  This keeps importing :mod:`src.llm`
# lightweight which is important for the test environment.
try:  # pragma: no cover - optional imports
    from .manager import LLMManager, ModelSpec, Task
    from .prompts import chat_prompt, apply_user_style
except Exception:  # pragma: no cover - any import issue
    LLMManager = ModelSpec = Task = None  # type: ignore

    def chat_prompt(*args, **kwargs):  # type: ignore
        raise RuntimeError("prompts module unavailable")

    def apply_user_style(*args, **kwargs):  # type: ignore
        raise RuntimeError("prompts module unavailable")

__all__ = [
    "BaseLLM",
    "LLMFactory",
    "MistralLLM",
    "QwenCoderLLM",
    "LLMManager",
    "ModelSpec",
    "Task",
    "chat_prompt",
    "apply_user_style",
]
