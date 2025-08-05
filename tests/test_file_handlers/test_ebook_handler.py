"""Tests for the :class:`EbookHandler`."""

from src.file_handlers.ebook_handler import EbookHandler


def test_epub_roundtrip(tmp_path):
    handler = EbookHandler()
    data = {
        "title": "sample",
        "content": "Hello EPUB",
        "metadata": {},
        "structure": {},
        "encoding": "utf-8",
        "format": "epub",
    }

    file_path = tmp_path / "sample.epub"
    assert handler.save_file(str(file_path), data)

    read_back = handler.read_file(str(file_path))
    assert "Hello EPUB" in read_back["content"]
