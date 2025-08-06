"""Track information sources and their trustworthiness for Neyra."""

from __future__ import annotations

from dataclasses import dataclass
import logging
from typing import List


@dataclass
class SourceEntry:
    """Information about a single source."""
    info: str
    source: str
    confidence: float


class SourceTracker:
    """Collects citations and ensures only trusted sources are used."""

    def __init__(self, reliability_threshold: float = 0.5) -> None:
        self.reliability_threshold = reliability_threshold
        self.entries: List[SourceEntry] = []
        self.logger = logging.getLogger(__name__)

    def add(self, info: str, source: str, confidence: float) -> None:
        """Register a new source for *info* with *confidence* rating.

        Raises
        ------
        ValueError
            If *confidence* is below the reliability threshold.
        """
        if confidence < self.reliability_threshold:
            self.logger.warning("Источник заблокирован: %s (%.2f)", source, confidence)
            raise ValueError("Ненадёжный источник заблокирован")
        entry = SourceEntry(info=info, source=source, confidence=confidence)
        self.entries.append(entry)
        self.logger.info("Источник принят: %s (%.2f)", source, confidence)

    def get_sources(self) -> List[str]:
        """Return list of all source links."""
        return [entry.source for entry in self.entries]

    def format_citations(self) -> str:
        """Return formatted citations suitable for display."""
        return "\n".join(
            f"{idx + 1}. {entry.source}" for idx, entry in enumerate(self.entries)
        )
