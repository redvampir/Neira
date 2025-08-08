from __future__ import annotations

"""Simple resource detection and iteration configuration utilities."""

from dataclasses import dataclass
from typing import Any, Dict
import os


@dataclass
class IterationConfig:
    """Configuration describing how iterative components should behave.

    Attributes
    ----------
    max_iterations:
        Upper bound for the number of refinement iterations.
    parallel:
        Whether expensive operations are allowed to run concurrently.
    cache:
        Dictionary describing cache limits. ``"hot_limit"`` and ``"warm_limit"``
        are the primary keys used by :class:`SmartCache`.
    """

    max_iterations: int
    parallel: bool
    cache: Dict[str, int]

    # Provide dictionary-like access used in existing code base
    def __getitem__(self, key: str) -> Any:  # pragma: no cover - trivial
        return getattr(self, key)

    def get(self, key: str, default: Any | None = None) -> Any:  # pragma: no cover
        return getattr(self, key, default)


class ResourceManager:
    """Evaluate available resources and derive :class:`IterationConfig`."""

    def __init__(self, gpu_memory: float | None = None, cpu_cores: int | None = None) -> None:
        self.gpu_memory = gpu_memory if gpu_memory is not None else self._detect_gpu_memory()
        self.cpu_cores = cpu_cores if cpu_cores is not None else self._detect_cpu_cores()

    # ------------------------------------------------------------------
    @staticmethod
    def _detect_gpu_memory() -> float:
        """Return total memory of the first CUDA device in GB.

        When CUDA or the ``torch`` package is unavailable ``0`` is returned. The
        implementation intentionally remains lightweight as unit tests provide
        explicit values.
        """

        try:  # pragma: no cover - hardware specific
            import torch

            if torch.cuda.is_available():
                # ``mem_get_info`` returns ``(free, total)`` in bytes
                _, total = torch.cuda.mem_get_info()  # type: ignore[call-arg]
                return total / (1024 ** 3)
        except Exception:  # pragma: no cover - optional dependency
            return 0.0
        return 0.0

    # ------------------------------------------------------------------
    @staticmethod
    def _detect_cpu_cores() -> int:
        """Return the number of available CPU cores."""

        return os.cpu_count() or 1

    # ------------------------------------------------------------------
    def get_config(self) -> IterationConfig:
        """Return a configuration tuned to detected resources."""

        gpu = self.gpu_memory
        cpu = self.cpu_cores

        if gpu < 4 or cpu < 4:
            config = IterationConfig(
                max_iterations=2,
                parallel=False,
                cache={"hot_limit": 4, "warm_limit": 16},
            )
        elif gpu < 8 or cpu < 8:
            config = IterationConfig(
                max_iterations=4,
                parallel=False,
                cache={"hot_limit": 8, "warm_limit": 32},
            )
        else:
            config = IterationConfig(
                max_iterations=8,
                parallel=True,
                cache={"hot_limit": 32, "warm_limit": 128},
            )
        return config


__all__ = ["ResourceManager", "IterationConfig"]
