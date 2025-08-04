"""Placeholder handler for writer specific formats like Scrivener or yWriter."""

from __future__ import annotations

from pathlib import Path
from typing import Any, Dict

from .base_handler import BaseFileHandler


class WriterFormatHandler(BaseFileHandler):
    """Handle writer-centric project files such as ``.scrivx`` or ``.ywriter``."""

    _extensions = {".scrivx", ".ywriter", ".json"}

    def can_handle(self, file_path: str) -> bool:  # noqa: D401
        """See base class."""
        return Path(file_path).suffix.lower() in self._extensions

    def read_file(self, file_path: str) -> Dict[str, Any]:  # noqa: D401
        """See base class."""
        raise NotImplementedError(
            "Writer format support is not implemented"
        )

    def save_file(self, file_path: str, data: Dict[str, Any]) -> bool:  # noqa: D401
        """See base class."""
        raise NotImplementedError(
            "Writer format support is not implemented"
        )
