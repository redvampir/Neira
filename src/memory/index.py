from __future__ import annotations

"""Compatibility wrapper exposing Rust-backed memory index."""

from .embedding_memory import EmbeddingMemory as MemoryIndex

__all__ = ["MemoryIndex"]
