from __future__ import annotations

"""Simple performance profiling utilities."""

from dataclasses import dataclass, field
from typing import Any, Callable, Dict, List
import time
import tracemalloc


@dataclass
class PerformanceProfiler:
    """Collect execution time and memory metrics for operations."""

    metrics: Dict[str, Dict[str, float]] = field(default_factory=dict)
    bottlenecks: Dict[str, Dict[str, float]] = field(default_factory=dict)
    optimization_history: List[str] = field(default_factory=list)
    resource_usage: Dict[str, Dict[str, float]] = field(default_factory=dict)

    # ------------------------------------------------------------------
    def profile_operation(
        self, name: str, func: Callable[..., Any], *args: Any, **kwargs: Any
    ) -> Any:
        """Profile ``func`` execution and store its metrics.

        Parameters
        ----------
        name:
            Identifier of the operation.
        func:
            Callable to execute and profile.
        *args, **kwargs:
            Arguments forwarded to ``func``.

        Returns
        -------
        Any
            Result returned by ``func``.
        """

        tracemalloc.start()
        start = time.perf_counter()
        result = func(*args, **kwargs)
        duration = time.perf_counter() - start
        current, peak = tracemalloc.get_traced_memory()
        tracemalloc.stop()

        data = {"time": duration, "memory": float(peak)}
        self.metrics[name] = data
        self.resource_usage[name] = data
        return result

    # ------------------------------------------------------------------
    def suggest_optimizations(self) -> List[str]:
        """Analyse collected metrics and suggest optimizations."""

        if not self.metrics:
            return []

        max_time_op = max(self.metrics.items(), key=lambda kv: kv[1]["time"])[0]
        max_mem_op = max(self.metrics.items(), key=lambda kv: kv[1]["memory"])[0]

        suggestions: List[str] = []
        for op in {max_time_op, max_mem_op}:
            metrics = self.metrics[op]
            suggestion = (
                f"Consider optimizing '{op}' "
                f"(time: {metrics['time']:.6f}s, memory: {metrics['memory']} bytes)"
            )
            suggestions.append(suggestion)
            self.bottlenecks[op] = metrics

        self.optimization_history.extend(suggestions)
        return suggestions


__all__ = ["PerformanceProfiler"]
