"""Tests for the :class:`PDFHandler`."""

from src.file_handlers.pdf_handler import PDFHandler


def test_pdf_roundtrip(tmp_path):
    handler = PDFHandler()
    data = {
        "title": "sample",
        "content": "Hello PDF world",
        "metadata": {},
        "structure": {},
        "encoding": "utf-8",
        "format": "pdf",
    }

    file_path = tmp_path / "sample.pdf"
    assert handler.save_file(str(file_path), data)

    read_back = handler.read_file(str(file_path))
    assert "Hello PDF world" in read_back["content"]
