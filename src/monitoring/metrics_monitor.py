from __future__ import annotations

"""Tools for logging quality and performance metrics."""

import json
import logging
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Dict


@dataclass
class MetricsMonitor:
    """Log metrics to a JSONL file and console."""

    log_file: Path = Path("logs/metrics.jsonl")
    logger: logging.Logger = field(default_factory=lambda: logging.getLogger("metrics"))

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

    def log_quality_metrics(self, **metrics: Any) -> None:
        """Persist quality-related ``metrics``."""

        entry = {"type": "quality", **metrics}
        self._write_jsonl(entry)
        self.logger.info(json.dumps(entry, ensure_ascii=False))

    def log_performance_metrics(self, **metrics: Any) -> None:
        """Persist performance-related ``metrics``."""

        entry = {"type": "performance", **metrics}
        self._write_jsonl(entry)
        self.logger.info(json.dumps(entry, ensure_ascii=False))


__all__ = ["MetricsMonitor"]
