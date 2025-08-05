import sys
from pathlib import Path

# Ensure project root is on sys.path for src layout
sys.path.append(str(Path(__file__).resolve().parents[1]))

from src.processing.queue import ProcessingQueue
from src.processing.types import Task, Priority


def make_task(name: str) -> Task:
    return Task(func=lambda: name)


def test_processing_queue_runs_tasks_by_priority():
    queue = ProcessingQueue()

    queue.add_task(make_task("low"), Priority.LOW)
    queue.add_task(make_task("high1"), Priority.HIGH)
    queue.add_task(make_task("medium"), Priority.MEDIUM)
    queue.add_task(make_task("high2"), Priority.HIGH)

    results = []
    while True:
        result = queue.process_next()
        if result is None:
            break
        results.append(result)

    assert results == ["high1", "high2", "medium", "low"]
