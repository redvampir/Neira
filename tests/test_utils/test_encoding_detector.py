"""Tests for the encoding detection helper."""
from __future__ import annotations

from pathlib import Path
import builtins
import importlib
import sys

ROOT = Path(__file__).resolve().parents[2]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

import src.utils.encoding_detector as encoding_detector


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
        assert encoding_detector.detect_encoding(file) == expected


def test_detect_encoding_without_chardet(monkeypatch, tmp_path: Path) -> None:
    """Detection should fall back gracefully when ``chardet`` is missing."""
    sample = "Привет, Нейра!"
    file = tmp_path / "sample_cp1251.txt"
    _write_text(file, sample, "cp1251")

    original_import = builtins.__import__

    def fake_import(name, *args, **kwargs):
        if name == "chardet":
            raise ImportError
        return original_import(name, *args, **kwargs)

    monkeypatch.setattr(builtins, "__import__", fake_import)
    monkeypatch.delitem(sys.modules, "chardet", raising=False)

    module = importlib.reload(encoding_detector)
    assert module.chardet is None
    assert module.detect_encoding(file) == "windows-1251"

    monkeypatch.setattr(builtins, "__import__", original_import)
    importlib.reload(encoding_detector)
