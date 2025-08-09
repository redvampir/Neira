"""Profiler panel for code editor.

This module exposes a small helper around :class:`PerformanceProfiler`
from :mod:`src.monitoring` to gather execution metrics and present them
in a textual form that could be displayed in a user interface.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any, Callable, List

from src.monitoring import PerformanceProfiler


@dataclass
class ProfilerPanel:
    """Simple facade over :class:`PerformanceProfiler`."""

    profiler: PerformanceProfiler = field(default_factory=PerformanceProfiler)

    # ------------------------------------------------------------------
    def profile(self, name: str, func: Callable[..., Any], *args: Any, **kwargs: Any) -> Any:
        """Profile ``func`` and store metrics using ``name``."""

        return self.profiler.profile_operation(name, func, *args, **kwargs)

    # ------------------------------------------------------------------
    def report(self) -> str:
        """Return a formatted report of collected metrics."""

        lines: List[str] = ["Profiling report", "------------------"]
        for name, data in self.profiler.metrics.items():
            lines.append(f"{name}: {data['time']:.6f}s, {data['memory']} bytes")
        return "\n".join(lines)

    # ------------------------------------------------------------------
    def suggestions(self) -> List[str]:
        """Return optimization suggestions based on collected data."""

        return self.profiler.suggest_optimizations()
