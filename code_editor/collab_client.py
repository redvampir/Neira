"""Collaborative client for the code editor."""

from collab.client import CollabClient


class CodeEditorCollabClient(CollabClient):
    """Thin wrapper around :class:`collab.client.CollabClient` for clarity."""

    pass


__all__ = ["CodeEditorCollabClient"]
