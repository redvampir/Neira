"""Simple CLI for managing local resources.

Resources are regular files stored in a directory.  Metadata including
arbitrary attributes and tags are persisted in ``index.db`` (SQLite).  The
manager provides helper methods used by tests and a minimal interactive
``run`` method that exposes listing, searching and importing functionality.
"""

from __future__ import annotations

import json
import shutil
import sqlite3
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict, List


@dataclass
class Resource:
    """Representation of a single resource entry."""

    name: str
    path: Path
    tags: List[str]
    metadata: Dict[str, Any]


class ResourceIndex:
    """Persistence layer storing resource metadata in SQLite."""

    def __init__(self, db_path: Path) -> None:
        self.db_path = db_path
        self.db_path.parent.mkdir(parents=True, exist_ok=True)
        self._init_db()

    # ------------------------------------------------------------------
    def _init_db(self) -> None:
        with sqlite3.connect(self.db_path) as conn:
            conn.execute(
                """
                CREATE TABLE IF NOT EXISTS resources (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT,
                    path TEXT,
                    tags TEXT,
                    metadata TEXT
                )
                """
            )

    # ------------------------------------------------------------------
    def add(self, resource: Resource) -> None:
        tags = ",".join(resource.tags)
        meta = json.dumps(resource.metadata, ensure_ascii=False)
        with sqlite3.connect(self.db_path) as conn:
            conn.execute(
                "INSERT INTO resources (name, path, tags, metadata) VALUES (?, ?, ?, ?)",
                (resource.name, str(resource.path), tags, meta),
            )

    # ------------------------------------------------------------------
    def list(self) -> List[Resource]:
        with sqlite3.connect(self.db_path) as conn:
            rows = conn.execute(
                "SELECT name, path, tags, metadata FROM resources ORDER BY name"
            ).fetchall()
        result: List[Resource] = []
        for name, path, tags, meta in rows:
            tag_list = [t for t in (tags or "").split(",") if t]
            metadata = json.loads(meta) if meta else {}
            result.append(Resource(name=name, path=Path(path), tags=tag_list, metadata=metadata))
        return result

    # ------------------------------------------------------------------
    def search(self, term: str) -> List[Resource]:
        like = f"%{term}%"
        with sqlite3.connect(self.db_path) as conn:
            rows = conn.execute(
                """
                SELECT name, path, tags, metadata FROM resources
                WHERE name LIKE ? OR tags LIKE ?
                ORDER BY name
                """,
                (like, like),
            ).fetchall()
        result: List[Resource] = []
        for name, path, tags, meta in rows:
            tag_list = [t for t in (tags or "").split(",") if t]
            metadata = json.loads(meta) if meta else {}
            result.append(Resource(name=name, path=Path(path), tags=tag_list, metadata=metadata))
        return result


class ResourceManagerMode:
    """Manage resources stored locally with metadata and tags."""

    def __init__(self, root: Path | str = Path("resources")) -> None:
        self.root = Path(root)
        self.root.mkdir(parents=True, exist_ok=True)
        self.index = ResourceIndex(self.root / "index.db")

    # ------------------------------------------------------------------
    def import_resource(
        self,
        source: Path,
        tags: List[str] | None = None,
        metadata: Dict[str, Any] | None = None,
    ) -> Resource:
        """Copy ``source`` into the resource directory and index it."""

        destination = self.root / source.name
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy(source, destination)
        resource = Resource(
            name=source.name,
            path=destination,
            tags=tags or [],
            metadata=metadata or {},
        )
        self.index.add(resource)
        return resource

    # ------------------------------------------------------------------
    def list_resources(self) -> List[Resource]:
        return self.index.list()

    def search(self, term: str) -> List[Resource]:
        return self.index.search(term)

    # ------------------------------------------------------------------
    def run(self) -> None:  # pragma: no cover - interactive helper
        print("Resource Manager")
        while True:
            cmd = input("(l)ist, (s)earch, (i)mport, (q)uit: ").strip().lower()
            if cmd == "l":
                for res in self.list_resources():
                    print(f"{res.name} [{', '.join(res.tags)}]")
            elif cmd == "s":
                term = input("Search term: ").strip()
                for res in self.search(term):
                    print(f"{res.name} [{', '.join(res.tags)}]")
            elif cmd == "i":
                path = Path(input("Path to file: ").strip())
                tags = [t.strip() for t in input("Tags (comma separated): ").split(",") if t.strip()]
                self.import_resource(path, tags)
                print("Imported")
            elif cmd == "q":
                break
            else:
                print("Unknown command")


__all__ = ["ResourceManagerMode", "Resource", "ResourceIndex"]
