from src.search.api_client import SearchAPIClient
from src.memory import MemoryIndex
import json
import pytest


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
    client.check_license = lambda url: True
    results = client.search_and_update("test")

    assert [r["url"] for r in results] == ["https://a", "https://b"]
    assert "A fact" in mem.cold_storage
    assert "B fact" in mem.cold_storage
    assert mem.source_reliability["https://a"] > 0.9
    assert mem.source_reliability["https://b"] > 0.2


def test_domain_filtering(tmp_path):
    config_path = tmp_path / "domains.json"
    config_path.write_text(
        json.dumps(
            {
                "allowed_domains": ["good.com"],
                "blocked_domains": ["bad.com"],
            }
        ),
        encoding="utf-8",
    )

    def fake_fetch(query: str, limit: int):
        return [
            {"url": "https://good.com/a", "snippet": "ok"},
            {"url": "https://bad.com/a", "snippet": "bad"},
            {"url": "https://other.com/a", "snippet": "other"},
        ]

    client = SearchAPIClient(fetcher=fake_fetch, domain_config_path=config_path)
    results = client.search("test")
    assert [r["url"] for r in results] == ["https://good.com/a"]


def test_search_rejects_invalid_token():
    client = SearchAPIClient(fetcher=lambda q, l: [], token="bad")
    with pytest.raises(PermissionError):
        client.search("test")
