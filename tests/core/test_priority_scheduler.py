import sys
import time
from pathlib import Path

sys.path.append(str(Path(__file__).resolve().parents[2]))

from src.core.event_bus import EventBus
from src.core.priority_scheduler import PriorityScheduler, Priority
from src.emotions.engine import NeyraEmotions


def test_scheduler_dynamic_priorities() -> None:
    bus = EventBus()
    emotions = NeyraEmotions(mood=0.65)
    scheduler = PriorityScheduler(bus, emotions)
    order: list[str] = []

    def high() -> None:
        order.append("high")

    def normal() -> None:
        order.append("normal")

    def low() -> None:
        order.append("low")

    scheduler.schedule(high, Priority.HIGH)
    scheduler.schedule(normal, Priority.NORMAL)
    scheduler.schedule(low, Priority.LOW)

    scheduler.run()
    # ensure bus processed final event
    time.sleep(0.05)
    assert order == ["normal", "high", "low"]
    assert emotions.mood > 0.7
