from __future__ import annotations

"""Priority based task scheduler with emotion-aware ordering."""

from enum import Enum
from queue import Empty, Queue
import time
from typing import Any, Callable, Dict, List

from .event_bus import Event, EventBus
from src.emotions.engine import NeyraEmotions

Task = Callable[[], Any]


class Priority(Enum):
    """Task priority levels."""

    HIGH = "high"
    NORMAL = "normal"
    LOW = "low"


class PriorityScheduler:
    """Manage tasks in three priority queues.

    The scheduler listens to ``task.completed`` events on the provided
    :class:`EventBus`.  After each task the associated
    :class:`NeyraEmotions` instance is updated which dynamically affects
    the order in which future tasks are processed.
    """

    def __init__(self, event_bus: EventBus, emotions: NeyraEmotions | None = None) -> None:
        self.event_bus = event_bus
        self.emotions = emotions or NeyraEmotions()
        self.queues: Dict[Priority, Queue[Task]] = {
            Priority.HIGH: Queue(),
            Priority.NORMAL: Queue(),
            Priority.LOW: Queue(),
        }
        self.event_bus.subscribe("task.completed", self._on_task_completed)

    # ------------------------------------------------------------------
    def schedule(self, task: Task, priority: Priority = Priority.NORMAL) -> None:
        """Add ``task`` to the queue with the given ``priority``."""

        self.queues[priority].put(task)

    # ------------------------------------------------------------------
    def run(self) -> None:
        """Run tasks until all queues are empty."""

        while any(not q.empty() for q in self.queues.values()):
            for priority in self._priority_order():
                queue = self.queues[priority]
                if queue.empty():
                    continue
                task = queue.get_nowait()
                success = True
                name = getattr(task, "__name__", "task")
                try:
                    task()
                except Exception:
                    success = False
                prev = self.emotions.mood
                self.event_bus.publish(Event("task.completed", {"task": name, "success": success}))
                # wait briefly for emotion handler to run
                start = time.time()
                while self.emotions.mood == prev and time.time() - start < 0.1:
                    time.sleep(0.005)
                break

    # ------------------------------------------------------------------
    def _priority_order(self) -> List[Priority]:
        mood = self.emotions.mood
        if mood > 0.7:
            return [Priority.HIGH, Priority.NORMAL, Priority.LOW]
        if mood < 0.3:
            return [Priority.LOW, Priority.NORMAL, Priority.HIGH]
        return [Priority.NORMAL, Priority.HIGH, Priority.LOW]

    # ------------------------------------------------------------------
    def _on_task_completed(self, event: Event[Dict[str, Any]]) -> None:
        payload = event.payload
        task = str(payload.get("task", "task"))
        success = bool(payload.get("success", True))
        self.emotions.update_mood_from_task(task, success)


__all__ = ["Priority", "PriorityScheduler"]
