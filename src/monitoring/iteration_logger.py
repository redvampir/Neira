from __future__ import annotations

"""Utilities for logging each iteration of the response generation process."""
from dataclasses import dataclass, asdict, is_dataclass, field
from datetime import datetime
import json
from pathlib import Path
from typing import Any


def _serialize(obj: Any) -> Any:
    """Recursively convert dataclasses and other objects to JSON-serialisable data."""
    if is_dataclass(obj):
        return asdict(obj)
    if isinstance(obj, list):
        return [_serialize(i) for i in obj]
    if isinstance(obj, dict):
        return {k: _serialize(v) for k, v in obj.items()}
    return obj


@dataclass
class IterationLogger:
    """Persist information about each iteration to separate JSON files."""

    log_dir: Path = Path("logs/iterations")
    run_id: str = field(
        default_factory=lambda: datetime.utcnow().strftime("%Y%m%dT%H%M%S")
    )

    def log_iteration(
        self,
        iter_idx: int,
        draft: str,
        gaps: Any,
        sources: Any,
        enhancements: Any,
        resource_metrics: Any | None = None,
    ) -> None:
        """Record details for a single iteration.

        Parameters
        ----------
        iter_idx:
            Index of the current iteration (starting from 1).
        draft:
            The draft text after enhancement.
        gaps:
            Detected knowledge gaps before enhancement.
        sources:
            Sources retrieved to address the gaps.
        enhancements:
            Result returned by the response enhancer.
        resource_metrics:
            Optional resource usage metrics for this iteration.
        """

        run_dir = self.log_dir / self.run_id
        run_dir.mkdir(parents=True, exist_ok=True)
        entry = {
            "iteration": iter_idx,
            "draft": draft,
            "gaps": _serialize(gaps),
            "sources": _serialize(sources),
            "enhancements": _serialize(enhancements),
        }
        if resource_metrics is not None:
            entry["resource_metrics"] = _serialize(resource_metrics)
        file_path = run_dir / f"iteration_{iter_idx}.json"
        with file_path.open("w", encoding="utf-8") as fh:
            json.dump(entry, fh, ensure_ascii=False, indent=2)


__all__ = ["IterationLogger"]
