from __future__ import annotations

"""Tools for logging quality and performance metrics."""

import json
import logging
import sys
import time
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Dict, List, MutableMapping


@dataclass
class MetricsMonitor:
    """Log metrics to a JSONL file and console."""

    log_file: Path = Path("logs/metrics.jsonl")
    logger: logging.Logger = field(default_factory=lambda: logging.getLogger("metrics"))
    thresholds: MutableMapping[str, Dict[str, float]] = field(default_factory=dict)
    time_series: MutableMapping[str, List[Dict[str, float]]] = field(default_factory=dict)
    resource_metrics: set[str] = field(default_factory=lambda: {"cpu", "memory"})

    def __post_init__(self) -> None:
        # Ensure log directory exists
        self.log_file.parent.mkdir(parents=True, exist_ok=True)

        # Configure logger for console output
        if not self.logger.handlers:
            handler = logging.StreamHandler(stream=sys.stdout)
            handler.setFormatter(logging.Formatter("%(message)s"))
            self.logger.addHandler(handler)
        self.logger.setLevel(logging.INFO)

    # ------------------------------------------------------------------
    def _write_jsonl(self, data: Dict[str, Any]) -> None:
        with self.log_file.open("a", encoding="utf-8") as f:
            f.write(json.dumps(data, ensure_ascii=False) + "\n")

    def _check_thresholds(self, metrics: Dict[str, Any]) -> None:
        for name, value in metrics.items():
            if not isinstance(value, (int, float)):
                continue
            threshold = self.thresholds.get(name, {})
            error_t = threshold.get("error")
            warning_t = threshold.get("warning")
            if error_t is not None and value >= error_t:
                self.logger.error(
                    f"{name}={value} exceeds error threshold {error_t}"
                )
            elif warning_t is not None and value >= warning_t:
                self.logger.warning(
                    f"{name}={value} exceeds warning threshold {warning_t}"
                )

    def _record_time_series(self, metrics: Dict[str, Any]) -> None:
        timestamp = time.time()
        for name in self.resource_metrics:
            if name in metrics and isinstance(metrics[name], (int, float)):
                self.time_series.setdefault(name, []).append(
                    {"timestamp": timestamp, "value": float(metrics[name])}
                )

    def log_quality_metrics(self, **metrics: Any) -> None:
        """Persist quality-related ``metrics``."""

        entry = {"type": "quality", **metrics}
        self._write_jsonl(entry)
        self.logger.info(json.dumps(entry, ensure_ascii=False))
        self._check_thresholds(metrics)
        self._record_time_series(metrics)

    def log_performance_metrics(self, **metrics: Any) -> None:
        """Persist performance-related ``metrics``."""

        entry = {"type": "performance", **metrics}
        self._write_jsonl(entry)
        self.logger.info(json.dumps(entry, ensure_ascii=False))
        self._check_thresholds(metrics)
        self._record_time_series(metrics)


__all__ = ["MetricsMonitor"]
