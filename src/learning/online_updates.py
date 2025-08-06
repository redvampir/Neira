from __future__ import annotations

import json
from dataclasses import dataclass, field
from typing import Callable, Dict, Iterable, List, Sequence
from urllib.request import urlopen


@dataclass
class OnlineLearningEngine:
    """Fetch and integrate information from online sources.

    The engine can monitor simple JSON based RSS/API feeds, filter spam
    messages and evaluate the quality of the source before integrating new
    items into a :class:`~src.learning.learning_system.LearningSystem` instance.
    """

    sources: Sequence[str]
    spam_keywords: Sequence[str] = field(default_factory=list)
    quality_threshold: float = 0.0
    fetch_func: Callable[[str], Iterable[Dict[str, object]]] | None = None

    # ------------------------------------------------------------------
    def _default_fetch(self, url: str) -> Iterable[Dict[str, object]]:
        """Basic JSON fetcher used when no custom ``fetch_func`` is supplied."""
        with urlopen(url) as fh:  # pragma: no cover - network disabled in tests
            data = fh.read().decode("utf-8")
        try:
            return json.loads(data)
        except json.JSONDecodeError:
            return []

    # ------------------------------------------------------------------
    def _is_spam(self, item: Dict[str, object]) -> bool:
        text = f"{item.get('title', '')} {item.get('content', '')}".lower()
        return any(word.lower() in text for word in self.spam_keywords)

    # ------------------------------------------------------------------
    def _filter_spam(
        self, items: Iterable[Dict[str, object]]
    ) -> List[Dict[str, object]]:
        return [item for item in items if not self._is_spam(item)]

    # ------------------------------------------------------------------
    def _evaluate_source(self, items: Sequence[Dict[str, object]]) -> float:
        if not items:
            return 0.0
        qualities = [float(item.get("quality", 1.0)) for item in items]
        return sum(qualities) / len(qualities)

    # ------------------------------------------------------------------
    def monitor(self) -> List[Dict[str, object]]:
        """Retrieve and filter updates from configured ``sources``."""
        fetcher = self.fetch_func or self._default_fetch
        aggregated: List[Dict[str, object]] = []
        for url in self.sources:
            try:
                items = list(fetcher(url))
            except Exception:
                items = []
            items = self._filter_spam(items)
            if self._evaluate_source(items) >= self.quality_threshold:
                for item in items:
                    item = dict(item)
                    item.setdefault("source", url)
                    aggregated.append(item)
        return aggregated

    # ------------------------------------------------------------------
    def integrate(self, system: "LearningSystem") -> int:
        """Integrate updates into ``system.knowledge_base``.

        Returns
        -------
        int
            Number of records added.
        """
        updates = self.monitor()
        for entry in updates:
            entry = dict(entry)
            entry["update_label"] = "online_update"
            system.knowledge_base.add(entry)
        if updates:
            system.knowledge_base.save()
        return len(updates)


__all__ = ["OnlineLearningEngine"]
