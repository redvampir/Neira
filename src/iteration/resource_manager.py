from __future__ import annotations

"""Simple resource detection and iteration configuration utilities."""

from dataclasses import dataclass
from typing import Any, Dict, List, Tuple
import os
import heapq
import math
from collections import defaultdict, deque

from optimization import UsagePredictor

try:  # pragma: no cover - optional dependency
    import psutil
except Exception:  # pragma: no cover - handled gracefully
    psutil = None


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

        # Track currently available CPU resources (simplified model)
        self.available_cpu = self.cpu_cores

        # Active allocations {component: amount}
        self.allocations: Dict[Any, int] = {}

        # Priority queue of pending allocation requests: [(-priority, component, amount)]
        self._queue: List[Tuple[int, Any, int]] = []

        # Historical usage statistics for moving average per component
        self.resource_usage: Dict[Any, deque[int]] = defaultdict(lambda: deque(maxlen=5))

        # Demand forecasting helper
        self._predictor = UsagePredictor()

        # System usage metrics updated via psutil
        self.current_cpu_usage: float = 0.0
        self.current_memory_usage: float = 0.0
        self._update_usage()

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

    # ------------------------------------------------------------------
    def _update_usage(self) -> None:
        """Refresh current CPU and memory usage statistics using ``psutil``."""

        if psutil is None:  # pragma: no cover - optional dependency
            self.current_cpu_usage = 0.0
            self.current_memory_usage = 0.0
            return

        try:  # pragma: no cover - system dependent values
            self.current_cpu_usage = float(psutil.cpu_percent(interval=0))
            self.current_memory_usage = float(psutil.virtual_memory().percent)
        except Exception:
            self.current_cpu_usage = 0.0
            self.current_memory_usage = 0.0

    def update_usage(self) -> Tuple[float, float]:
        """Public helper returning current CPU and memory usage."""

        self._update_usage()
        return self.current_cpu_usage, self.current_memory_usage

    # ------------------------------------------------------------------
    def _record_usage(self, component: Any, amount: int) -> None:
        """Store historical usage for ``component``."""

        self.resource_usage[component].append(amount)
        self._predictor.record(component, amount)

    def get_moving_average(self, component: Any, window: int = 5) -> float:
        """Return moving average of recent allocations for ``component``."""

        data = list(self.resource_usage.get(component, []))[-window:]
        if not data:
            return 0.0
        return sum(data) / len(data)

    def predict_next_demand(self, component: Any) -> float:
        """Forecast the next allocation amount for ``component``."""

        return self._predictor.predict(component)

    # ------------------------------------------------------------------
    def _schedule(self) -> None:
        """Attempt to allocate resources to queued components based on priority."""

        if not self._queue:
            return

        pending: List[Tuple[int, Any, int]] = []
        while self._queue:
            priority, component, amount = heapq.heappop(self._queue)
            predicted = max(float(amount), self.predict_next_demand(component))
            required = int(math.ceil(predicted))
            if required <= self.available_cpu:
                self.available_cpu -= required
                self.allocations[component] = required
                self._record_usage(component, required)
            else:
                pending.append((priority, component, amount))

        for item in pending:
            heapq.heappush(self._queue, item)

        self._update_usage()

    # ------------------------------------------------------------------
    def allocate(self, component: Any, amount: int) -> bool:
        """Request allocation for ``component`` and return ``True`` if granted."""

        priority = getattr(component, "priority", 0)
        heapq.heappush(self._queue, (-int(priority), component, int(amount)))
        self._schedule()
        return component in self.allocations

    # ------------------------------------------------------------------
    def release(self, component: Any) -> None:
        """Release resources held by ``component``."""

        amount = self.allocations.pop(component, 0)
        if amount:
            self.available_cpu += amount
            self._update_usage()
            self._schedule()


__all__ = ["ResourceManager", "IterationConfig"]
