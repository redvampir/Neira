from __future__ import annotations

"""Rust-backed vector memory using sentence-transformers."""

from pathlib import Path
from typing import List

from sentence_transformers import SentenceTransformer
from neira_rust import MemoryIndex as RustMemoryIndex


class EmbeddingMemory:
    """Store and retrieve text snippets via a Rust index."""

    def __init__(
        self,
        storage_path: str | Path = "data/embeddings",
        model_name: str = "sentence-transformers/all-MiniLM-L6-v2",
    ) -> None:
        self.storage_path = Path(storage_path)
        self.model = SentenceTransformer(model_name)
        self.index = RustMemoryIndex()
        self.load()

    def add(self, text: str) -> None:
        embedding = self.model.encode([text])[0].tolist()
        self.index.add(text, embedding)

    def similar(self, query: str, k: int = 5) -> List[str]:
        embedding = self.model.encode([query])[0].tolist()
        return self.index.similar(embedding, k)

    def save(self) -> None:
        self.storage_path.mkdir(parents=True, exist_ok=True)
        self.index.save(str(self.storage_path / "memory.bin"))

    def load(self) -> None:
        file = self.storage_path / "memory.bin"
        if file.exists():
            self.index.load(str(file))


__all__ = ["EmbeddingMemory"]
