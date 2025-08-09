"""Base classes for visual language parsers.

This module defines the :class:`LanguageParser` abstract base class that
specifies the interface required for converting source code files into a
structure suitable for the visual programming mode.  Concrete parsers for
specific languages should subclass :class:`LanguageParser` and implement the
three core methods.
"""
from __future__ import annotations

from abc import ABC, abstractmethod
from pathlib import Path
from typing import Any, Iterable


class LanguageParser(ABC):
    """Abstract interface for language specific parsers.

    Subclasses are expected to parse a source file and then extract the
    visual graph representation consisting of *nodes* and *connections*.
    """

    @abstractmethod
    def parse_file(self, path: str | Path) -> Any:
        """Parse ``path`` and return an intermediate representation.

        Parameters
        ----------
        path:
            Path to the source file that should be processed.
        """

    @abstractmethod
    def extract_nodes(self, tree: Any) -> Iterable[Any]:
        """Return an iterable of nodes from ``tree``."""

    @abstractmethod
    def extract_connections(self, tree: Any) -> Iterable[Any]:
        """Return an iterable of connections from ``tree``."""
