from __future__ import annotations

"""Simple knowledge graph for storing relations between entities.

This module relies on :mod:`networkx` to keep track of nodes (characters,
worlds, etc.) and edges (relationships).  The graph is persisted in both JSON
and GraphML formats so that it remains compatible with the existing file based
storage used throughout the project.
"""

from pathlib import Path
import json
from typing import Any, Dict, Tuple

import networkx as nx
from networkx.readwrite import json_graph

try:  # pragma: no cover - networkx may not support GraphML in minimal builds
    from networkx.readwrite.graphml import write_graphml
except Exception:  # pragma: no cover - fallback for environments without graphml
    write_graphml = None  # type: ignore


class KnowledgeGraph:
    """Maintain a directed multigraph of entities and relations."""

    def __init__(
        self,
        json_path: str | Path | None = None,
        graphml_path: str | Path | None = None,
    ) -> None:
        self.json_path = Path(json_path or "data/knowledge_graph.json")
        self.graphml_path = Path(graphml_path or "data/knowledge_graph.graphml")
        self.graph = nx.MultiDiGraph()
        self._load()

    # ------------------------------------------------------------------
    def _load(self) -> None:
        """Load previously stored graph if available."""
        if not self.json_path.exists():
            return
        try:
            data = json.loads(self.json_path.read_text(encoding="utf-8"))
            self.graph = json_graph.node_link_graph(data, multigraph=True)
        except Exception:
            self.graph = nx.MultiDiGraph()

    # ------------------------------------------------------------------
    def add_world(self, name: str, **attrs: Any) -> None:
        """Register a world node."""
        self.graph.add_node(name, type="world", **attrs)

    def add_character(self, character: Any, world: str | None = None) -> None:
        """Register a character node and its relations."""
        if hasattr(character, "to_dict"):
            data = character.to_dict()
        elif isinstance(character, dict):
            data = character
        else:
            data = {"name": str(character)}
        name = data.get("name") or getattr(character, "name", None)
        if not name:
            return
        self.graph.add_node(name, type="character")
        # Optional link to world
        world = world or data.get("world") or getattr(character, "world", None)
        if world:
            self.assign_character_world(name, world)
        # Relationships between characters
        relationships = data.get("relationships", {})
        for other, relation in relationships.items():
            self.relate_characters(name, other, relation)

    def relate_characters(self, src: str, dst: str, relation: str) -> None:
        """Add an edge describing a relation between two characters."""
        self.graph.add_edge(src, dst, relation=relation)

    def assign_character_world(self, character: str, world: str) -> None:
        """Connect a character node with a world node."""
        self.graph.add_edge(character, world, relation="belongs_to")

    # ------------------------------------------------------------------
    def export_json(self) -> None:
        """Persist graph to JSON using node-link format."""
        self.json_path.parent.mkdir(parents=True, exist_ok=True)
        data = json_graph.node_link_data(self.graph)
        self.json_path.write_text(
            json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8"
        )

    def export_graphml(self) -> None:
        """Persist graph to GraphML if the writer is available."""
        if write_graphml is None:
            return
        self.graphml_path.parent.mkdir(parents=True, exist_ok=True)
        write_graphml(self.graph, self.graphml_path)

    # ------------------------------------------------------------------
    def check_relation(self, src: str, dst: str, relation: str) -> bool:
        """Check whether a specific relation exists between two nodes."""
        edges = self.graph.get_edge_data(src, dst) or {}
        return any(data.get("relation") == relation for data in edges.values())

    def check_claim(self, claim: str) -> Tuple[bool | None, float]:
        """Naively verify textual claims using the graph.

        The function recognises simple Russian phrases describing relations,
        for example::

            "Алиса принадлежит миру Wonderland"
            "Боб связан с Алиса"
        """

        import re

        pattern_world = re.search(
            r"(?P<char>[A-Za-zА-Яа-яЁё]+)\s+принадлежит\s+миру\s+(?P<world>[A-Za-zА-Яа-яЁё]+)",
            claim,
            re.IGNORECASE,
        )
        if pattern_world:
            char = pattern_world.group("char")
            world = pattern_world.group("world")
            return self.check_relation(char, world, "belongs_to"), 1.0

        pattern_rel = re.search(
            r"(?P<src>[A-Za-zА-Яа-яЁё]+)\s+связан\s+с\s+(?P<dst>[A-Za-zА-Яа-яЁё]+)",
            claim,
            re.IGNORECASE,
        )
        if pattern_rel:
            src = pattern_rel.group("src")
            dst = pattern_rel.group("dst")
            exists = self.graph.has_edge(src, dst) or self.graph.has_edge(dst, src)
            return exists, 1.0

        return None, 0.0


# Shared instance used across the project
knowledge_graph = KnowledgeGraph()

__all__ = ["KnowledgeGraph", "knowledge_graph"]
