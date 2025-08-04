"""Base class for all file handlers used by Neira."""

from __future__ import annotations

from abc import ABC, abstractmethod
from typing import Any, Dict


class BaseFileHandler(ABC):
    """Abstract base class for reading and writing files.

    Concrete handlers implement support for specific file formats.  Methods
    generally return dictionaries with the following keys::

        {
            'title': str,      # filename without extension
            'content': str,    # textual content of the file
            'metadata': dict,  # any file metadata extracted
            'structure': dict, # optional structural information
            'encoding': str,   # text encoding used when reading/writing
            'format': str      # file format identifier
        }
    """

    @abstractmethod
    def can_handle(self, file_path: str) -> bool:
        """Return ``True`` if the handler supports ``file_path``."""

    @abstractmethod
    def read_file(self, file_path: str) -> Dict[str, Any]:
        """Read ``file_path`` and return structured data."""

    @abstractmethod
    def save_file(self, file_path: str, data: Dict[str, Any]) -> bool:
        """Persist ``data`` to ``file_path``."""
