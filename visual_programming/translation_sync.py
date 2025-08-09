"""Utilities for synchronising graph nodes with ``@neyra`` code comments.

The visual programming environment stores nodes inside a graph structure.  Each
node has an English identifier (``id``) used in the generated source code and a
``display`` name shown to the user.  Translation comments in the source code use
the ``@neyra`` format implemented by :class:`src.translation.manager.TranslationManager`.

This module provides helper functions to keep node names and comments in sync,
perform reverse translation when users edit names on the graph and generate
identifiers when only a translated display string is available.  It also supports
importing external files describing nodes and automatically creating the
corresponding graph entries.
"""

from __future__ import annotations

from dataclasses import dataclass, field
import json
import re
from pathlib import Path
from typing import Any, Dict, List, Tuple

from src.translation.manager import TranslationManager


NEYRA_RE = re.compile(r"@neyra:(?:visual_block|var) id=\"(?P<id>[^\"]+)\" display=\"(?P<display>[^\"]+)\"")


@dataclass
class TranslationSync:
    """Synchronise node names with ``@neyra`` comments.

    Parameters
    ----------
    lang:
        The target language for translations.  Defaults to ``"en"``.
    manager:
        Optional :class:`TranslationManager` instance.  When omitted a new one is
        created for every :class:`TranslationSync` instance.
    """

    lang: str = "en"
    manager: TranslationManager = field(default_factory=TranslationManager)

    # ------------------------------------------------------------------ Graph ↔ Code
    def sync(self, code: str, graph: Dict[str, Any]) -> Tuple[str, Dict[str, Any]]:
        """Synchronise ``graph`` node names with ``code`` comments.

        The operation is bidirectional:

        * ``graph`` → ``code``: ``@neyra`` comments are inserted or updated based
          on node display names.
        * ``code`` → ``graph``: node display names are refreshed from existing
          comments.
        * Missing identifiers are generated from their display names ensuring an
          English ``id`` is always present.
        """

        # Update code comments from graph nodes ------------------------- Graph → Code
        dictionary = {
            node["id"]: node.get("display", "")
            for node in graph.get("nodes", [])
            if node.get("id") and node.get("display")
        }
        self.manager.dictionary = dictionary
        code = self.manager.annotate_source(code, self.lang)

        # Refresh graph names from code comments ------------------------ Code → Graph
        nodes_by_id = {node.get("id", ""): node for node in graph.get("nodes", [])}
        for match in NEYRA_RE.finditer(code):
            node_id = match.group("id")
            display = match.group("display")
            node = nodes_by_id.get(node_id)
            if node is None:
                continue
            node["display"] = display
            i18n = node.setdefault("i18n", {})
            i18n[self.lang] = display

        # Generate missing identifiers or display names ----------------- Reverse translation
        for node in graph.get("nodes", []):
            if node.get("display") and not node.get("id"):
                node["id"] = self.manager.generate_name(node["display"])
            elif node.get("id") and not node.get("display"):
                display = self.manager.reverse_translate_name(node["id"])
                node["display"] = display
                node.setdefault("i18n", {})[self.lang] = display

        return code, graph

    # ------------------------------------------------------------------ External import
    def import_file(self, path: str | Path) -> List[Dict[str, Any]]:
        """Import nodes from an external ``path``.

        ``path`` must point to a JSON file containing either a mapping of
        ``identifier`` → ``display`` or a list where each element is either a
        string (display name) or a mapping with at least a ``display`` key.  For
        every entry a node dictionary with generated identifier and ``i18n``
        information is returned.
        """

        data = json.loads(Path(path).read_text(encoding="utf8"))
        nodes: List[Dict[str, Any]] = []

        def handle(display: str, identifier: str | None = None) -> None:
            display = display.strip()
            if not identifier:
                identifier = self.manager.generate_name(display)
            nodes.append({
                "id": identifier,
                "display": display,
                "i18n": {self.lang: display},
            })

        if isinstance(data, dict):
            for identifier, display in data.items():
                handle(str(display), str(identifier))
        elif isinstance(data, list):
            for item in data:
                if isinstance(item, str):
                    handle(item)
                elif isinstance(item, dict):
                    display = str(item.get("display") or item.get("name") or "")
                    if display:
                        handle(display, item.get("id"))
        else:
            raise ValueError("Unsupported file format for node import")

        return nodes


__all__ = ["TranslationSync"]
