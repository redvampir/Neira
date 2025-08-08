import sys
import types
from pathlib import Path

# Provide a stub for the optional Rust extension
sys.modules.setdefault("neira_rust", types.SimpleNamespace(ping=lambda: "pong"))

sys.path.append(str(Path(__file__).resolve().parents[2]))

from src.monitoring.performance_profiler import PerformanceProfiler


def time_heavy_operation():
    for _ in range(10000):
        pass
    return "done"


def memory_heavy_operation():
    data = [0] * 10000
    return len(data)


def test_profile_operation_records_metrics():
    profiler = PerformanceProfiler()
    result = profiler.profile_operation("mem_op", memory_heavy_operation)

    assert result == 10000
    assert "mem_op" in profiler.metrics
    metrics = profiler.metrics["mem_op"]
    assert metrics["time"] >= 0
    assert metrics["memory"] >= 0
    assert profiler.resource_usage["mem_op"] == metrics


def test_suggest_optimizations_identifies_bottlenecks():
    profiler = PerformanceProfiler()
    profiler.profile_operation("time_op", time_heavy_operation)
    profiler.profile_operation("mem_op", memory_heavy_operation)

    suggestions = profiler.suggest_optimizations()

    assert len(suggestions) >= 1
    assert any("time_op" in s for s in suggestions)
    assert any("mem_op" in s for s in suggestions)
    assert set(profiler.bottlenecks.keys()) == {"time_op", "mem_op"}
    assert profiler.optimization_history == suggestions
