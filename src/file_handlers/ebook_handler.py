"""Handler for electronic book formats using :mod:`ebooklib`."""

from __future__ import annotations

import re
from pathlib import Path
from typing import Any, Dict

from .base_handler import BaseFileHandler

try:  # pragma: no-cover - optional dependency
    from ebooklib import epub, ITEM_DOCUMENT
except Exception:  # pragma: no-cover
    epub = None  # type: ignore
    ITEM_DOCUMENT = None  # type: ignore


class EbookHandler(BaseFileHandler):
    """Handle ``.epub`` files.

    The handler currently supports reading and writing ``.epub`` documents.  The
    ``.fb2`` and ``.mobi`` formats are recognised but will raise a
    ``RuntimeError`` when used.
    """

    _extensions = {".epub", ".fb2", ".mobi"}

    def can_handle(self, file_path: str) -> bool:  # noqa: D401
        """See base class."""
        return Path(file_path).suffix.lower() in self._extensions

    def _require_epub(self) -> None:
        if epub is None:  # pragma: no cover - happens when dependency missing
            raise RuntimeError("ebooklib is required to handle e-book files")

    def read_file(self, file_path: str) -> Dict[str, Any]:  # noqa: D401
        """See base class."""
        path = Path(file_path)
        ext = path.suffix.lower()
        if ext != ".epub":
            raise RuntimeError(f"Unsupported e-book format: {ext}")
        self._require_epub()
        try:
            book = epub.read_epub(str(path))
        except Exception as exc:  # pragma: no cover - corrupted file
            raise RuntimeError(f"Failed to read e-book: {exc}") from exc
        texts = []
        for item in book.get_items_of_type(ITEM_DOCUMENT):
            try:
                raw = item.get_content().decode("utf-8", errors="ignore")
            except Exception:  # pragma: no cover - safety
                continue
            texts.append(re.sub(r"<[^>]+>", "", raw))
        metadata = {k: v for k, v in book.metadata.items()}
        return {
            "title": book.get_metadata("DC", "title")[0][0]
            if book.get_metadata("DC", "title")
            else path.stem,
            "content": "\n".join(texts),
            "metadata": metadata,
            "structure": {"chapters": len(texts)},
            "encoding": "utf-8",
            "format": ext.lstrip("."),
        }

    def save_file(self, file_path: str, data: Dict[str, Any]) -> bool:  # noqa: D401
        """See base class."""
        path = Path(file_path)
        ext = path.suffix.lower()
        if ext != ".epub":
            raise RuntimeError(f"Unsupported e-book format: {ext}")
        self._require_epub()
        try:
            book = epub.EpubBook()
            title = data.get("title", path.stem)
            book.set_title(title)
            chapter = epub.EpubHtml(
                title="chapter1",
                file_name="chap_1.xhtml",
                content=f"<p>{data.get('content', '')}</p>",
            )
            book.add_item(chapter)
            book.add_item(epub.EpubNcx())
            book.add_item(epub.EpubNav())
            book.spine = ["nav", chapter]
            epub.write_epub(str(path), book)
        except Exception as exc:  # pragma: no cover - disk issues etc.
            raise RuntimeError(f"Failed to write e-book: {exc}") from exc
        return True
