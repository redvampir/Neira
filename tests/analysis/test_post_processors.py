from typing import Dict, List, Tuple
import sys
import types

# Stub modules that may be missing in test environment
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
from src.core.neyra_config import NeyraConfig
from src.analysis.post_processor import PostProcessor


class UpperCaseProcessor(PostProcessor):
    def process(self, text: str) -> Tuple[str, List[Dict[str, str]]]:
        return text.upper(), [{"rule": "upper"}]


class ExclaimProcessor(PostProcessor):
    def process(self, text: str) -> Tuple[str, List[Dict[str, str]]]:
        return text + "!", [{"rule": "exclaim"}]


def test_multiple_post_processors(monkeypatch):
    config = NeyraConfig(post_processors=[])
    neyra = Neyra(config)
    neyra.post_processors = [UpperCaseProcessor(), ExclaimProcessor()]
    monkeypatch.setattr(neyra, "process_command", lambda q: q)
    monkeypatch.setattr(neyra.gap_analyzer, "analyze", lambda draft: [])
    monkeypatch.setattr(neyra.deep_searcher, "search", lambda *a, **k: [])
    monkeypatch.setattr(
        neyra.response_enhancer,
        "enhance",
        lambda text, results, integration, self_correct=True: text,
    )
    monkeypatch.setattr(neyra.iteration_controller, "should_iterate", lambda text: False)
    result, corrections = neyra.iterative_response("hi")
    assert result == "HI!"
    assert [c["rule"] for c in corrections] == ["upper", "exclaim"]
