from __future__ import annotations

"""Simple planner producing THINK/ACT reasoning steps."""

from dataclasses import dataclass
from typing import List


@dataclass
class ReasoningStep:
    """One step in a reasoning chain.

    ``marker`` designates the kind of step (e.g. ``"THINK"`` or ``"ACT"``).
    ``content`` holds the textual instruction or query.
    ``source`` optionally specifies where an ``ACT`` step should retrieve
    information from (``"memory"`` or ``"rag"``).
    """

    marker: str
    content: str
    source: str | None = None


class ReasoningPlanner:
    """Generate a basic reasoning plan for a given query."""

    def plan(self, query: str) -> List[ReasoningStep]:
        """Return a list of steps tagged with ``THINK``/``ACT`` markers.

        The planner first reflects on the question, then schedules lookups in
        memory and in the retrieval layer and finally signals that the answer
        should be composed.
        """

        return [
            ReasoningStep("THINK", f"Analyse the request: {query}"),
            ReasoningStep("ACT", query, source="memory"),
            ReasoningStep("ACT", query, source="rag"),
            ReasoningStep("THINK", "Compose final response"),
        ]


__all__ = ["ReasoningPlanner", "ReasoningStep"]
