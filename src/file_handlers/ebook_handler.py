"""Placeholder handler for electronic book formats."""

from __future__ import annotations

from pathlib import Path
from typing import Any, Dict

from .base_handler import BaseFileHandler


class EbookHandler(BaseFileHandler):
    """Handle ``.epub``, ``.fb2`` and ``.mobi`` files."""

    _extensions = {".epub", ".fb2", ".mobi"}

    def can_handle(self, file_path: str) -> bool:  # noqa: D401
        """See base class."""
        return Path(file_path).suffix.lower() in self._extensions

    def read_file(self, file_path: str) -> Dict[str, Any]:  # noqa: D401
        """See base class."""
        raise NotImplementedError("E-book support is not implemented")

    def save_file(self, file_path: str, data: Dict[str, Any]) -> bool:  # noqa: D401
        """See base class."""
        raise NotImplementedError("E-book support is not implemented")
