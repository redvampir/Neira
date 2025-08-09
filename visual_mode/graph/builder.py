from __future__ import annotations

"""Utilities for constructing visual graphs from metadata.

This module converts the metadata produced by the visual mode parsers into a
node/link representation expected by the visual editor.  The metadata format is
specified in :mod:`visual_mode.metadata_schema` and essentially consists of
``blocks``, ``variables`` and ``connections`` entries.  Each entry may define
optional display names, localisation mappings and categories.

The :func:`build_graph` function performs the in memory transformation while
:func:`write_graph` persists the graph structure as ``.neyra-graph`` JSON file.
Both functions accept raw metadata in dictionary form and are completely
language agnostic.
"""

from pathlib import Path
import json
from typing import Any, Dict, List


def _node_from_metadata(item: Dict[str, Any], kind: str) -> Dict[str, Any]:
    """Return a node dictionary extracted from ``item``.

    ``kind`` describes the node type (e.g. ``"block"`` or ``"variable"``).
    All optional metadata like display names, categories and localisation
    mappings are forwarded verbatim if present.
    """

    node: Dict[str, Any] = {"id": item["id"], "type": kind}
    if "display" in item:
        node["display"] = item["display"]
    if "i18n" in item:
        node["i18n"] = dict(item["i18n"])
    if "category" in item:
        node["category"] = item["category"]
    if "category_i18n" in item:
        node["category_i18n"] = dict(item["category_i18n"])
    if "range" in item:
        node["range"] = item["range"]
    return node


def _link_from_metadata(item: Dict[str, Any]) -> Dict[str, Any]:
    """Return a connection dictionary extracted from ``item``."""

    link: Dict[str, Any] = {"from": item["from"], "to": item["to"]}
    if "display" in item:
        link["display"] = item["display"]
    if "i18n" in item:
        link["i18n"] = dict(item["i18n"])
    if "category" in item:
        link["category"] = item["category"]
    if "category_i18n" in item:
        link["category_i18n"] = dict(item["category_i18n"])
    return link


def build_graph(metadata: Dict[str, Any]) -> Dict[str, List[Dict[str, Any]]]:
    """Construct a visual graph representation from ``metadata``.

    Parameters
    ----------
    metadata:
        Dictionary following :mod:`visual_mode.metadata_schema` containing
        ``blocks``, ``variables`` and ``connections`` arrays.

    Returns
    -------
    dict
        A dictionary with ``nodes`` and ``links`` lists ready to be consumed by
        the visual editor.
    """

    nodes: List[Dict[str, Any]] = []
    for block in metadata.get("blocks", []):
        nodes.append(_node_from_metadata(block, "block"))
    for variable in metadata.get("variables", []):
        nodes.append(_node_from_metadata(variable, "variable"))

    links: List[Dict[str, Any]] = []
    for connection in metadata.get("connections", []):
        links.append(_link_from_metadata(connection))

    return {"nodes": nodes, "links": links}


def write_graph(metadata: Dict[str, Any], path: str | Path) -> Path:
    """Build a graph from ``metadata`` and write it as ``.neyra-graph`` JSON.

    The function returns the final path of the written file.  If ``path`` does
    not already end with the ``.neyra-graph`` extension it is appended
    automatically.
    """

    graph = build_graph(metadata)
    out_path = Path(path)
    if out_path.suffix != ".neyra-graph":
        out_path = out_path.with_suffix(".neyra-graph")
    out_path.write_text(json.dumps(graph, indent=2, ensure_ascii=False))
    return out_path
