"""General utilities for working with text files."""

from __future__ import annotations

from pathlib import Path
from typing import Tuple

from .encoding_detector import detect_encoding


def read_text_file(path: str | Path) -> Tuple[str, str]:
    """Return the contents of *path* and the detected encoding."""
    path = Path(path)
    encoding = detect_encoding(path)
    return path.read_text(encoding=encoding), encoding


def write_text_file(path: str | Path, content: str, encoding: str = "utf-8") -> None:
    """Write *content* to *path* using *encoding*."""
    Path(path).write_text(content, encoding=encoding)
