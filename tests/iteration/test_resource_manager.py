import sys
import math
import pytest

try:  # pragma: no cover - optional dependency
    import psutil
except Exception:  # pragma: no cover - handled gracefully
    psutil = None  # type: ignore[assignment]

import importlib.util
from pathlib import Path

sys.path.append(str(Path(__file__).resolve().parents[2] / "src"))

module_path = Path(__file__).resolve().parents[2] / "src/iteration/resource_manager.py"
spec = importlib.util.spec_from_file_location("resource_manager", module_path)
resource_manager = importlib.util.module_from_spec(spec)
assert spec.loader is not None
sys.modules[spec.name] = resource_manager
spec.loader.exec_module(resource_manager)  # type: ignore[assignment]
ResourceManager = resource_manager.ResourceManager


def test_low_resource_config() -> None:
    manager = ResourceManager(gpu_memory=2, cpu_cores=2)
    cfg = manager.get_config()
    assert cfg.max_iterations == 2
    assert cfg.parallel is False
    assert cfg.cache["hot_limit"] == 4


def test_high_resource_config() -> None:
    manager = ResourceManager(gpu_memory=16, cpu_cores=16)
    cfg = manager.get_config()
    assert cfg.max_iterations == 8
    assert cfg.parallel is True
    assert cfg.cache["hot_limit"] == 32




def test_system_usage(monkeypatch) -> None:
    if psutil is None:
        pytest.skip("psutil not installed")

    class DummyMem:
        percent = 40.0

    def fake_cpu_percent(interval=0):
        return 10.0

    def fake_virtual_memory():
        return DummyMem()

    monkeypatch.setattr(psutil, "cpu_percent", fake_cpu_percent)
    monkeypatch.setattr(psutil, "virtual_memory", fake_virtual_memory)

    manager = ResourceManager(gpu_memory=0, cpu_cores=1)
    assert manager.current_cpu_usage == 10.0
    assert manager.current_memory_usage == 40.0


def test_allocation_priority() -> None:
    class Comp:
        def __init__(self, name: str, priority: int) -> None:
            self.name = name
            self.priority = priority

    manager = ResourceManager(gpu_memory=0, cpu_cores=4)
    a = Comp("a", priority=1)
    b = Comp("b", priority=10)
    c = Comp("c", priority=5)

    assert manager.allocate(a, 3) is True
    assert manager.allocate(b, 3) is False  # queued
    assert manager.allocate(c, 1) is True   # high priority fits
    manager.release(a)
    assert b in manager.allocations


def test_moving_average() -> None:
    class Comp:
        def __init__(self, priority: int) -> None:
            self.priority = priority

    c = Comp(priority=1)
    manager = ResourceManager(gpu_memory=0, cpu_cores=10)
    manager.allocate(c, 2)
    manager.release(c)
    manager.allocate(c, 4)
    manager.release(c)
    manager.allocate(c, 6)
    manager.release(c)
    assert manager.get_moving_average(c) == 4.0


def test_demand_prediction() -> None:
    class Comp:
        def __init__(self, priority: int) -> None:
            self.priority = priority

    c = Comp(priority=1)
    manager = ResourceManager(gpu_memory=0, cpu_cores=10)
    # Repeating pattern to build history
    for amount in [2, 4, 2, 4]:
        manager._record_usage(c, amount)

    predicted = manager.predict_next_demand(c)
    assert predicted == pytest.approx(3.25, rel=1e-2)

    assert manager.allocate(c, 1) is True
    assert manager.allocations[c] == math.ceil(predicted)
