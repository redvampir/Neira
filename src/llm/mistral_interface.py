"""Mistral LLM interface using llama-cpp-python."""
from __future__ import annotations

from typing import Iterable, Optional

from src.core.config import get_logger
from .base_llm import BaseLLM, LLMFactory, get_available_vram

# The real implementation relies on ``llama_cpp`` which may not be available
# in lightweight environments (like the test environment for this kata).
# Import the class lazily and provide a helpful fallback so that the module can
# be imported even when the dependency is missing.  Tests only require the
# class to exist – they don't actually instantiate the heavy model.
try:  # pragma: no cover - simple import guard
    from llama_cpp import Llama  # type: ignore
except ModuleNotFoundError:  # pragma: no cover
    Llama = None  # type: ignore


logger = get_logger(__name__)


class MistralLLM(BaseLLM):
    """Wrapper around a local Mistral GGUF model."""

    model_name = "mistral"

    def __init__(
        self,
        model_path: str,
        *,
        n_gpu_layers: int = 0,
        n_ctx: int = 2048,
        n_batch: int = 512,
        use_mmap: bool = True,
        use_mlock: bool = False,
        seed: int = 0,
    ) -> None:
        """Store configuration for later lazy loading.

        Parameters mirror those of :class:`llama_cpp.Llama` so that tests can
        verify we pass the expected values even though the heavy dependency is
        not actually loaded in the test environment.
        """

        self.model_path = model_path
        self.n_gpu_layers = n_gpu_layers
        self.n_ctx = n_ctx
        self.n_batch = n_batch
        self.use_mmap = use_mmap
        self.use_mlock = use_mlock
        self.seed = seed

        self.model: Optional["Llama"] = None
        self._load_error: Optional[str] = None

    # ------------------------------------------------------------------
    def _ensure_vram(self) -> None:
        """Adapt configuration to the currently available VRAM."""

        available = get_available_vram()
        if available <= 0:  # Could not determine VRAM or no GPU
            return

        required = self.n_ctx * self.n_batch * 2 / 1024  # very rough estimate in MB
        while required > available and (self.n_ctx > 128 or self.n_batch > 1):
            if self.n_ctx > 128:
                self.n_ctx //= 2
            if self.n_batch > 1:
                self.n_batch = max(1, self.n_batch // 2)
            required = self.n_ctx * self.n_batch * 2 / 1024
            logger.warning(
                "Reducing context to %s and batch to %s due to low VRAM", self.n_ctx, self.n_batch
            )

        if required > available:
            logger.warning("Switching to CPU due to insufficient VRAM")
            self.n_gpu_layers = 0

    # ------------------------------------------------------------------
    def _load_model(self) -> None:
        """Load the underlying ``Llama`` model if it hasn't been loaded yet."""

        if self.model is not None:
            return
        if Llama is None:
            self._load_error = "llama_cpp is required to use MistralLLM"
            logger.error(self._load_error)
            return

        try:
            self._ensure_vram()
            logger.info("Loading Mistral model from %s", self.model_path)
            self.model = Llama(
                model_path=self.model_path,
                n_gpu_layers=self.n_gpu_layers,
                n_ctx=self.n_ctx,
                n_batch=self.n_batch,
                use_mmap=self.use_mmap,
                use_mlock=self.use_mlock,
                seed=self.seed,
                verbose=False,
            )
            logger.info("Mistral model loaded successfully")
            self._load_error = None
        except Exception as exc:  # pragma: no cover - error handling
            self._load_error = str(exc)
            logger.exception("Failed to load Mistral model: %s", exc)
            self.model = None

    def is_available(self) -> bool:
        """Return ``True`` if the model has been loaded successfully."""

        return self.model is not None and self._load_error is None

    # ------------------------------------------------------------------
    def generate(
        self,
        prompt: str,
        *,
        max_tokens: int = 512,
        temperature: float = 0.8,
        top_p: float = 0.95,
        repeat_penalty: float = 1.1,
        stop: Optional[Iterable[str]] = None,
    ) -> str:
        """Generate text from the given ``prompt`` using the loaded model."""

        self._load_model()
        if not self.is_available():
            raise RuntimeError(self._load_error or "model failed to load")

        result = self.model(  # type: ignore[call-arg]
            prompt,
            max_tokens=max_tokens,
            temperature=temperature,
            top_p=top_p,
            repeat_penalty=repeat_penalty,
            stop=stop or ["</s>"],
        )
        return result["choices"][0]["text"].strip()

    # ------------------------------------------------------------------
    @classmethod
    def is_backend_available(cls) -> bool:  # pragma: no cover - simple availability check
        return Llama is not None


# Register the implementation in the factory
LLMFactory.register("mistral", MistralLLM)
