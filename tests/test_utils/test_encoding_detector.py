"""Tests for the encoding detection helper."""
from __future__ import annotations

from pathlib import Path

from src.utils.encoding_detector import detect_encoding


def _write_text(path: Path, text: str, encoding: str) -> None:
    path.write_bytes(text.encode(encoding))


def test_detect_encoding_handles_common_encodings(tmp_path: Path) -> None:
    """The helper should recognise several common encodings."""
    sample = "Привет, Нейра!"
    cases = {
        "utf-8": "utf-8",
        "utf-16": "utf-16",
        "cp1251": "windows-1251",
        "cp866": "cp866",
    }

    for enc, expected in cases.items():
        file = tmp_path / f"sample_{enc}.txt"
        _write_text(file, sample, enc)
        assert detect_encoding(file) == expected
