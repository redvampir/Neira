from src.core.neyra_brain import Neyra
from src.iteration import DeepSearcher, KnowledgeGap, TokenBudgetManager
from src.iteration.plugin_registry import register_search_plugin, clear_search_plugins


class DummyPlugin:
    def __init__(self):
        self.limits: list[int] = []

    def search(self, query: str, limit: int = 5):  # pragma: no cover - simple stub
        self.limits.append(limit)
        return [
            {
                "source": "dummy",
                "reference": "",
                "content": "x" * 100,
                "priority": 0.1,
            }
            for _ in range(limit)
        ]


def test_allocation_basic():
    mgr = TokenBudgetManager(
        total_tokens=100,
        draft_ratio=0.4,
        search_ratio=0.3,
        refine_ratio=0.3,
        per_result_tokens=10,
    )
    assert mgr.draft_tokens == 40
    assert mgr.refine_tokens == 30
    assert mgr.tokens_per_search_query() == 30
    assert mgr.search_limit() == 3


def test_deep_searcher_respects_budget():
    clear_search_plugins()
    plugin = DummyPlugin()
    register_search_plugin(plugin)
    mgr = TokenBudgetManager(
        total_tokens=100,
        draft_ratio=0.4,
        search_ratio=0.4,
        refine_ratio=0.2,
        per_result_tokens=10,
    )
    searcher = DeepSearcher(use_default_plugins=False, token_budget_manager=mgr)
    results = searcher.search("hi")
    assert plugin.limits == [mgr.search_limit()]
    max_len = mgr.tokens_per_search_query()
    assert all(len(r["content"]) <= max_len for r in results)
    clear_search_plugins()


def test_iterative_response_uses_budget(monkeypatch):
    neyra = Neyra()
    neyra.llm_max_tokens = 100

    tokens_used = []

    def fake_process(query):
        tokens_used.append(neyra.llm_max_tokens)
        neyra.last_draft = "draft"
        return "draft"

    monkeypatch.setattr(neyra, "process_command", fake_process)

    monkeypatch.setattr(
        neyra.gap_analyzer,
        "analyze",
        lambda draft: [KnowledgeGap(claim="gap", questions=[], confidence=0.0)],
    )

    limits: list[int | None] = []

    def fake_search(query, user_id=None, limit=None):
        limits.append(limit)
        return []

    monkeypatch.setattr(neyra.deep_searcher, "search", fake_search)
    monkeypatch.setattr(neyra.iteration_controller, "should_iterate", lambda text: False)

    neyra.iterative_response("q")
    mgr = TokenBudgetManager(100)
    assert tokens_used[0] == mgr.draft_tokens
    assert limits[0] == mgr.search_limit(1)
