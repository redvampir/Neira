"""LLM interfaces for Neyra.

The real project bundles a number of optional model backends.  Importing all of
them would require a large set of third‑party dependencies which are not
available in the execution environment used for the tests.  The ``__init__``
module therefore mirrors other packages in the repository and gracefully
degrades by exposing ``None`` placeholders when imports fail.
"""

try:  # pragma: no cover - optional dependency
    from .base_llm import BaseLLM, LLMFactory
except Exception:  # pragma: no cover
    BaseLLM = LLMFactory = None  # type: ignore[assignment]

try:  # pragma: no cover - optional dependency
    from .mistral_interface import MistralLLM
except Exception:  # pragma: no cover
    MistralLLM = None  # type: ignore[assignment]

try:  # pragma: no cover - optional dependency
    from .qwen_coder_interface import QwenCoderLLM
except Exception:  # pragma: no cover
    QwenCoderLLM = None  # type: ignore[assignment]

# ``LLMManager`` and the prompt helpers pull in a large dependency graph.  They
# are optional for most unit tests, so import them lazily and fall back to stubs
# when dependencies are missing.  This keeps importing :mod:`src.llm` lightweight
# which is important for the test environment.
try:  # pragma: no cover - optional imports
    from .manager import LLMManager, ModelSpec, Task
    from .prompts import chat_prompt, apply_user_style
except Exception:  # pragma: no cover - any import issue
    LLMManager = ModelSpec = Task = None  # type: ignore[assignment]

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
