"""Проверяю временную последовательность событий истории."""

from __future__ import annotations

from typing import List

from src.memory.story_timeline import StoryTimeline


class TimelineChecker:
    """Сравниваю события в :class:`StoryTimeline` и ищу конфликты."""

    def __init__(self, timeline: StoryTimeline | None = None) -> None:
        self.timeline = timeline or StoryTimeline()

    # ------------------------------------------------------------------
    def check(self) -> List[str]:
        """Вернуть описания конфликтов во временной линии."""
        events = self.timeline.get()
        conflicts: List[str] = []
        items = list(events.items())
        for i, (name_a, data_a) in enumerate(items):
            start_a = data_a.get("start")
            end_a = data_a.get("end")
            if start_a is None or end_a is None:
                continue
            for name_b, data_b in items[i + 1 :]:
                start_b = data_b.get("start")
                end_b = data_b.get("end")
                if start_b is None or end_b is None:
                    continue
                if start_a < end_b and start_b < end_a:
                    conflicts.append(f"{name_a} overlaps with {name_b}")
        return conflicts


__all__ = ["TimelineChecker"]
