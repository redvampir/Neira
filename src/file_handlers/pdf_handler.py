"""Placeholder handler for PDF documents."""

from __future__ import annotations

from pathlib import Path
from typing import Any, Dict

from .base_handler import BaseFileHandler


class PDFHandler(BaseFileHandler):
    """Handle ``.pdf`` files."""

    _extensions = {".pdf"}

    def can_handle(self, file_path: str) -> bool:  # noqa: D401
        """See base class."""
        return Path(file_path).suffix.lower() in self._extensions

    def read_file(self, file_path: str) -> Dict[str, Any]:  # noqa: D401
        """See base class."""
        raise NotImplementedError("PDF support is not implemented")

    def save_file(self, file_path: str, data: Dict[str, Any]) -> bool:  # noqa: D401
        """See base class."""
        raise NotImplementedError("PDF support is not implemented")
