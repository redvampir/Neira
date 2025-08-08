import time
import sys
import types
from pathlib import Path

root = Path(__file__).resolve().parents[2]
src_pkg = types.ModuleType("src")
src_pkg.__path__ = [str(root / "src")]
sys.modules.setdefault("src", src_pkg)

iteration_pkg = types.ModuleType("src.iteration")
iteration_pkg.__path__ = [str(root / "src" / "iteration")]
sys.modules.setdefault("src.iteration", iteration_pkg)

core_pkg = types.ModuleType("src.core")
core_pkg.__path__ = [str(root / "src" / "core")]
sys.modules.setdefault("src.core", core_pkg)

neira_rust_stub = types.ModuleType("neira_rust")
neira_rust_stub.KnowledgeGraph = object
neira_rust_stub.MemoryIndex = object
neira_rust_stub.ping = lambda: "pong"
neira_rust_stub.VerificationResult = object
neira_rust_stub.verify_claim = lambda *a, **k: neira_rust_stub.VerificationResult
sys.modules.setdefault("neira_rust", neira_rust_stub)

from src.iteration.smart_cache import SmartCache

chat_session_stub = types.ModuleType("src.interaction.chat_session")
chat_session_stub.ChatSession = object
sys.modules.setdefault("src.interaction.chat_session", chat_session_stub)

def test_entry_expires(tmp_path):
    cache = SmartCache(cache_dir=tmp_path)
    cache.set("q", "v", ttl=0.1)
    assert cache.get("q") == "v"
    time.sleep(0.2)
    assert cache.get("q") is None


def test_stale_cleanup(tmp_path):
    cache = SmartCache(cache_dir=tmp_path, stale_after=0.1)
    cache.set("q", "v")
    time.sleep(0.2)
    cache.cleanup()
    assert cache.get("q") is None


