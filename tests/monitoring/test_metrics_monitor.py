import json
import sys
from pathlib import Path

sys.path.append(str(Path(__file__).resolve().parents[2]))

from src.monitoring.metrics_monitor import MetricsMonitor
from src.iteration.iterative_generator import IterativeGenerator
from src.iteration.gap_analyzer import KnowledgeGap
from src.iteration.iteration_controller import IterationController


class DummyDraftGenerator:
    def generate_draft(self, query, context):
        return "draft ___"


class DummyGapAnalyzer:
    def analyze(self, draft):
        if "___" in draft:
            return [KnowledgeGap(claim="info", questions=[], confidence=0.0)]
        return []


class DummyDeepSearcher:
    def __init__(self):
        self.queries = []

    def search(self, query, user_id=None, limit=None):
        self.queries.append(query)
        return [{"content": "resolved"}]


class DummyResponseEnhancer:
    def enhance(self, draft, search_results, integration, self_correct=True):
        return draft.replace("___", search_results[0]["content"])


def test_metrics_monitor_logs(tmp_path, capsys):
    log_file = tmp_path / "metrics.jsonl"
    monitor = MetricsMonitor(log_file=log_file)
    monitor.log_performance_metrics(duration=1.23, num_sources=2)
    monitor.log_quality_metrics(final_quality=0.5)

    captured = capsys.readouterr().out
    assert "performance" in captured
    assert "quality" in captured

    lines = log_file.read_text().strip().splitlines()
    assert len(lines) == 2
    perf = json.loads(lines[0])
    qual = json.loads(lines[1])
    assert perf["type"] == "performance"
    assert perf["duration"] == 1.23
    assert qual["type"] == "quality"
    assert qual["final_quality"] == 0.5


def test_iterative_generator_logs_metrics(tmp_path):
    log_file = tmp_path / "metrics.jsonl"
    monitor = MetricsMonitor(log_file=log_file)
    controller = IterationController(max_iterations=3, max_critical_spaces=0)
    generator = IterativeGenerator(
        draft_generator=DummyDraftGenerator(),
        gap_analyzer=DummyGapAnalyzer(),
        deep_searcher=DummyDeepSearcher(),
        response_enhancer=DummyResponseEnhancer(),
        iteration_controller=controller,
        metrics_monitor=monitor,
    )

    generator.generate_response("question", {})

    lines = log_file.read_text().strip().splitlines()
    assert len(lines) == 2
    perf = json.loads(lines[0])
    qual = json.loads(lines[1])
    assert perf["type"] == "performance"
    assert "duration" in perf
    assert "num_sources" in perf
    assert qual["type"] == "quality"
    assert "final_quality" in qual
