import sys
import types


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

from src.core.neyra_brain import Neyra
from src.iteration import KnowledgeGap


def _prepare(monkeypatch, attention: float, curiosity: float, mood: str) -> Neyra:
    neyra = Neyra()
    neyra.iteration_controller.max_iterations = 5
    neyra.iteration_controller.max_critical_spaces = 1
    neyra.personality.attention_to_detail = attention
    neyra.personality.curiosity_level = curiosity
    neyra.emotional_state = mood

    def fake_process(text: str) -> str:
        neyra.last_draft = text
        return text

    monkeypatch.setattr(neyra, "process_command", fake_process)
    monkeypatch.setattr(
        neyra.gap_analyzer,
        "analyze",
        lambda draft: [KnowledgeGap(claim=draft, questions=[], confidence=0.0)],
    )
    monkeypatch.setattr(neyra.deep_searcher, "search", lambda *a, **k: [])
    monkeypatch.setattr(
        neyra.response_enhancer,
        "enhance",
        lambda text, results, integration, self_correct=True: text,
    )
    return neyra


def _run_iterations(neyra: Neyra) -> int:
    neyra.iteration_controller._iterations = 0
    neyra.iterative_response("___")
    return neyra.iteration_controller._iterations


def test_personality_and_emotion_influence_iterations(monkeypatch):
    high = _prepare(monkeypatch, attention=1.0, curiosity=1.0, mood="взволнованная")
    low = _prepare(monkeypatch, attention=0.1, curiosity=0.1, mood="спокойная")

    high_iters = _run_iterations(high)
    low_iters = _run_iterations(low)

    assert high_iters > low_iters
    assert low_iters == 2
