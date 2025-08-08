"""Storage for information about fictional worlds."""
from __future__ import annotations

from dataclasses import asdict, dataclass, field
import json
import logging
from pathlib import Path
from typing import Any, Dict, List

from .knowledge_graph import knowledge_graph

@dataclass
class WorldRule:
    """Rule that defines some aspect of a world."""

    category: str
    description: str
    examples: List[str] = field(default_factory=list)

    # ------------------------------------------------------------------
    def to_dict(self) -> Dict[str, Any]:
        """Return a serialisable representation of the rule."""
        return asdict(self)

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "WorldRule":
        """Create a :class:`WorldRule` from a serialised form."""
        return cls(**data)


@dataclass
class CulturalInfo:
    """Information about a culture inside a world."""

    name: str
    category: str
    description: str
    examples: List[str] = field(default_factory=list)

    # ------------------------------------------------------------------
    def to_dict(self) -> Dict[str, Any]:
        """Return a serialisable representation of the cultural info."""
        return asdict(self)

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "CulturalInfo":
        """Create a :class:`CulturalInfo` from a serialised form."""
        return cls(**data)


class WorldMemory:
    """Remember details about worlds and persist them to disk."""

    def __init__(self, storage_path: str | Path | None = None) -> None:
        self.storage_path = Path(storage_path or "data/world_memory.json")
        # Mapping of world name to its rules and cultural information
        self._data: Dict[str, Dict[str, List[Any]]] = {}
        self.load()

    # ------------------------------------------------------------------
    def add_rule(
        self,
        world: str,
        category: str,
        description: str,
        examples: List[str] | None = None,
    ) -> None:
        """Add a rule for a specific world."""
        rule = WorldRule(category=category, description=description, examples=examples or [])
        world_entry = self._data.setdefault(world, {"rules": [], "cultures": []})
        world_entry["rules"].append(rule)
        return rule

    def add_culture(
        self,
        world: str,
        name: str,
        category: str,
        description: str,
        examples: List[str] | None = None,
    ) -> None:
        """Add cultural information for a world."""
        culture = CulturalInfo(
            name=name,
            category=category,
            description=description,
            examples=examples or [],
        )
        world_entry = self._data.setdefault(world, {"rules": [], "cultures": []})
        world_entry["cultures"].append(culture)
        return culture

    # ------------------------------------------------------------------
    def get(self, world: str | None = None) -> Any:
        """Retrieve information about a world or all worlds."""
        if world is None:
            return {
                name: {
                    "rules": [asdict(rule) for rule in data.get("rules", [])],
                    "cultures": [asdict(c) for c in data.get("cultures", [])],
                }
                for name, data in self._data.items()
            }
        entry = self._data.get(world)
        if entry is None:
            return None
        return {
            "rules": [asdict(rule) for rule in entry.get("rules", [])],
            "cultures": [asdict(c) for c in entry.get("cultures", [])],
        }

    # ------------------------------------------------------------------
    def save(self) -> None:
        """Persist current memory to the storage file."""
        serialised: Dict[str, Any] = {}
        for world, info in self._data.items():
            serialised[world] = {
                "rules": [r.to_dict() for r in info.get("rules", [])],
                "cultures": [c.to_dict() for c in info.get("cultures", [])],
            }
        self.storage_path.parent.mkdir(parents=True, exist_ok=True)
        self.storage_path.write_text(
            json.dumps(serialised, ensure_ascii=False, indent=2),
            encoding="utf-8",
        )
        for world in serialised.keys():
            knowledge_graph.add_world(world)
        knowledge_graph.export_json()
        knowledge_graph.export_graphml()

    def load(self) -> None:
        """Load memory from disk."""
        if not self.storage_path.exists():
            return
        try:
            raw = json.loads(self.storage_path.read_text(encoding="utf-8"))
        except json.JSONDecodeError as exc:
            logging.getLogger(__name__).warning(
                "Failed to decode world memory file %s: %s", self.storage_path, exc
            )
            try:
                backup_path = self.storage_path.with_suffix(self.storage_path.suffix + ".bak")
                self.storage_path.replace(backup_path)
            except Exception as backup_exc:  # pragma: no cover - best effort
                logging.getLogger(__name__).warning(
                    "Failed to back up corrupted world memory file %s: %s",
                    self.storage_path,
                    backup_exc,
                )
            raw = {}
        self._data = {}
        for world, info in raw.items():
            rules = [WorldRule.from_dict(r) for r in info.get("rules", [])]
            cultures = [CulturalInfo.from_dict(c) for c in info.get("cultures", [])]
            self._data[world] = {"rules": rules, "cultures": cultures}

    # Compatibility with previous API ---------------------------------
    def add(self, name: str, info: Dict[str, Any]) -> None:
        """Add or update information about a world (legacy API)."""
        world_entry = self._data.setdefault(name, {"rules": [], "cultures": []})
        for rule in info.get("rules", []):
            if isinstance(rule, WorldRule):
                world_entry["rules"].append(rule)
            else:
                world_entry["rules"].append(WorldRule(**rule))
        for culture in info.get("cultures", []):
            if isinstance(culture, CulturalInfo):
                world_entry["cultures"].append(culture)
            else:
                world_entry["cultures"].append(CulturalInfo(**culture))


__all__ = ["WorldMemory", "WorldRule", "CulturalInfo"]
