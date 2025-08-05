"""PDF document handler using :mod:`PyPDF2`."""

from __future__ import annotations

from pathlib import Path
from typing import Any, Dict

from .base_handler import BaseFileHandler

try:  # pragma: no-cover - optional dependency
    from PyPDF2 import PdfReader, PdfWriter
except Exception:  # pragma: no-cover
    PdfReader = None  # type: ignore
    PdfWriter = None  # type: ignore


class PDFHandler(BaseFileHandler):
    """Handle ``.pdf`` files."""

    _extensions = {".pdf"}

    def can_handle(self, file_path: str) -> bool:  # noqa: D401
        """See base class."""
        return Path(file_path).suffix.lower() in self._extensions

    def _require_pypdf(self) -> None:
        if PdfReader is None or PdfWriter is None:  # pragma: no cover
            raise RuntimeError("PyPDF2 is required to handle PDF files")

    def read_file(self, file_path: str) -> Dict[str, Any]:  # noqa: D401
        """See base class."""
        self._require_pypdf()
        path = Path(file_path)
        try:
            reader = PdfReader(str(path))
        except Exception as exc:  # pragma: no cover - corrupted file
            raise RuntimeError(f"Failed to read PDF: {exc}") from exc
        texts = []
        for page in reader.pages:
            text = page.extract_text() or ""
            if text:
                texts.append(text)
        metadata = {k[1:]: v for k, v in (reader.metadata or {}).items()}
        return {
            "title": path.stem,
            "content": "\n".join(texts),
            "metadata": metadata,
            "structure": {"pages": len(reader.pages)},
            "encoding": "utf-8",
            "format": "pdf",
        }

    def save_file(self, file_path: str, data: Dict[str, Any]) -> bool:  # noqa: D401
        """See base class."""
        self._require_pypdf()
        path = Path(file_path)
        content = data.get("content", "")
        try:
            try:  # pragma: no cover - reportlab may not be installed
                from reportlab.pdfgen import canvas
            except Exception as exc:  # pragma: no cover
                raise RuntimeError("reportlab is required to save PDF files") from exc

            # First create a simple PDF with ReportLab
            c = canvas.Canvas(str(path))
            textobject = c.beginText(40, 800)
            for line in content.splitlines():
                textobject.textLine(line)
            c.drawText(textobject)
            c.save()

            # Attach any metadata using PyPDF2
            reader = PdfReader(str(path))
            writer = PdfWriter()
            writer.clone_document_from_reader(reader)
            if meta := data.get("metadata"):
                writer.add_metadata({f"/{k}": v for k, v in meta.items()})
            with open(path, "wb") as fh:
                writer.write(fh)
        except Exception as exc:  # pragma: no cover - disk issues etc.
            raise RuntimeError(f"Failed to write PDF: {exc}") from exc
        return True
