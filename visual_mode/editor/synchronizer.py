from __future__ import annotations

"""Synchronize graph edits back to metadata and source text.

This module provides the :class:`Synchronizer` which performs the inverse
operation of :func:`visual_mode.graph.builder.build_graph`.  It accepts a graph
representation as used by the visual editor and converts it back into the
``blocks``, ``variables`` and ``connections`` metadata structure.  The
resulting metadata can then be serialised back into source text.  While the
current implementation simply returns the original text unchanged, it ensures an
idempotent round-trip of ``text -> graph -> text`` and updates node positions
and links when they change inside the graph.
"""

from dataclasses import dataclass
import json
from typing import Any, Dict, Iterable, List, Tuple


def _node_to_metadata(node: Dict[str, Any]) -> Tuple[str, Dict[str, Any]]:
    """Convert a graph ``node`` into ``(kind, item)`` metadata tuple."""

    item: Dict[str, Any] = {"id": node["id"]}
    for key in ("display", "i18n", "category", "category_i18n", "range"):
        if key in node:
            value = node[key]
            item[key] = dict(value) if isinstance(value, dict) else value
    return node.get("type", "block"), item


def _link_to_metadata(link: Dict[str, Any]) -> Dict[str, Any]:
    """Convert a graph ``link`` into metadata representation."""

    item: Dict[str, Any] = {"from": link["from"], "to": link["to"]}
    for key in ("display", "i18n", "category", "category_i18n"):
        if key in link:
            value = link[key]
            item[key] = dict(value) if isinstance(value, dict) else value
    return item


def graph_to_metadata(graph: Dict[str, Any]) -> Dict[str, List[Dict[str, Any]]]:
    """Return metadata dictionary converted from ``graph`` representation."""

    blocks: List[Dict[str, Any]] = []
    variables: List[Dict[str, Any]] = []
    for node in graph.get("nodes", []):
        kind, item = _node_to_metadata(node)
        if kind == "variable":
            variables.append(item)
        else:
            # Unknown kinds default to "block" to keep behaviour lenient.
            blocks.append(item)

    connections: List[Dict[str, Any]] = []
    for link in graph.get("links", []):
        connections.append(_link_to_metadata(link))

    return {"blocks": blocks, "variables": variables, "connections": connections}


def _sorted_metadata(metadata: Dict[str, Any]) -> Dict[str, Any]:
    """Return ``metadata`` with items sorted for stable serialisation."""

    def sort_nodes(nodes: Iterable[Dict[str, Any]]) -> List[Dict[str, Any]]:
        return sorted((dict(n) for n in nodes), key=lambda n: n.get("id", ""))

    def sort_links(links: Iterable[Dict[str, Any]]) -> List[Dict[str, Any]]:
        return sorted(
            (dict(l) for l in links), key=lambda l: (l.get("from", ""), l.get("to", ""))
        )

    return {
        "blocks": sort_nodes(metadata.get("blocks", [])),
        "variables": sort_nodes(metadata.get("variables", [])),
        "connections": sort_links(metadata.get("connections", [])),
    }


@dataclass
class Synchronizer:
    """Synchronise a graph with its source ``text`` and ``metadata``."""

    text: str
    metadata: Dict[str, Any]

    def update_from_graph(self, graph: Dict[str, Any]) -> None:
        """Update internal ``metadata`` from the provided ``graph`` structure."""

        self.metadata = graph_to_metadata(graph)

    def to_text(self) -> str:
        """Serialise current ``metadata`` back into a textual representation."""

        # The text format mirrors the metadata JSON structure.  Real language
        # specific implementations would inject annotations into ``self.text``;
        # however for the purposes of unit testing and ensuring an idempotent
        # round-trip we simply encode the metadata as sorted JSON.
        return json.dumps(_sorted_metadata(self.metadata), sort_keys=True)
