from __future__ import annotations

from abc import ABC, abstractmethod
from typing import Any, Dict, Type


class BaseLLM(ABC):
    """Abstract base class for all LLM implementations."""

    model_name: str

    @abstractmethod
    def generate(self, prompt: str, max_tokens: int = 512) -> str:
        """Generate text based on ``prompt``."""
        raise NotImplementedError

    @abstractmethod
    def is_available(self) -> bool:
        """Return ``True`` if the model has been loaded successfully."""
        raise NotImplementedError

    @classmethod
    def is_backend_available(cls) -> bool:  # pragma: no cover - simple availability check
        """Return ``True`` if the backend dependencies are installed."""
        return True


class LLMFactory:
    """Factory for creating LLM instances by ``model_type``."""

    _registry: Dict[str, Type[BaseLLM]] = {}

    @classmethod
    def register(cls, model_type: str, llm_cls: Type[BaseLLM]) -> None:
        """Register an ``llm_cls`` under ``model_type``."""
        cls._registry[model_type] = llm_cls

    @classmethod
    def create(cls, model_type: str, **kwargs: Any) -> BaseLLM:
        """Instantiate the LLM associated with ``model_type``."""
        llm_cls = cls._registry.get(model_type)
        if llm_cls is None:
            raise ValueError(f"Unknown model_type: {model_type}")
        if not llm_cls.is_backend_available():
            raise RuntimeError(f"{llm_cls.model_name} is not available")
        return llm_cls(**kwargs)
