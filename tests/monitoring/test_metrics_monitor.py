import json
import sys
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

from src.monitoring.metrics_monitor import MetricsMonitor
from src.monitoring.predictive_diagnostics import PredictiveDiagnostics


class SimpleGenerator:
    def __init__(self, monitor: MetricsMonitor):
        self.monitor = monitor

    def run(self) -> None:
        self.monitor.log_performance_metrics(cpu=10, memory=20)
        self.monitor.log_performance_metrics(duration=1.0, num_sources=0)
        self.monitor.log_quality_metrics(final_quality=0.5)


def test_metrics_monitor_logs(tmp_path, capsys):
    log_file = tmp_path / "metrics.jsonl"
    monitor = MetricsMonitor(log_file=log_file)
    monitor.log_performance_metrics(duration=1.23, num_sources=2)
    monitor.log_quality_metrics(final_quality=0.5)

    captured = capsys.readouterr().out
    assert "performance" in captured
    assert "quality" in captured

    lines = log_file.read_text(encoding="utf-8").strip().splitlines()
    assert len(lines) == 2
    perf = json.loads(lines[0])
    qual = json.loads(lines[1])
    assert perf["type"] == "performance"
    assert perf["duration"] == 1.23
    assert qual["type"] == "quality"
    assert qual["final_quality"] == 0.5


def test_monitor_records_multiple_entries(tmp_path):
    log_file = tmp_path / "metrics.jsonl"
    monitor = MetricsMonitor(log_file=log_file)
    SimpleGenerator(monitor).run()

    lines = log_file.read_text(encoding="utf-8").strip().splitlines()
    assert len(lines) == 3
    res = json.loads(lines[0])
    perf = json.loads(lines[1])
    qual = json.loads(lines[2])
    assert "cpu" in res and "memory" in res
    assert "duration" in perf
    assert "final_quality" in qual


def test_metrics_monitor_thresholds(tmp_path, caplog):
    monitor = MetricsMonitor(thresholds={"cpu": {"warning": 50}})
    with caplog.at_level("WARNING"):
        monitor.log_performance_metrics(cpu=60)
    assert "warning threshold" in caplog.text
    assert "cpu" in monitor.time_series


def test_predictive_diagnostics_warns_on_trend(tmp_path):
    monitor = MetricsMonitor()
    for value in [10, 20, 40, 80]:
        monitor.log_performance_metrics(cpu=value)
    diag = PredictiveDiagnostics(monitor, window=2, threshold=0.5)
    alerts = diag.analyse()
    assert "cpu" in alerts
