"""Predict future resource demand based on historical allocations.

The :class:`UsagePredictor` stores recent resource usage for arbitrary
components and applies simple exponential smoothing to forecast the next
expected demand.  The implementation intentionally avoids external
dependencies to keep unit tests lightweight.
"""

from __future__ import annotations

from collections import defaultdict, deque
from typing import Any, Deque, Dict


class UsagePredictor:
    """Track usage history and forecast the next demand value.

    Parameters
    ----------
    alpha:
        Smoothing factor for exponential smoothing. Values closer to ``1``
        weight recent observations more heavily.
    history_size:
        Maximum number of historical samples retained for each component.
    """

    def __init__(self, alpha: float = 0.5, history_size: int = 20) -> None:
        self.alpha = float(alpha)
        self._history: Dict[Any, Deque[float]] = defaultdict(
            lambda: deque(maxlen=history_size)
        )
        self._smoothed: Dict[Any, float] = {}

    # ------------------------------------------------------------------
    def record(self, component: Any, amount: float) -> None:
        """Record ``amount`` of resources used by ``component``."""

        hist = self._history[component]
        hist.append(float(amount))
        if component not in self._smoothed:
            self._smoothed[component] = float(amount)
        else:
            prev = self._smoothed[component]
            self._smoothed[component] = self.alpha * float(amount) + (1 - self.alpha) * prev

    # ------------------------------------------------------------------
    def predict(self, component: Any) -> float:
        """Return the forecasted demand for ``component``."""

        return self._smoothed.get(component, 0.0)


__all__ = ["UsagePredictor"]
