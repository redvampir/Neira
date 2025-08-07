from __future__ import annotations

"""Tests for the sandbox execution helper in :mod:`src.gui.text_editor`."""

import pytest

from src.gui.text_editor import NeyraTextEditor


def test_run_code_success() -> None:
    editor = NeyraTextEditor()
    out, err = editor.run_code("print('hello')")
    assert out.strip() == "hello"
    assert err == ""


def test_run_code_error() -> None:
    editor = NeyraTextEditor()
    out, err = editor.run_code("raise ValueError('boom')")
    assert out == ""
    assert "ValueError" in err


def test_run_code_timeout() -> None:
    editor = NeyraTextEditor()
    with pytest.raises(TimeoutError):
        editor.run_code("import time; time.sleep(1)", timeout=0.2)
