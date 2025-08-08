import sys
import types
import time
from pathlib import Path

# Stubs for optional dependencies
sys.modules.setdefault(
    "neira_rust",
    types.SimpleNamespace(
        ping=lambda: "pong",
        KnowledgeGraph=object,
        MemoryIndex=object,
        VerificationResult=object,
        verify_claim=lambda *_a, **_k: True,
        parse=lambda *_a, **_k: [],
        suggest_entities=lambda *_a, **_k: [],
        Tag=object,
    ),
)

stub = types.ModuleType("prompt_toolkit")
stub.PromptSession = object
completion = types.ModuleType("prompt_toolkit.completion")
completion.WordCompleter = object
history = types.ModuleType("prompt_toolkit.history")
history.InMemoryHistory = object
sys.modules.setdefault("prompt_toolkit", stub)
sys.modules.setdefault("prompt_toolkit.completion", completion)
sys.modules.setdefault("prompt_toolkit.history", history)

rich = types.ModuleType("rich")
console = types.ModuleType("rich.console")
console.Console = object
markdown = types.ModuleType("rich.markdown")
markdown.Markdown = object
panel = types.ModuleType("rich.panel")
panel.Panel = object
sys.modules.setdefault("rich", rich)
sys.modules.setdefault("rich.console", console)
sys.modules.setdefault("rich.markdown", markdown)
sys.modules.setdefault("rich.panel", panel)

sentence_transformers = types.ModuleType("sentence_transformers")

class _DummyST:
    def __init__(self, *a, **k):
        pass

    def encode(self, *a, **k):
        return []

sentence_transformers.SentenceTransformer = _DummyST
sys.modules.setdefault("sentence_transformers", sentence_transformers)

requests = types.ModuleType("requests")
requests.get = lambda *a, **k: None
class _DummySession:
    def get(self, *a, **k):
        return types.SimpleNamespace(json=lambda: {})

requests.Session = _DummySession
sys.modules.setdefault("requests", requests)

yaml = types.ModuleType("yaml")
yaml.safe_load = lambda *a, **k: {}
sys.modules.setdefault("yaml", yaml)

sys.path.append(str(Path(__file__).resolve().parents[2]))

from src.core.neyra_brain import Neyra, NeyraConfig
from src.iteration import KnowledgeGap


def test_slow_search_disables_parallel_and_reduces_iterations(monkeypatch):
    neyra = Neyra()
    neyra.iteration_controller.max_iterations = 3
    neyra.time_threshold = 0.01
    neyra.deep_searcher.parallel = True
    neyra.verification_system.memory = {}

    monkeypatch.setattr(
        neyra.gap_analyzer,
        "analyze",
        lambda draft: [KnowledgeGap(claim=draft, questions=[], confidence=0.0)],
    )
    def slow_search(*args, **kwargs):
        time.sleep(0.02)
        return []
    monkeypatch.setattr(neyra.deep_searcher, "search", slow_search)
    monkeypatch.setattr(neyra.response_enhancer, "enhance", lambda t, r, i: t)

    neyra.iterative_response("@Проверка:нет@ ___")

    assert neyra.deep_searcher.parallel is False
    assert neyra.iteration_controller.max_iterations == 1
    assert any("search_sources" in msg for msg in neyra.optimization_history)


def test_slow_post_processing_reduces_iterations(monkeypatch):
    neyra = Neyra(NeyraConfig(min_iterations=1))
    neyra.iteration_controller.max_iterations = 3
    neyra.time_threshold = 0.01
    neyra.verification_system.memory = {}

    monkeypatch.setattr(neyra.gap_analyzer, "analyze", lambda draft: [])
    monkeypatch.setattr(neyra.deep_searcher, "search", lambda *a, **k: [])
    monkeypatch.setattr(neyra.response_enhancer, "enhance", lambda t, r, i: t)

    def slow_post(text, processors, candidate_generator=None, candidate_selector=None):
        time.sleep(0.02)
        return text, []

    monkeypatch.setattr("src.core.neyra_brain.run_post_processors", slow_post)

    neyra.iterative_response("просто текст")

    assert neyra.iteration_controller.max_iterations == 1
    assert any("post_processing" in msg for msg in neyra.optimization_history)
