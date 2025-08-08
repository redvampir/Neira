from src.search.api_client import SearchAPIClient


class DummyResponse:
    def __init__(self, headers=None, text=""):
        self.headers = headers or {}
        self.text = text


def test_check_license_header(monkeypatch, tmp_path):
    cfg = tmp_path / "licenses.yml"
    cfg.write_text("allowed_licenses:\n  - CC-BY\n", encoding="utf-8")
    client = SearchAPIClient(license_config_path=cfg)
    monkeypatch.setattr(
        client.session,
        "get",
        lambda url, timeout=5: DummyResponse({"License": "CC-BY"}),
    )
    assert client.check_license("https://example.com")


def test_check_license_meta(monkeypatch, tmp_path):
    cfg = tmp_path / "licenses.yml"
    cfg.write_text("allowed_licenses:\n  - CC-BY\n", encoding="utf-8")
    client = SearchAPIClient(license_config_path=cfg)
    html = '<html><head><meta name="license" content="CC-BY"></head></html>'
    monkeypatch.setattr(
        client.session,
        "get",
        lambda url, timeout=5: DummyResponse(text=html),
    )
    assert client.check_license("https://example.com")


def test_search_and_update_skips_unlicensed(monkeypatch, tmp_path):
    cfg = tmp_path / "licenses.yml"
    cfg.write_text("allowed_licenses:\n  - CC-BY\n", encoding="utf-8")

    def fake_fetch(query: str, limit: int):
        return [{"url": "https://example.com", "snippet": "Fact"}]

    client = SearchAPIClient(fetcher=fake_fetch, license_config_path=cfg)
    monkeypatch.setattr(client, "check_license", lambda url: False)
    client.search_and_update("test")
    assert "Fact" not in client.memory.cold_storage
