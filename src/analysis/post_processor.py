from __future__ import annotations

from abc import ABC, abstractmethod
from typing import Dict, List, Tuple


class PostProcessor(ABC):
    """Interface for response post-processing steps."""

    @abstractmethod
    def process(self, text: str) -> Tuple[str, List[Dict[str, str]]]:
        """Process ``text`` and return modified text and list of corrections."""
        raise NotImplementedError
