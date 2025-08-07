import sys
import types

prompt_toolkit = types.SimpleNamespace(
    PromptSession=lambda *a, **k: None,
    completion=types.SimpleNamespace(WordCompleter=lambda *a, **k: None),
    history=types.SimpleNamespace(InMemoryHistory=lambda *a, **k: None),
)
sys.modules.setdefault("prompt_toolkit", prompt_toolkit)
sys.modules.setdefault("prompt_toolkit.completion", prompt_toolkit.completion)
sys.modules.setdefault("prompt_toolkit.history", prompt_toolkit.history)

rich_console = types.SimpleNamespace(Console=lambda *a, **k: None)
rich_markdown = types.SimpleNamespace(Markdown=lambda *a, **k: None)
rich_panel = types.SimpleNamespace(Panel=lambda *a, **k: None)
sys.modules.setdefault("rich.console", rich_console)
sys.modules.setdefault("rich.markdown", rich_markdown)
sys.modules.setdefault("rich.panel", rich_panel)
sys.modules.setdefault(
    "rich",
    types.SimpleNamespace(console=rich_console, markdown=rich_markdown, panel=rich_panel),
)

from src.core.neyra_brain import Neyra


def test_min_iterations_with_grammar(monkeypatch):
    neyra = Neyra()
    monkeypatch.setattr(neyra, "process_command", lambda q: "превет мир")
    monkeypatch.setattr(neyra.gap_analyzer, "analyze", lambda draft: [])
    monkeypatch.setattr(neyra.deep_searcher, "search", lambda *a, **k: [])
    monkeypatch.setattr(
        neyra.response_enhancer,
        "enhance",
        lambda text, results, integration, self_correct=True: text,
    )

    iterations: list[int] = []
    metrics: list[tuple[int, str, str]] = []

    def fake_update(stage, iteration=None):
        if stage == "iteration":
            iterations.append(iteration)

    def fake_metrics(iteration, prev, current):
        metrics.append((iteration, prev, current))

    monkeypatch.setattr("src.core.neyra_brain.update_progress", fake_update)
    monkeypatch.setattr("src.core.neyra_brain.log_metrics", fake_metrics)

    result, corrections = neyra.iterative_response("query")

    assert iterations == [1, 2]
    assert metrics[0] == (1, "превет мир", "превет мир")
    assert metrics[1] == (2, "превет мир", "превет мир")
    assert result == "привет мир"
    assert corrections  # non-empty list


def test_skip_check_tag_disables_grammar(monkeypatch):
    neyra = Neyra()
    monkeypatch.setattr(neyra, "process_command", lambda q: "превет мир")
    monkeypatch.setattr(neyra.gap_analyzer, "analyze", lambda draft: [])
    monkeypatch.setattr(neyra.deep_searcher, "search", lambda *a, **k: [])
    monkeypatch.setattr(
        neyra.response_enhancer,
        "enhance",
        lambda text, results, integration, self_correct=True: text,
    )
    monkeypatch.setattr(
        neyra.iteration_controller, "should_iterate", lambda text: False
    )

    called: list[str] = []

    class DummyProcessor:
        def process(self, text):
            called.append(text)
            return text, []

    neyra.post_processors = [DummyProcessor()]

    iterations: list[int] = []

    def fake_update(stage, iteration=None):
        if stage == "iteration":
            iterations.append(iteration)

    monkeypatch.setattr("src.core.neyra_brain.update_progress", fake_update)

    result, corrections = neyra.iterative_response("@Проверка:нет@ query")

    assert called == []
    assert iterations == [1]
    assert neyra.iteration_controller.min_iterations == 1
    assert neyra.iteration_controller._iterations == 1
    assert result == "превет мир"
    assert corrections == []
