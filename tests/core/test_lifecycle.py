from __future__ import annotations

from src.core.lifecycle import BaseModule, LifecycleManager


class DummyModule(BaseModule):
    def __init__(self, name: str, events: list[str]) -> None:
        self.name = name
        self.events = events
        self.health = True

    def start(self) -> None:
        self.events.append(f"{self.name}-start")

    def stop(self) -> None:
        self.events.append(f"{self.name}-stop")

    def health_check(self) -> bool:
        return self.health


def test_initialization_and_shutdown_order() -> None:
    events: list[str] = []
    manager = LifecycleManager()

    a = DummyModule("a", events)
    b = DummyModule("b", events)
    c = DummyModule("c", events)

    manager.register("a", a)
    manager.register("b", b, dependencies=["a"])
    manager.register("c", c, dependencies=["b"])

    manager.start_all()
    manager.stop_all()

    assert events == [
        "a-start",
        "b-start",
        "c-start",
        "c-stop",
        "b-stop",
        "a-stop",
    ]


def test_health_check_and_restart() -> None:
    events: list[str] = []
    manager = LifecycleManager()
    module = DummyModule("a", events)

    manager.register("a", module)
    manager.start_all()

    module.health = False
    manager.check_health()

    assert events == ["a-start", "a-stop", "a-start"]
