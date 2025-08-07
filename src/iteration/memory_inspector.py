from __future__ import annotations

"""Utilities for inspecting stored reference memory."""

from dataclasses import dataclass
from typing import List, TYPE_CHECKING

if TYPE_CHECKING:  # pragma: no cover - for type hints only
    from .reference_memory import ReferenceMemory, ReferenceEntry


@dataclass
class SourceContribution:
    """Representation of a single source and its contribution."""

    summary: str
    path: str
    confidence: float


class MemoryInspector:
    """Inspect :class:`ReferenceMemory` and report linked sources."""

    def __init__(self, memory: "ReferenceMemory") -> None:
        self.memory = memory

    # ------------------------------------------------------------------
    def linked_sources(self) -> List[SourceContribution]:
        """Return a list describing all linked sources."""

        sources: List[SourceContribution] = []
        for entry in self.memory.internal_sources + self.memory.external_sources:
            sources.append(
                SourceContribution(
                    summary=entry.summary, path=entry.path, confidence=entry.confidence
                )
            )
        return sources

    # ------------------------------------------------------------------
    def generate_report(self) -> str:
        """Return a human readable report of linked sources."""

        sources = self.linked_sources()
        if not sources:
            return "No sources linked."
        lines = [
            f"- {src.summary}: {src.path} (confidence {src.confidence:.2f})"
            for src in sources
        ]
        return "\n".join(lines)


__all__ = ["MemoryInspector", "SourceContribution"]
