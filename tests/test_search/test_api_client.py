from src.search import SearchAPIClient
from src.memory import MemoryIndex


def test_search_ranking_and_update():
    mem = MemoryIndex()
    mem.source_reliability["https://a"] = 0.9
    mem.source_reliability["https://b"] = 0.2

    def fake_fetch(query: str, limit: int):
        return [
            {"url": "https://b", "snippet": "B fact. Another."},
            {"url": "https://a", "snippet": "A fact."},
        ]

    client = SearchAPIClient(memory=mem, fetcher=fake_fetch)
    results = client.search_and_update("test")

    assert [r["url"] for r in results] == ["https://a", "https://b"]
    assert "A fact" in mem.cold_storage
    assert "B fact" in mem.cold_storage
    assert mem.source_reliability["https://a"] > 0.9
    assert mem.source_reliability["https://b"] > 0.2
