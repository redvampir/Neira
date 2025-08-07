import time
import sys
import types
from src.iteration.smart_cache import SmartCache

chat_session_stub = types.ModuleType("src.interaction.chat_session")
chat_session_stub.ChatSession = object
sys.modules.setdefault("src.interaction.chat_session", chat_session_stub)

from src.core.neyra_brain import Neyra


def test_entry_expires(tmp_path):
    cache = SmartCache(cache_dir=tmp_path)
    cache.set("q", "v", ttl=0.1)
    assert cache.get("q") == "v"
    time.sleep(0.2)
    assert cache.get("q") is None


def test_cleanup_called_during_iteration(monkeypatch, tmp_path):
    neyra = Neyra()
    calls = {"count": 0}

    def fake_cleanup():
        calls["count"] += 1

    neyra.cache = SmartCache(cache_dir=tmp_path)
    neyra.cache.cleanup = fake_cleanup  # type: ignore

    monkeypatch.setattr(neyra, "process_command", lambda q: "base text")
    monkeypatch.setattr(neyra.gap_analyzer, "analyze", lambda draft: [])

    neyra.iterative_response("query")
    assert calls["count"] == 1
