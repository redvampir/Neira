import json
import sys
from dataclasses import dataclass
from pathlib import Path

sys.path.append(str(Path(__file__).resolve().parents[2]))

import types

sys.modules.setdefault(
    "neira_rust",
    types.SimpleNamespace(
        KnowledgeGraph=object,
        MemoryIndex=object,
        VerificationResult=object,
        verify_claim=lambda *a, **k: None,
        ping=lambda: "pong",
    ),
)
sys.modules.setdefault("requests", types.ModuleType("requests"))

from src.monitoring.iteration_logger import IterationLogger


@dataclass
class KnowledgeGap:
    claim: str
    questions: list
    confidence: float


def test_iteration_logger_writes_files(tmp_path):
    log_dir = tmp_path / "iterations"
    logger = IterationLogger(log_dir=log_dir, run_id="run_a")
    gaps = [KnowledgeGap(claim="claim", questions=[], confidence=0.5)]
    logger.log_iteration(1, "draft", gaps, sources=["src"], enhancements={"x": 1})
    log_file = log_dir / "run_a" / "iteration_1.json"
    assert log_file.exists()
    data = json.loads(log_file.read_text(encoding="utf-8"))
    assert data["iteration"] == 1
    assert data["draft"] == "draft"
    assert data["gaps"][0]["claim"] == "claim"


def test_iteration_logger_records_resource_metrics(tmp_path):
    log_dir = tmp_path / "iterations"
    logger = IterationLogger(log_dir=log_dir, run_id="run_metrics")
    gaps = [KnowledgeGap(claim="c", questions=[], confidence=0.5)]
    logger.log_iteration(
        1,
        "draft",
        gaps,
        sources=["src"],
        enhancements={"x": 1},
        resource_metrics={"cpu": 10},
    )
    log_file = log_dir / "run_metrics" / "iteration_1.json"
    data = json.loads(log_file.read_text(encoding="utf-8"))
    assert data["resource_metrics"]["cpu"] == 10


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
    assert json.loads(file1.read_text(encoding="utf-8"))["draft"] == "draft1"
    assert json.loads(file2.read_text(encoding="utf-8"))["draft"] == "draft2"
