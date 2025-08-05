"""Tests for the :class:`OfficeHandler`."""

from src.file_handlers.office_handler import OfficeHandler


def test_docx_roundtrip(tmp_path):
    handler = OfficeHandler()
    data = {
        "title": "sample",
        "content": "Hello DOCX",
        "metadata": {},
        "structure": {},
        "encoding": "utf-8",
        "format": "docx",
    }

    file_path = tmp_path / "sample.docx"
    assert handler.save_file(str(file_path), data)

    read_back = handler.read_file(str(file_path))
    assert read_back["content"].strip() == "Hello DOCX"
