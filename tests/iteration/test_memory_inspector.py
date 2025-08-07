"""Tests for the MemoryInspector component."""

from src.iteration.reference_memory import ReferenceMemory
from src.iteration.memory_inspector import MemoryInspector
from src.utils.source_tracker import SourceTracker


class DummySearcher:
    """Stub deep searcher that records queries."""

    def __init__(self) -> None:
        self.queries = []

    def search(self, query: str, user_id=None, limit: int = 5):  # noqa: D401 - simple stub
        self.queries.append(query)
        return []


def test_linked_sources_and_report(tmp_path):
    tracker = SourceTracker()
    searcher = DummySearcher()
    memory = ReferenceMemory(tracker=tracker, searcher=searcher)

    file = tmp_path / "file.txt"
    file.write_text("hello")

    memory.create_reference_link("internal", str(file), 0.9)
    url = "https://example.com"
    memory.create_reference_link("external", url, 0.8)

    inspector = MemoryInspector(memory)
    sources = inspector.linked_sources()

    assert len(sources) == 2
    assert sources[0].path == str(file)
    assert sources[1].path == url

    report = inspector.generate_report()
    assert "internal" in report and str(file) in report
    assert "external" in report and url in report


def test_reference_memory_report(tmp_path):
    tracker = SourceTracker()
    searcher = DummySearcher()
    memory = ReferenceMemory(tracker=tracker, searcher=searcher)

    file = tmp_path / "note.txt"
    file.write_text("hi")

    memory.create_reference_link("note", str(file), 0.7)

    report = memory.report()
    assert "note" in report
    assert str(file) in report
