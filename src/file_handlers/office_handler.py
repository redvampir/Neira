"""Placeholder handlers for office document formats."""

from __future__ import annotations

from pathlib import Path
from typing import Any, Dict

from .base_handler import BaseFileHandler


class OfficeHandler(BaseFileHandler):
    """Handle ``.doc``, ``.docx`` and ``.odt`` files.

    The implementation is intentionally minimal and only validates support for
    a given file extension.  Actual reading and writing will be implemented in
    future iterations.
    """

    _extensions = {".doc", ".docx", ".odt"}

    def can_handle(self, file_path: str) -> bool:  # noqa: D401
        """See base class."""
        return Path(file_path).suffix.lower() in self._extensions

    def read_file(self, file_path: str) -> Dict[str, Any]:  # noqa: D401
        """See base class."""
        raise NotImplementedError("Office document support is not implemented")

    def save_file(self, file_path: str, data: Dict[str, Any]) -> bool:  # noqa: D401
        """See base class."""
        raise NotImplementedError("Office document support is not implemented")
