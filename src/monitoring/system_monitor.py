from __future__ import annotations

"""Aggregate system metrics from different monitors."""

from dataclasses import dataclass, field
from typing import Any, Dict, TYPE_CHECKING

from .metrics_monitor import MetricsMonitor
from .performance_profiler import PerformanceProfiler

# Lazy loading utilities for ResourceManager to avoid heavy imports at module level
import importlib.util
import sys
from pathlib import Path


def _load_resource_manager() -> type:
    path = Path(__file__).resolve().parent.parent / "iteration" / "resource_manager.py"
    spec = importlib.util.spec_from_file_location("_resource_manager", path)
    module = importlib.util.module_from_spec(spec)
    assert spec and spec.loader  # for type checkers
    sys.modules["_resource_manager"] = module
    spec.loader.exec_module(module)  # type: ignore[union-attr]
    return module.ResourceManager


def get_resource_manager() -> type:
    """Return the ``ResourceManager`` class."""

    return _load_resource_manager()


if TYPE_CHECKING:  # pragma: no cover - imported only for type hints
    from src.iteration.resource_manager import ResourceManager


@dataclass
class SystemMonitor:
    """Collect metrics from multiple monitoring utilities."""

    metrics_monitor: MetricsMonitor
    performance_profiler: PerformanceProfiler
    resource_manager: "ResourceManager"
    components: Dict[str, Any] = field(default_factory=dict)

    # ------------------------------------------------------------------
    def register_component(self, name: str, component: Any) -> None:
        """Register a component by ``name``."""

        self.components[name] = component

    # ------------------------------------------------------------------
    def get_metrics(self) -> Dict[str, Any]:
        """Return a snapshot of collected metrics."""

        cpu, memory = self.resource_manager.update_usage()
        data = {
            "components": list(self.components.keys()),
            "metrics_monitor": {"time_series": self.metrics_monitor.time_series},
            "performance_profiler": {"metrics": self.performance_profiler.metrics},
            "resource_manager": {
                "cpu": cpu,
                "memory": memory,
                "allocations": self.resource_manager.allocations,
            },
        }
        return data


# ---------------------------------------------------------------------------
def main() -> None:  # pragma: no cover - CLI helper
    """Simple CLI interface printing current metrics."""

    import argparse
    import json
    import pprint

    parser = argparse.ArgumentParser(description="Display system metrics")
    parser.add_argument(
        "--json", action="store_true", help="Output metrics as JSON"
    )
    args = parser.parse_args()

    monitor = SystemMonitor(
        metrics_monitor=MetricsMonitor(),
        performance_profiler=PerformanceProfiler(),
        resource_manager=get_resource_manager()(),
    )
    data = monitor.get_metrics()

    if args.json:
        print(json.dumps(data, ensure_ascii=False))
    else:
        pprint.pprint(data)


if __name__ == "__main__":  # pragma: no cover - CLI entry point
    main()
