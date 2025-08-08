import asyncio
import time
import sys
from pathlib import Path

import pytest

sys.path.append(str(Path(__file__).resolve().parents[2]))

from src.core.event_bus import EventBus, Event


def wait_for(predicate, timeout: float = 1.0) -> None:
    end = time.time() + timeout
    while time.time() < end:
        if predicate():
            return
        time.sleep(0.05)
    raise AssertionError("timeout waiting for event")


def test_event_bus_sync_and_async_handlers() -> None:
    bus = EventBus()
    results: list[int] = []

    def sync_handler(event: Event[int]) -> None:
        results.append(event.payload)

    async def async_handler(event: Event[int]) -> None:
        await asyncio.sleep(0.01)
        results.append(event.payload * 2)

    bus.subscribe("number", sync_handler)
    bus.subscribe("number", async_handler)
    bus.publish(Event("number", 21))

    wait_for(lambda: results)
    assert results == [21, 42]


def test_event_bus_rejects_invalid_token() -> None:
    bus = EventBus()
    with pytest.raises(PermissionError):
        bus.publish(Event("number", 1), token="invalid")
