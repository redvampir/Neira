from __future__ import annotations

"""Analyse metric trends and issue early warnings."""

from dataclasses import dataclass, field
from typing import Dict
import logging
import statistics

from src.core.config import get_logger
from .metrics_monitor import MetricsMonitor


@dataclass
class PredictiveDiagnostics:
    """Simple trend analyser using moving averages."""

    monitor: MetricsMonitor
    window: int = 3
    threshold: float = 0.2  # 20% increase triggers alert
    logger: logging.Logger = field(default_factory=lambda: get_logger("diagnostics"))

    def analyse(self) -> Dict[str, str]:
        """Return alerts for metrics with rising trends."""

        alerts: Dict[str, str] = {}
        for name, points in self.monitor.time_series.items():
            if len(points) <= self.window:
                continue
            values = [p["value"] for p in points]
            recent = values[-self.window :]
            previous = values[-self.window - 1 : -1]
            if not previous:
                continue
            prev_avg = statistics.fmean(previous)
            recent_avg = statistics.fmean(recent)
            if prev_avg == 0:
                continue
            increase = (recent_avg - prev_avg) / prev_avg
            if increase >= self.threshold:
                msg = (
                    f"Rising trend detected for {name}"
                    f" ({increase:.0%} increase)"
                )
                self.logger.warning(msg)
                alerts[name] = msg
        return alerts


__all__ = ["PredictiveDiagnostics"]
