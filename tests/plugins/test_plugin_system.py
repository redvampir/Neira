from pathlib import Path
import sys

# Ensure project root is importable
sys.path.append(str(Path(__file__).resolve().parents[2]))

from src.plugins import PluginManager
from src.iteration.iterative_generator import IterativeGenerator


class DummyDraftGenerator:
    def generate_draft(self, query, context):
        return "draft"


class DummyGapAnalyzer:
    def analyze(self, draft):
        return []


class DummyIterationController:
    def __init__(self):
        self._called = 0

    def should_iterate(self, draft):
        self._called += 1
        return self._called == 1

    def reset(self):
        self._called = 0

    def assess_quality(self, draft):
        return 1


class DummyResponseEnhancer:
    def enhance(self, draft, search_results, integration_type):
        return draft


def test_plugin_manager_loads_and_runs_hooks():
    plugin_dir = Path(__file__).resolve().parents[2] / "plugins"
    manager = PluginManager(plugin_dir)
    assert manager.plugins, "plugin was not loaded"
    plugin = manager.plugins[0]

    manager.on_draft("d", {})
    manager.on_gap_analysis("d", [])
    manager.on_finalize("r")

    assert [e[0] for e in plugin.events] == ["draft", "gap", "final"]


def test_iterative_generator_triggers_plugin_hooks():
    plugin_dir = Path(__file__).resolve().parents[2] / "plugins"
    manager = PluginManager(plugin_dir)
    plugin = manager.plugins[0]

    generator = IterativeGenerator(
        draft_generator=DummyDraftGenerator(),
        gap_analyzer=DummyGapAnalyzer(),
        response_enhancer=DummyResponseEnhancer(),
        iteration_controller=DummyIterationController(),
        deep_searcher=None,
        plugin_manager=manager,
    )

    generator.generate_response("q", {})
    events = [e[0] for e in plugin.events]
    assert events.count("draft") == 1
    assert events.count("gap") == 1
    assert events.count("final") == 1
