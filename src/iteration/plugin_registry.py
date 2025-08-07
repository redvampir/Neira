from __future__ import annotations

"""Registry for search plugins used by :class:`DeepSearcher`.

This module provides a minimal plugin system allowing external components to
register search handlers that implement the :class:`SearchPlugin` protocol.
Registered plugins are later invoked by ``DeepSearcher`` to obtain additional
search results.  A simple example plugin wrapping :class:`SearchAPIClient` is
provided for convenience.
"""

from typing import Dict, List, Any, Protocol, runtime_checkable

from src.search import SearchAPIClient


@runtime_checkable
class SearchPlugin(Protocol):
    """Protocol that all search plugins must follow."""

    def search(self, query: str, limit: int = 5) -> List[Dict[str, Any]]:
        """Return a list of search result dictionaries."""
        ...


# Internal storage for registered plugins.  A dictionary keyed by plugin name
# is used to avoid duplicate registrations of the same plugin class.
_search_plugins: Dict[str, SearchPlugin] = {}


# ---------------------------------------------------------------------------
def register_search_plugin(plugin: SearchPlugin, name: str | None = None) -> None:
    """Register ``plugin`` under ``name``.

    Parameters
    ----------
    plugin:
        Instance implementing :class:`SearchPlugin`.
    name:
        Optional name under which the plugin is stored.  When omitted, the
        plugin's class name is used.  Registering another plugin with the same
        name replaces the previous entry.
    """

    key = name or plugin.__class__.__name__
    _search_plugins[key] = plugin


# ---------------------------------------------------------------------------
def get_search_plugins() -> List[SearchPlugin]:
    """Return all registered search plugins."""

    return list(_search_plugins.values())


# ---------------------------------------------------------------------------
def clear_search_plugins() -> None:
    """Remove all registered search plugins.

    Primarily intended for test isolation.
    """

    _search_plugins.clear()


# ---------------------------------------------------------------------------
class APISearchPlugin:
    """Example plugin that delegates to :class:`SearchAPIClient`.

    The plugin performs a web search and converts results to the structure
    expected by :class:`DeepSearcher`.
    """

    def __init__(self, client: SearchAPIClient | None = None) -> None:
        self.client = client or SearchAPIClient()

    def search(self, query: str, limit: int = 5) -> List[Dict[str, Any]]:
        results: List[Dict[str, Any]] = []
        for item in self.client.search(query, limit):
            results.append(
                {
                    "source": "web",
                    "reference": item.get("url", ""),
                    "content": item.get("snippet", ""),
                    "priority": 0.3,
                }
            )
        return results


__all__ = [
    "SearchPlugin",
    "register_search_plugin",
    "get_search_plugins",
    "clear_search_plugins",
    "APISearchPlugin",
]
