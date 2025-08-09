"""Collaborative client for the visual programming editor."""

from collab.client import CollabClient


class VisualCollabClient(CollabClient):
    """Wrapper around :class:`collab.client.CollabClient` for the visual editor."""

    pass


__all__ = ["VisualCollabClient"]
