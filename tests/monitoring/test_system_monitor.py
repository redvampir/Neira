import sys
import types
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.append(str(ROOT))
sys.path.append(str(ROOT / "src"))

# Provide stub for optional extension
sys.modules.setdefault(
    "neira_rust",
    types.SimpleNamespace(
        KnowledgeGraph=object,
        MemoryIndex=object,
        VerificationResult=object,
        verify_claim=lambda *a, **k: None,
        ping=lambda: "pong",
    ),
)

from src.monitoring.metrics_monitor import MetricsMonitor
from src.monitoring.performance_profiler import PerformanceProfiler
from src.monitoring.system_monitor import SystemMonitor, get_resource_manager


def test_system_monitor_collects_metrics() -> None:
    metrics_monitor = MetricsMonitor()
    profiler = PerformanceProfiler()
    RM = get_resource_manager()
    resource_manager = RM(gpu_memory=0, cpu_cores=1)

    monitor = SystemMonitor(metrics_monitor, profiler, resource_manager)
    monitor.register_component("dummy", object())

    metrics_monitor.log_performance_metrics(cpu=10)
    profiler.profile_operation("op", lambda: None)

    metrics = monitor.get_metrics()

    assert "dummy" in metrics["components"]
    assert "op" in metrics["performance_profiler"]["metrics"]
    assert "cpu" in metrics["resource_manager"]
    assert "memory" in metrics["resource_manager"]
    assert "cpu" in metrics["metrics_monitor"]["time_series"]
