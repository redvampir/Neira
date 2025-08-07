from __future__ import annotations

"""Search utilities combining multiple knowledge sources."""

from pathlib import Path
from typing import List, Dict, Any
import json
from concurrent.futures import ThreadPoolExecutor, as_completed

from src.memory import CharacterMemory, WorldMemory, StyleMemory
from src.search import SearchAPIClient
from .plugin_registry import (
    APISearchPlugin,
    get_search_plugins,
    register_search_plugin,
)
from .resource_iterator import ResourceAwareIterator


class DeepSearcher:
    """Search across memories, local files and the web.

    The searcher aggregates results from several sources assigning a rough
    priority to each source so callers can decide which information is most
    relevant.
    """

    def __init__(
        self,
        character_memory: CharacterMemory | None = None,
        world_memory: WorldMemory | None = None,
        style_memory: StyleMemory | None = None,
        api_client: SearchAPIClient | None = None,
        data_path: str | Path | None = None,
        use_default_plugins: bool = True,
        resource_iterator: ResourceAwareIterator | None = None,
        parallel: bool | None = None,
    ) -> None:
        self.character_memory = character_memory or CharacterMemory()
        self.world_memory = world_memory or WorldMemory()
        self.style_memory = style_memory or StyleMemory()
        self.api_client = api_client or SearchAPIClient()
        self.data_path = Path(data_path or "data")
        if use_default_plugins:
            register_search_plugin(APISearchPlugin(self.api_client))

        if parallel is None:
            if resource_iterator is not None:
                parallel = bool(resource_iterator.config.get("parallel", True))
            else:
                parallel = True
        self.parallel = parallel
        self.resource_iterator = resource_iterator

    # ------------------------------------------------------------------
    def search(self, query: str, user_id: str | None = None, limit: int = 5) -> List[Dict[str, Any]]:
        """Return ranked results for ``query`` from all configured sources."""
        results: List[Dict[str, Any]] = []
        q = query.lower()

        # Character memory -------------------------------------------------
        try:
            characters = self.character_memory.get() or {}
            for name, char in characters.items():
                text = json.dumps(char.to_dict()).lower()
                if q in text:
                    results.append(
                        {
                            "source": "character_memory",
                            "reference": str(self.character_memory.storage_path),
                            "content": char.to_dict(),
                            "priority": 1.0,
                        }
                    )
                    break
        except Exception:
            pass

        # World memory -----------------------------------------------------
        try:
            worlds = self.world_memory.get() or {}
            for world, info in worlds.items():
                text = json.dumps({world: info}).lower()
                if q in text:
                    results.append(
                        {
                            "source": "world_memory",
                            "reference": str(self.world_memory.storage_path),
                            "content": {world: info},
                            "priority": 0.9,
                        }
                    )
                    break
        except Exception:
            pass

        # Style memory -----------------------------------------------------
        try:
            styles: Dict[str, Any] = {}
            if user_id:
                styles = self.style_memory.get_style(user_id) or {}
            else:
                for uid in getattr(self.style_memory, "_data", {}):
                    styles.update(self.style_memory.get_style(uid) or {})
            for author, pattern in styles.items():
                text = json.dumps(pattern.to_dict()).lower()
                if q in text:
                    results.append(
                        {
                            "source": "style_memory",
                            "reference": str(self.style_memory.storage_path),
                            "content": {author: pattern.to_dict()},
                            "priority": 0.8,
                        }
                    )
                    break
        except Exception:
            pass

        # Cold storage files -----------------------------------------------
        if self.data_path.exists():
            for file in self.data_path.rglob("*"):
                if not file.is_file():
                    continue
                try:
                    content = file.read_text(encoding="utf-8")
                except Exception:
                    continue
                if q in content.lower():
                    results.append(
                        {
                            "source": "file",
                            "reference": str(file),
                            "content": content,
                            "priority": 0.5,
                        }
                    )
                    break

        # Plugin search ----------------------------------------------------
        plugins = get_search_plugins()
        if self.parallel and len(plugins) > 1:
            with ThreadPoolExecutor(max_workers=len(plugins)) as executor:
                future_to_plugin = {
                    executor.submit(plugin.search, query, limit): plugin
                    for plugin in plugins
                }
                for future in as_completed(future_to_plugin):
                    plugin = future_to_plugin[future]
                    try:
                        for item in future.result():
                            item.setdefault("source", plugin.__class__.__name__)
                            item.setdefault("priority", 0.0)
                            results.append(item)
                    except Exception:
                        continue
        else:
            for plugin in plugins:
                try:
                    for item in plugin.search(query, limit):
                        item.setdefault("source", plugin.__class__.__name__)
                        item.setdefault("priority", 0.0)
                        results.append(item)
                except Exception:
                    continue

        results.sort(key=lambda r: r["priority"], reverse=True)
        return results


__all__ = ["DeepSearcher"]
