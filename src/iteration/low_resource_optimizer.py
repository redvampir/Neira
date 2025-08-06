"""Configuration tuning for low-resource environments."""

from __future__ import annotations

from typing import Any, Mapping


class LowResourceOptimizer:
    """Suggest configuration options for limited resources.

    The optimizer examines available memory and decides on cache sizes and
    whether parallelism should be enabled.  It is intentionally simple and
    relies only on rough thresholds which are sufficient for unit tests.
    """

    def __init__(self, resources: Mapping[str, float]) -> None:
        self.resources = dict(resources)

    # ------------------------------------------------------------------
    def suggest(self) -> dict[str, Any]:
        """Return a configuration dictionary tuned to the resources."""

        memory = float(self.resources.get("cpu", 0))
        if memory < 4:
            cache = {"hot_limit": 4, "warm_limit": 16}
            parallel = False
        elif memory < 8:
            cache = {"hot_limit": 8, "warm_limit": 32}
            parallel = False
        else:
            cache = {"hot_limit": 32, "warm_limit": 128}
            parallel = True
        return {"cache": cache, "parallel": parallel}


__all__ = ["LowResourceOptimizer"]

