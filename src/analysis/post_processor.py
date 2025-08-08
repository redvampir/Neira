from __future__ import annotations

from abc import ABC, abstractmethod
from typing import Dict, List, Tuple

from .candidate_generator import CandidateGenerator
from .candidate_selector import CandidateSelector


class PostProcessor(ABC):
    """Interface for response post-processing steps."""

    @abstractmethod
    def process(self, text: str) -> Tuple[str, List[Dict[str, str]]]:
        """Process ``text`` and return modified text and list of corrections."""
        raise NotImplementedError


def run_post_processors(
    text: str,
    processors: List[PostProcessor],
    candidate_generator: CandidateGenerator | None = None,
    candidate_selector: CandidateSelector | None = None,
) -> Tuple[str, List[Dict[str, str]]]:
    """Run ``processors`` on ``text`` and aggregate corrections.

    The function optionally generates several candidate responses and selects
    the most suitable one before applying regular post-processors.  This keeps
    the pipeline flexible while maintaining backward compatibility for cases
    where candidate generation is not required.
    """

    corrections: List[Dict[str, str]] = []

    if candidate_generator and candidate_selector:
        candidates = candidate_generator.generate_candidates(text, text)
        text = candidate_selector.select_best(candidates)

    for processor in processors:
        text, corr = processor.process(text)
        if corr:
            corrections.extend(corr)

    return text, corrections
