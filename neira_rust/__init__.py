"""Fallback Python implementations for the optional Rust extension."""

from __future__ import annotations

from dataclasses import dataclass
from typing import List, Tuple


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


@dataclass
class VerificationResult:
    claim: str
    verified: bool


def verify_claim(claim: str) -> VerificationResult:  # pragma: no cover - stub
    return VerificationResult(claim=claim, verified=True)


__all__ = ["ping", "MemoryIndex"]
