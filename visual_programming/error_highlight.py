from __future__ import annotations

"""Helpers for detecting common graph issues such as unconnected ports or
missing required data.  The returned errors can be used by the editor to visually
highlight problematic nodes or ports.
"""

from typing import Any, Dict, List


def highlight_errors(graph: Dict[str, Any]) -> List[str]:
    """Return a list of human readable error descriptions for ``graph``.

    The function operates on a very small set of conventions in order to remain
    compatible with various graph implementations:

    * Nodes may define a ``ports`` mapping.  Each port is expected to contain a
      ``connections`` list (or ``connected_to``) describing links to other
      nodes.  Ports without connections are reported as errors.
    * When a node specifies ``requires_data`` it must also provide a non-empty
      ``data`` entry.  Otherwise an error about missing data is generated.
    """

    errors: List[str] = []
    for node in graph.get("nodes", []):
        node_id = node.get("id", "<unnamed>")
        ports = node.get("ports", {})
        for port_name, port in ports.items():
            connections = port.get("connections") or port.get("connected_to") or []
            if not connections:
                errors.append(f"{node_id}.{port_name} has no connection")
        if node.get("requires_data") and not node.get("data"):
            errors.append(f"{node_id} is missing required data")
    return errors


__all__ = ["highlight_errors"]
