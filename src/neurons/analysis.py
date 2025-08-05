"""Analysis neuron."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Dict

from .base import Neuron


@dataclass
class AnalysisNeuron(Neuron):
    """Perform simple analysis on text input."""

    type: str = "analysis"

    # ------------------------------------------------------------------
    def process(self, text: str) -> Dict[str, Any]:
        """Return basic statistics about the given text."""

        self.activate()
        return {
            "length": len(text),
            "words": len(text.split()),
        }
