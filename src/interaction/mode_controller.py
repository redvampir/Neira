from __future__ import annotations

"""Response formatting modes for source visibility."""

from typing import Iterable, Protocol

from src.utils.source_manager import ManagedSource


class ResponseMode(Protocol):
    """Protocol for formatting responses with optional sources."""

    def format_response(self, content: str, sources: Iterable[ManagedSource]) -> str:
        """Return formatted response using ``content`` and ``sources``."""
        raise NotImplementedError


class HiddenSourcesMode:
    """Return content without any source references."""

    def format_response(self, content: str, sources: Iterable[ManagedSource]) -> str:  # noqa: D401
        return content


class VisibleSourcesMode:
    """Append full source information to the content."""

    def format_response(self, content: str, sources: Iterable[ManagedSource]) -> str:
        source_list = list(sources)
        if not source_list:
            return content
        lines = [content, "", "Sources:"]
        for idx, src in enumerate(source_list, 1):
            lines.append(f"[{idx}] {src.summary} ({src.path})")
        return "\n".join(lines)


class LightweightMode:
    """Append only source paths to keep output compact."""

    def format_response(self, content: str, sources: Iterable[ManagedSource]) -> str:
        source_list = list(sources)
        if not source_list:
            return content
        lines = [content, "", "Sources:"]
        for idx, src in enumerate(source_list, 1):
            lines.append(f"[{idx}] {src.path}")
        return "\n".join(lines)


__all__ = [
    "ResponseMode",
    "HiddenSourcesMode",
    "VisibleSourcesMode",
    "LightweightMode",
]
