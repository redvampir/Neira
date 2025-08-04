"""Library manager for the desktop interface."""

from __future__ import annotations

from pathlib import Path
from typing import Any, Dict

from ..file_handlers import (
    BaseFileHandler,
    EbookHandler,
    OfficeHandler,
    PDFHandler,
    TextHandler,
    WriterFormatHandler,
)


class BookManager:
    """Maintain a collection of books and projects."""

    def __init__(self, parent: Any | None = None) -> None:
        self.parent = parent
        self.file_handlers = self.load_file_handlers()

    def load_file_handlers(self) -> Dict[str, BaseFileHandler]:
        """Instantiate file handlers for supported formats."""
        return {
            ".txt": TextHandler(),
            ".md": TextHandler(),
            ".rtf": TextHandler(),
            ".tex": TextHandler(),
            ".doc": OfficeHandler(),
            ".docx": OfficeHandler(),
            ".odt": OfficeHandler(),
            ".epub": EbookHandler(),
            ".fb2": EbookHandler(),
            ".mobi": EbookHandler(),
            ".pdf": PDFHandler(),
            ".scrivx": WriterFormatHandler(),
            ".ywriter": WriterFormatHandler(),
            ".json": WriterFormatHandler(),
        }

    def _get_handler(self, file_path: str) -> BaseFileHandler:
        ext = Path(file_path).suffix.lower()
        handler = self.file_handlers.get(ext)
        if not handler:
            raise ValueError(f"Unsupported file format: {ext}")
        return handler

    def open_file(self, file_path: str) -> Dict[str, Any]:
        """Open ``file_path`` using the appropriate handler."""
        handler = self._get_handler(file_path)
        return handler.read_file(file_path)

    def save_file(self, file_path: str, data: Dict[str, Any]) -> bool:
        """Save ``data`` to ``file_path`` using the appropriate handler."""
        handler = self._get_handler(file_path)
        return handler.save_file(file_path, data)
