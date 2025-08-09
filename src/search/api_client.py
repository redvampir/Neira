from __future__ import annotations

"""Simple client for querying external search services and updating knowledge."""

from typing import Callable, Iterable, List, Dict
import requests
import re
import json
from pathlib import Path
from urllib.parse import urlparse
import yaml

from src.core.security import require_permission

from src.memory import MemoryIndex
from src.utils.spam_filter import is_spam
from src.utils.pii import redact_pii
from src.utils.lang_quality import detect_language, quality_score


class SearchAPIClient:
    """Interact with search APIs and maintain reliability metrics."""

    def __init__(
        self,
        memory: MemoryIndex | None = None,
        fetcher: Callable[[str, int], Iterable[Dict[str, str]]] | None = None,
        domain_config_path: str | Path | None = None,
        license_config_path: str | Path | None = None,
        lang_quality_config_path: str | Path | None = None,
        token: str = "public",
    ) -> None:
        """
        Parameters
        ----------
        memory:
            Optional memory index used to store extracted facts and reliability
            information.
        fetcher:
            Callable performing the actual HTTP request. It receives the query and
            limit and must return an iterable of search result dictionaries. Each
            result dictionary should contain ``url`` and ``snippet`` keys. When not
            provided, a minimal DuckDuckGo JSON API is used.
        domain_config_path:
            Optional path to a JSON file containing ``allowed_domains`` and
            ``blocked_domains`` lists used to filter search results.
        license_config_path:
            Optional path to a YAML file specifying ``allowed_licenses`` used to
            filter sources based on licensing information.
        lang_quality_config_path:
            Optional path to a YAML file defining ``allowed_languages`` and a
            ``min_quality`` threshold for extracted facts.
        """
        self.memory = memory or MemoryIndex()
        # Ensure ``memory`` exposes the minimal attributes used in tests.  Some
        # lightweight implementations may not provide these, so we create them on
        # the fly when missing.
        if not hasattr(self.memory, "cold_storage"):
            self.memory.cold_storage = {}
        if not hasattr(self.memory, "source_reliability"):
            self.memory.source_reliability = {}

        if not hasattr(self.memory, "set"):
            def _set(key: str, value: str, reliability: float = 0.5) -> None:
                self.memory.cold_storage[key] = value
                self.memory.source_reliability[key] = reliability

            self.memory.set = _set  # type: ignore[attr-defined]

        self.fetcher = fetcher or self._duckduckgo_fetch
        self.session = requests.Session()
        self.allowed_domains, self.blocked_domains = self._load_domain_config(
            domain_config_path
        )
        self.token = token
        self.allowed_licenses = self._load_license_config(license_config_path)
        (
            self.allowed_languages,
            self.quality_threshold,
        ) = self._load_lang_quality_config(lang_quality_config_path)

    # ------------------------------------------------------------------
    def _load_domain_config(
        self, path: str | Path | None
    ) -> tuple[set[str], set[str]]:
        """Load allowed and blocked domains from config file."""
        config_path = (
            Path(path)
            if path is not None
            else Path(__file__).resolve().parents[2]
            / "config"
            / "search_domains.json"
        )
        try:
            with config_path.open("r", encoding="utf-8") as f:
                data = json.load(f)
        except Exception:  # pragma: no cover - missing or invalid config
            data = {}
        allowed = {d.lower() for d in data.get("allowed_domains", [])}
        blocked = {d.lower() for d in data.get("blocked_domains", [])}
        return allowed, blocked

    # ------------------------------------------------------------------
    def _load_license_config(self, path: str | Path | None) -> set[str]:
        """Load allowed licenses from config file."""
        if path is None:
            return set()
        config_path = Path(path)
        try:
            with config_path.open("r", encoding="utf-8") as f:
                data = yaml.safe_load(f) or {}
        except Exception:  # pragma: no cover - missing or invalid config
            data = {}
        return set(data.get("allowed_licenses", []))

    # ------------------------------------------------------------------
    def _load_lang_quality_config(
        self, path: str | Path | None
    ) -> tuple[set[str], float]:
        """Load allowed languages and quality threshold."""
        config_path = (
            Path(path)
            if path is not None
            else Path(__file__).resolve().parents[2]
            / "config"
            / "lang_quality.yml"
        )
        try:
            with config_path.open("r", encoding="utf-8") as f:
                data = yaml.safe_load(f) or {}
        except Exception:  # pragma: no cover - missing or invalid config
            data = {}
        allowed = set(data.get("allowed_languages", []))
        threshold = float(data.get("min_quality", 0.0))
        return allowed, threshold

    # ------------------------------------------------------------------
    def _duckduckgo_fetch(self, query: str, limit: int) -> Iterable[Dict[str, str]]:
        """Fetch results from DuckDuckGo.

        This method is intentionally very small and only parses the parts used in
        unit tests. Network errors are swallowed and an empty list is returned.
        """
        try:
            resp = self.session.get(
                "https://duckduckgo.com/?q=" + query + "&format=json&no_redirect=1"
            )
            resp.raise_for_status()
            data = resp.json()
            results = []
            for item in data.get("RelatedTopics", [])[:limit]:
                if isinstance(item, dict) and item.get("Text") and item.get("FirstURL"):
                    results.append({"url": item["FirstURL"], "snippet": item["Text"]})
            return results
        except Exception:  # pragma: no cover - best effort network call
            return []

    # ------------------------------------------------------------------
    def search(self, query: str, limit: int = 5) -> List[Dict[str, str]]:
        """Return search results ranked by source reliability."""
        require_permission(self.token, "external.search")
        raw_results = list(self.fetcher(query, limit))
        filtered: List[Dict[str, str]] = []
        for result in raw_results:
            url = result.get("url", "")
            domain = urlparse(url).netloc.lower()
            if domain.startswith("www."):
                domain = domain[4:]
            if (self.allowed_domains and domain not in self.allowed_domains) or domain in self.blocked_domains:
                continue
            filtered.append(result)
        filtered.sort(
            key=lambda r: self.memory.source_reliability.get(r.get("url", ""), 0.5),
            reverse=True,
        )
        return filtered

    # ------------------------------------------------------------------
    def extract_facts(self, text: str) -> List[str]:
        """Naively extract facts by splitting text into sentences."""
        sentences = re.split(r"[\.!?]+", text)
        return [s.strip() for s in sentences if s.strip()]

    # ------------------------------------------------------------------
    def check_license(self, url: str) -> bool:
        """Verify that the page at ``url`` uses an allowed license."""
        if not self.allowed_licenses:
            return True
        try:
            resp = self.session.get(url, timeout=5)
        except Exception:  # pragma: no cover - network errors
            return False
        license_val = resp.headers.get("License") or resp.headers.get("X-License")
        if not license_val:
            meta = re.search(
                r'<meta[^>]+name=["\']license["\'][^>]*content=["\']([^"\']+)["\']',
                resp.text,
                re.IGNORECASE,
            )
            if not meta:
                meta = re.search(
                    r'<meta[^>]+content=["\']([^"\']+)["\'][^>]*name=["\']license["\']',
                    resp.text,
                    re.IGNORECASE,
                )
            if meta:
                license_val = meta.group(1)
        return bool(license_val and license_val in self.allowed_licenses)

    # ------------------------------------------------------------------
    def search_and_update(self, query: str, limit: int = 5) -> List[Dict[str, str]]:
        """Perform a search and update memory with extracted facts."""
        # Import lazily to avoid heavy optional dependencies during module import.
        from src.analysis.verification_system import verify_fact

        results = self.search(query, limit)
        for result in results:
            url = result.get("url", "")
            snippet = result.get("snippet", "")
            if is_spam(snippet):
                continue
            if not self.check_license(url):
                continue
            reliability = self.memory.source_reliability.get(url, 0.5)
            for fact in self.extract_facts(snippet):
                if self.allowed_languages and detect_language(fact) not in self.allowed_languages:
                    continue
                if quality_score(fact) < self.quality_threshold:
                    continue
                original_fact = fact
                fact = redact_pii(fact)
                # Only store the fact if independent search confirms it.
                if verify_fact(original_fact, search_func=self.search):
                    # Store the redacted fact so that the memory index can perform
                    # deduplication based on content rather than the constant
                    # ``True`` placeholder.
                    self.memory.set(fact, fact, reliability=reliability)
            # ``MemoryIndex.update_reliability`` only updates existing keys, so we
            # modify the reliability mapping directly to ensure the source is
            # tracked even if it wasn't previously stored.
            self.memory.source_reliability[url] = min(1.0, reliability + 0.1)
        return results


__all__ = ["SearchAPIClient"]
