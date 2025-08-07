from src.analysis.timeline_checker import TimelineChecker
from src.memory.story_timeline import StoryTimeline


def test_no_conflicts():
    timeline = StoryTimeline()
    timeline.add("begin", {"start": 0, "end": 1})
    timeline.add("middle", {"start": 2, "end": 3})
    checker = TimelineChecker(timeline)
    assert checker.check() == []


def test_detects_overlapping_events():
    timeline = StoryTimeline()
    timeline.add("a", {"start": 0, "end": 5})
    timeline.add("b", {"start": 3, "end": 6})
    checker = TimelineChecker(timeline)
    conflicts = checker.check()
    assert conflicts == ["a overlaps with b"]
