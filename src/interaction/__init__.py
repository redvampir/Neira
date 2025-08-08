"""High level helpers for interacting with Neyra."""

try:  # pragma: no cover - optional dependencies during tests
    from .chat_session import ChatSession
except Exception:  # noqa: BLE001
    ChatSession = None  # type: ignore
from .request_history import RequestHistory
from .tag_processor import TagProcessor, handle_command

__all__ = ["ChatSession", "RequestHistory", "TagProcessor", "handle_command"]
