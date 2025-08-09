from __future__ import annotations

"""Palette of available node templates for the visual programming editor.

The palette stores a collection of template definitions that can be searched by
name.  It is intentionally lightweight and does not perform any GUI related
logic; the editor is expected to consume the data structure and render it
accordingly.
"""

from dataclasses import dataclass, field
from typing import Dict, List, Any


@dataclass
class NodeTemplate:
    """Description of a node template.

    Parameters
    ----------
    name:
        Human readable name of the node template.
    data:
        Arbitrary mapping with additional metadata required to instantiate the
        node inside a graph.  The structure is purposely left open to keep the
        module decoupled from the editor implementation.
    """

    name: str
    data: Dict[str, Any] = field(default_factory=dict)


@dataclass
class NodePalette:
    """Container managing a set of :class:`NodeTemplate` objects."""

    templates: Dict[str, NodeTemplate] = field(default_factory=dict)

    # ------------------------------------------------------------------ manage
    def add_template(self, template: NodeTemplate) -> None:
        """Register ``template`` in the palette."""

        self.templates[template.name] = template

    def get_template(self, name: str) -> NodeTemplate | None:
        """Return template identified by ``name`` or ``None`` if missing."""

        return self.templates.get(name)

    # ------------------------------------------------------------------ search
    def search(self, query: str) -> List[NodeTemplate]:
        """Return templates whose names contain ``query``.

        The search is case insensitive and returns a new list with matching
        templates in arbitrary order.
        """

        query = query.lower().strip()
        return [tpl for n, tpl in self.templates.items() if query in n.lower()]


__all__ = ["NodePalette", "NodeTemplate"]
