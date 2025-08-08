import json
import sys
from pathlib import Path

sys.path.append(str(Path(__file__).resolve().parents[2]))

from src.monitoring.iteration_logger import IterationLogger
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
    def search(self, query, user_id=None, limit=None):
        return [
            {"content": "resolved", "reference": "ref", "priority": 0.5}
        ]


class DummyResponseEnhancer:
    def enhance(self, draft, search_results, integration, self_correct=True):
        return draft.replace("___", search_results[0]["content"])


def test_iteration_logger_writes_files(tmp_path):
    log_dir = tmp_path / "iterations"
    logger = IterationLogger(log_dir=log_dir, run_id="run_a")
    gaps = [KnowledgeGap(claim="claim", questions=[], confidence=0.5)]
    logger.log_iteration(1, "draft", gaps, sources=["src"], enhancements={"x": 1})
    log_file = log_dir / "run_a" / "iteration_1.json"
    assert log_file.exists()
    data = json.loads(log_file.read_text())
    assert data["iteration"] == 1
    assert data["draft"] == "draft"
    assert data["gaps"][0]["claim"] == "claim"


def test_iterative_generator_logs_iterations(tmp_path):
    log_dir = tmp_path / "iterations"
    logger = IterationLogger(log_dir=log_dir, run_id="run_b")
    controller = IterationController(max_iterations=3, max_critical_spaces=0)
    generator = IterativeGenerator(
        draft_generator=DummyDraftGenerator(),
        gap_analyzer=DummyGapAnalyzer(),
        deep_searcher=DummyDeepSearcher(),
        response_enhancer=DummyResponseEnhancer(),
        iteration_controller=controller,
        iteration_logger=logger,
    )

    generator.generate_response("question", {})

    log_file = log_dir / "run_b" / "iteration_1.json"
    assert log_file.exists()


def test_logger_creates_separate_files_for_same_iter_idx(tmp_path):
    log_dir = tmp_path / "iterations"
    gaps = [KnowledgeGap(claim="claim", questions=[], confidence=0.5)]
    logger1 = IterationLogger(log_dir=log_dir, run_id="run1")
    logger2 = IterationLogger(log_dir=log_dir, run_id="run2")
    logger1.log_iteration(1, "draft1", gaps, sources=["src1"], enhancements={"x": 1})
    logger2.log_iteration(1, "draft2", gaps, sources=["src2"], enhancements={"x": 2})
    file1 = log_dir / "run1" / "iteration_1.json"
    file2 = log_dir / "run2" / "iteration_1.json"
    assert file1.exists()
    assert file2.exists()
    assert json.loads(file1.read_text())["draft"] == "draft1"
    assert json.loads(file2.read_text())["draft"] == "draft2"
