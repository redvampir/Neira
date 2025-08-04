"""Handlers for plain text based formats."""

from __future__ import annotations

from pathlib import Path
from typing import Any, Dict

from ..utils.encoding_detector import detect_encoding
from .base_handler import BaseFileHandler

try:  # pragma: no-cover - optional dependency
    from striprtf.striprtf import rtf_to_text
except Exception:  # pragma: no-cover
    rtf_to_text = None  # type: ignore


class TextHandler(BaseFileHandler):
    """Read and write simple text formats such as ``.txt`` or ``.md``."""

    _extensions = {".txt", ".md", ".rtf", ".tex"}

    def can_handle(self, file_path: str) -> bool:  # noqa: D401 - short and sweet
        """See base class."""
        return Path(file_path).suffix.lower() in self._extensions

    def read_file(self, file_path: str) -> Dict[str, Any]:  # noqa: D401
        """See base class."""
        path = Path(file_path)
        ext = path.suffix.lower()
        encoding = detect_encoding(path)
        if ext == ".rtf":
            if rtf_to_text is None:
                raise RuntimeError("striprtf is required to read RTF files")
            raw = path.read_text(encoding=encoding)
            content = rtf_to_text(raw)
        else:
            content = path.read_text(encoding=encoding)
        return {
            "title": path.stem,
            "content": content,
            "metadata": {},
            "structure": {},
            "encoding": encoding,
            "format": ext.lstrip("."),
        }

    def save_file(self, file_path: str, data: Dict[str, Any]) -> bool:  # noqa: D401
        """See base class."""
        path = Path(file_path)
        ext = path.suffix.lower()
        encoding = data.get("encoding", "utf-8")
        content = data.get("content", "")
        if ext == ".rtf":
            raise NotImplementedError("Saving RTF files is not supported yet")
        path.write_text(content, encoding=encoding)
        return True
