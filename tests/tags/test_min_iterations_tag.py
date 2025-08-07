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
sys.modules.setdefault("rich", types.SimpleNamespace(console=rich_console, markdown=rich_markdown, panel=rich_panel))

from src.core.neyra_brain import Neyra


def test_min_iterations_tag_applies(monkeypatch):
    neyra = Neyra()
    monkeypatch.setattr(neyra.draft_generator, "generate_draft", lambda text, memory: text)
    monkeypatch.setattr(neyra.gap_analyzer, "analyze", lambda draft: [])
    monkeypatch.setattr(neyra.deep_searcher, "search", lambda *a, **k: [])
    monkeypatch.setattr(
        neyra.response_enhancer,
        "enhance",
        lambda text, results, integration, self_correct=True: text,
    )

    iterations = []

    def fake_update(stage, iteration=None):
        if stage == "iteration":
            iterations.append(iteration)

    monkeypatch.setattr("src.core.neyra_brain.update_progress", fake_update)

    neyra.iterative_response("@Минимум:3@")
    assert neyra.config.min_iterations == 3
    assert neyra.iteration_controller.min_iterations == 3
    assert iterations == [1, 2, 3]
