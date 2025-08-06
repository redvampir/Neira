from __future__ import annotations

"""Iteration strategy presets for the :class:`IterationController`.

The :class:`AdaptiveIterationManager` encapsulates a small collection of
predefined iteration strategies.  Each strategy specifies limits for the
:class:`~src.iteration.iteration_controller.IterationController` such as the
maximum number of refinement loops and how many critical placeholders are
allowed to remain in a response.  The presets are intentionally lightweight –
no external configuration or learning is required – but they provide a
convenient interface for choosing sensible defaults depending on the desired
trade‑off between speed and thoroughness.

The available presets are:

``quick``
    Perform at most one additional iteration.  Suitable for lightweight
    interactions where speed is preferred over quality.
``standard``
    A balanced configuration used as the default throughout the project.
``thorough``
    Allow more refinement cycles for detailed answers.
``research``
    Aggressive search intended for exploratory or uncertain topics.  It
    tolerates one unresolved placeholder to keep the loop going a little
    longer.
"""

from dataclasses import dataclass


@dataclass(frozen=True)
class IterationStrategy:
    """Configuration for an iteration strategy."""

    max_iterations: int
    max_critical_spaces: int


class AdaptiveIterationManager:
    """Manage iteration strategies with simple presets."""

    #: Mapping of preset name to :class:`IterationStrategy`.
    PRESETS: dict[str, IterationStrategy] = {
        "quick": IterationStrategy(max_iterations=1, max_critical_spaces=0),
        "standard": IterationStrategy(max_iterations=3, max_critical_spaces=0),
        "thorough": IterationStrategy(max_iterations=5, max_critical_spaces=0),
        "research": IterationStrategy(max_iterations=8, max_critical_spaces=1),
    }

    def __init__(self, preset: str = "standard") -> None:
        self.set_preset(preset)

    # ------------------------------------------------------------------
    def set_preset(self, preset: str) -> None:
        """Activate ``preset`` as the current iteration strategy.

        Parameters
        ----------
        preset:
            One of :data:`PRESETS`.  A :class:`ValueError` is raised for unknown
            names.
        """

        try:
            strategy = self.PRESETS[preset]
        except KeyError as exc:  # pragma: no cover - defensive programming
            raise ValueError(f"Unknown iteration preset: {preset}") from exc
        self.preset = preset
        self.max_iterations = strategy.max_iterations
        self.max_critical_spaces = strategy.max_critical_spaces

    # ------------------------------------------------------------------
    def configure(self, controller: "IterationController") -> None:
        """Apply the current strategy to ``controller``.

        The method simply copies the ``max_iterations`` and
        ``max_critical_spaces`` values to the provided
        :class:`IterationController` instance.
        """

        controller.max_iterations = self.max_iterations
        controller.max_critical_spaces = self.max_critical_spaces


__all__ = ["AdaptiveIterationManager", "IterationStrategy"]
