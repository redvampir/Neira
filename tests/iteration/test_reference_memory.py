"""Tests for the ReferenceMemory component."""

from src.iteration.reference_memory import ReferenceMemory
from src.utils.source_manager import SourceManager
from src.utils.source_tracker import SourceTracker


class DummySearcher:
    """Simple stub for DeepSearcher that records search queries."""

    def __init__(self) -> None:
        self.queries = []

    def search(self, query: str, user_id=None, limit: int = 5):
        self.queries.append(query)
        return []


def test_create_reference_link_internal(tmp_path):
    tracker = SourceTracker()
    manager = SourceManager(tracker=tracker)
    searcher = DummySearcher()
    memory = ReferenceMemory(manager=manager, searcher=searcher)

    file = tmp_path / "file.txt"
    file.write_text("hello", encoding="utf-8")

    link = memory.create_reference_link("test summary", str(file), 0.9)

    assert link == f"[test summary]({file})"
    assert memory.internal_sources[0].path == str(file)
    assert tracker.entries[0].source == str(file)
    assert searcher.queries == ["test summary"]


def test_create_reference_link_external():
    tracker = SourceTracker()
    manager = SourceManager(tracker=tracker)
    searcher = DummySearcher()
    memory = ReferenceMemory(manager=manager, searcher=searcher)

    url = "https://example.com"

    link = memory.create_reference_link("external", url, 0.8)

    assert link == f"[external]({url})"
    assert memory.external_sources[0].path == url
    assert tracker.entries[0].source == url
    assert searcher.queries == ["external"]
