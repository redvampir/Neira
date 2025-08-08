"""Plan iteration count based on available system resources."""

from __future__ import annotations

from typing import Mapping, Any

from .low_resource_optimizer import LowResourceOptimizer
from .resource_manager import ResourceManager, IterationConfig


class ResourceAwareIterator:
    """Determine iteration plan given resource constraints.

    Parameters
    ----------
    resources:
        Mapping describing currently available resources. Typical keys are
        ``"gpu"`` for GPU memory, ``"cpu"`` for CPU memory and ``"time"`` for
        the time budget in seconds.
    """

    def __init__(
        self,
        resources: Mapping[str, float] | None = None,
        resource_manager: ResourceManager | None = None,
    ) -> None:
        """Create iterator from explicit resources or a ``ResourceManager``.

        When ``resources`` are provided the behaviour mirrors the legacy
        implementation and :class:`LowResourceOptimizer` is used to create a
        configuration.  Otherwise the supplied ``resource_manager`` (or a new
        instance) determines both available resources and the
        :class:`IterationConfig`.
        """

        if resources is not None:
            self.resources = dict(resources)
            optimizer = LowResourceOptimizer(self.resources)
            suggestion = optimizer.suggest()
            self.config = IterationConfig(
                max_iterations=4,
                parallel=suggestion.get("parallel", True),
                cache=suggestion.get("cache", {}),
            )
        else:
            self.resource_manager = resource_manager or ResourceManager()
            self.config = self.resource_manager.get_config()
            self.resources: dict[str, Any] = {
                "gpu": self.resource_manager.gpu_memory,
                "cpu": self.resource_manager.cpu_cores,
            }

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
