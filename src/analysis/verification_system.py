"""Python adapter for Rust claim verification."""

from __future__ import annotations
from typing import Iterable, Dict, Callable

from neira_rust import VerificationResult, verify_claim as _verify_claim


def verify_claim(claim: str, context: Iterable[str] | None = None) -> VerificationResult:
    return _verify_claim(claim, list(context or []))


class VerificationSystem:
    def verify_claim(self, claim: str, context: Iterable[str] | None = None) -> VerificationResult:
        return _verify_claim(claim, list(context or []))


def verify_fact(
    fact: str,
    search_func: Callable[[str, int], Iterable[Dict[str, str]]] | None = None,
    limit: int = 3,
) -> bool:
    search_func = search_func or (lambda q, limit=limit: [])
    try:
        results = search_func(fact, limit)
    except Exception:
        return False
    for result in results:
        snippet = result.get("snippet", "")
        if fact.lower() in snippet.lower():
            return True
    return False


__all__ = ["VerificationSystem", "VerificationResult", "verify_fact", "verify_claim"]
