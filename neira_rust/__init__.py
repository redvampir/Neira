"""Fallback Python implementations for the optional Rust extension."""

from __future__ import annotations

from dataclasses import dataclass
from typing import List, Tuple, Any


# Minimal tag structure used by TagProcessor
@dataclass
class Tag:
    type: str = ""
    subject: str = ""
    commands: List[str] | None = None


def parse(text: str) -> List[Tag]:  # pragma: no cover - stub
    """Return an empty list representing parsed tags."""
    return []


def suggest_entities(prefix: str) -> List[str]:  # pragma: no cover - stub
    return []


def ping() -> str:  # pragma: no cover - simple stub
    return "pong"


class MemoryIndex:
    """Very small in-memory replacement for the Rust ``MemoryIndex``.

    The real project provides a fast Rust implementation.  For the unit tests in
    this kata we only need a handful of methods, so a minimal Python version is
    supplied.  It stores embeddings alongside their original text and performs a
    naive similarity search using dot products.
    """

    def __init__(self) -> None:
        self._data: List[Tuple[str, List[float]]] = []

    def add(self, text: str, embedding: List[float]) -> None:
        self._data.append((text, embedding))

    def similar(self, embedding: List[float], k: int) -> List[str]:
        def _score(item: Tuple[str, List[float]]) -> float:
            other = item[1]
            return sum(a * b for a, b in zip(embedding, other))

        return [t for t, _ in sorted(self._data, key=_score, reverse=True)[:k]]

    def save(self, path: str) -> None:  # pragma: no cover - stub
        pass

    def load(self, path: str) -> None:  # pragma: no cover - stub
        pass


class KnowledgeGraph:
    """Lightweight stand-in for the Rust ``KnowledgeGraph``.

    It stores simple triples in memory and exposes the minimal API used by the
    Python codebase.  The real project offers a much richer implementation but
    for tests we only need to ensure imports succeed and method calls do not
    fail."""

    def __init__(self) -> None:
        self.facts: List[Tuple[str, str, str]] = []

    def add_fact(self, subject: str, relation: str, obj: str) -> None:
        self.facts.append((subject, relation, obj))

    def check_claim(self, claim: str) -> bool:  # pragma: no cover - trivial
        return True


@dataclass
class VerificationResult:
    claim: str
    verified: bool


def verify_claim(claim: str) -> VerificationResult:  # pragma: no cover - stub
    return VerificationResult(claim=claim, verified=True)


__all__ = ["ping", "MemoryIndex", "KnowledgeGraph"]
