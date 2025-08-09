"""Collaboration utilities for Neira editors."""

try:  # pragma: no cover - optional dependency
    from .server import CollabServer
except Exception:  # pragma: no cover - gracefully degrade
    CollabServer = None  # type: ignore[assignment]

try:  # pragma: no cover - optional dependency
    from .client import CollabClient
except Exception:  # pragma: no cover - gracefully degrade
    CollabClient = None  # type: ignore[assignment]

__all__ = ["CollabServer", "CollabClient"]
