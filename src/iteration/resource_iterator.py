"""Plan iteration count based on available system resources."""

from __future__ import annotations

from typing import Mapping

from .low_resource_optimizer import LowResourceOptimizer


class ResourceAwareIterator:
    """Determine iteration plan given resource constraints.

    Parameters
    ----------
    resources:
        Mapping describing currently available resources. Typical keys are
        ``"gpu"`` for GPU memory, ``"cpu"`` for CPU memory and ``"time"`` for
        the time budget in seconds.
    """

    def __init__(self, resources: Mapping[str, float]) -> None:
        self.resources = dict(resources)
        self.optimizer = LowResourceOptimizer(self.resources)
        self.config = self.optimizer.suggest()

    # ------------------------------------------------------------------
    def plan(self, per_iteration: Mapping[str, float]) -> list[int]:
        """Return iteration indices that fit within ``per_iteration`` usage.

        Parameters
        ----------
        per_iteration:
            Mapping of resource consumption for a single iteration.

        Returns
        -------
        list[int]
            List of iteration numbers starting from ``0``. Its length equals
            the maximum number of iterations possible under the given
            constraints. When no resources are specified the result is an empty
            list.
        """

        capacities: list[int] = []
        for name, usage in per_iteration.items():
            if usage <= 0:
                continue
            available = self.resources.get(name, 0)
            capacities.append(int(available // usage))

        count = min(capacities) if capacities else 0
        return list(range(count))


__all__ = ["ResourceAwareIterator"]
