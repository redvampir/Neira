"""Utilities for managing bidirectional links between document nodes."""

from __future__ import annotations

from collections import defaultdict
from typing import Dict, Iterable, List, Set


class LinkManager:
    """Maintain bidirectional links between objects.

    The manager tracks forward links (``source -> targets``) and
    backward links (``target -> sources``). Adding or removing a link
    automatically updates both directions so that lookups stay in sync.
    """

    def __init__(self) -> None:
        self._forward: Dict[str, Set[str]] = defaultdict(set)
        self._backward: Dict[str, Set[str]] = defaultdict(set)

    # ------------------------------------------------------------------
    def add_link(self, source: str, target: str) -> None:
        """Create a link from ``source`` to ``target``.

        The relationship is stored in both directions allowing callers to
        later query either side.
        """

        self._forward[source].add(target)
        self._backward[target].add(source)

    # ------------------------------------------------------------------
    def remove_link(self, source: str, target: str) -> None:
        """Remove an existing link.

        Both the forward and backward mappings are updated. If removing the
        link leaves a node without any connections, its entry is deleted to
        keep the structure compact.
        """

        self._forward[source].discard(target)
        self._backward[target].discard(source)

        if not self._forward[source]:
            del self._forward[source]
        if not self._backward[target]:
            del self._backward[target]

    # ------------------------------------------------------------------
    def get_targets(self, source: str) -> List[str]:
        """Return a sorted list of nodes linked from ``source``."""

        return sorted(self._forward.get(source, set()))

    # ------------------------------------------------------------------
    def get_sources(self, target: str) -> List[str]:
        """Return a sorted list of nodes linking to ``target``."""

        return sorted(self._backward.get(target, set()))

    # ------------------------------------------------------------------
    def clear(self) -> None:
        """Remove all links."""

        self._forward.clear()
        self._backward.clear()

    # ------------------------------------------------------------------
    def iter_links(self) -> Iterable[tuple[str, str]]:
        """Iterate over all stored links as ``(source, target)`` pairs."""

        for src, targets in self._forward.items():
            for tgt in targets:
                yield src, tgt
