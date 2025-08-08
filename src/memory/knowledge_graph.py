"""Python adapter for the Rust knowledge graph."""

from __future__ import annotations
from typing import Any

from neira_rust import KnowledgeGraph as _RustKG


class KnowledgeGraph:
    def __init__(self) -> None:
        self._kg = _RustKG()

    def add_fact(self, subject: str, relation: str, obj: str) -> None:
        self._kg.add_fact(subject, relation, obj)

    def check_claim(self, claim: str):
        return self._kg.check_claim(claim)

    def add_world(self, name: str, **_attrs: Any) -> None:
        self.add_fact(name, "type", "world")

    def add_character(self, character: Any, world: str | None = None) -> None:
        if hasattr(character, "to_dict"):
            data = character.to_dict()
        elif isinstance(character, dict):
            data = character
        else:
            data = {"name": str(character)}
        name = data.get("name") or getattr(character, "name", None)
        if not name:
            return
        self.add_fact(name, "type", "character")
        world = world or data.get("world") or getattr(character, "world", None)
        if world:
            self.add_fact(name, "belongs_to", world)
        relationships = data.get("relationships", {})
        for other, relation in relationships.items():
            self.add_fact(name, relation, other)

    def relate_characters(self, src: str, dst: str, relation: str) -> None:
        self.add_fact(src, relation, dst)

    def assign_character_world(self, character: str, world: str) -> None:
        self.add_fact(character, "belongs_to", world)

    # Persistence is handled elsewhere in the project; these are no-ops.
    def export_json(self) -> None:  # pragma: no cover - compatibility shim
        return None

    def export_graphml(self) -> None:  # pragma: no cover - compatibility shim
        return None


knowledge_graph = KnowledgeGraph()

__all__ = ["KnowledgeGraph", "knowledge_graph"]
