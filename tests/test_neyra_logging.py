"""Logging tests for Neyra core."""

from __future__ import annotations

import logging
import pytest

from src.core.neyra_brain import Neyra


def test_load_llm_invalid_config_logs_error(tmp_path, caplog, monkeypatch) -> None:
    config_dir = tmp_path / "config"
    config_dir.mkdir()
    config_file = config_dir / "llm_config.json"
    config_file.write_text("{bad json", encoding="utf-8")
    monkeypatch.chdir(tmp_path)
    with caplog.at_level(logging.ERROR, logger="src.core.neyra_brain"):
        Neyra()
    assert "llm_config.json" in caplog.text
    assert "JSON" in caplog.text


def test_load_book_unreadable_logs_error(tmp_path, caplog) -> None:
    neyra = Neyra()
    unreadable_path = tmp_path / "somedir"
    unreadable_path.mkdir()
    with caplog.at_level(logging.ERROR, logger="src.core.neyra_brain"):
        neyra.load_book(str(unreadable_path))
    assert str(unreadable_path) in caplog.text
    assert "Ошибка" in caplog.text
