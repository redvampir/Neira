from __future__ import annotations

"""Vector based memory powered by sentence-transformers.

This module provides a tiny wrapper around ``sentence-transformers`` and a
vector database (FAISS or Chroma) so that pieces of text can be stored and
queried by semantic similarity.  The interface intentionally mirrors the
minimal behaviour of :class:`~src.memory.character_memory.CharacterMemory` and
only exposes a handful of methods used throughout the project.
"""

from pathlib import Path
import json
from typing import List

from sentence_transformers import SentenceTransformer

try:  # pragma: no cover - optional backend
    import faiss  # type: ignore
except Exception:  # pragma: no cover - missing dependency
    faiss = None  # type: ignore

try:  # pragma: no cover - optional backend
    import chromadb  # type: ignore
except Exception:  # pragma: no cover - missing dependency
    chromadb = None  # type: ignore


class EmbeddingMemory:
    """Store and retrieve short text snippets using vector similarity."""

    def __init__(
        self,
        storage_path: str | Path = "data/embeddings",
        model_name: str = "sentence-transformers/all-MiniLM-L6-v2",
        backend: str = "faiss",
    ) -> None:
        self.storage_path = Path(storage_path)
        self.model = SentenceTransformer(model_name)
        self.backend = backend
        self.texts: List[str] = []
        self.index = None
        self._collection = None
        self._client = None
        self.load()

    # ------------------------------------------------------------------
    def add(self, text: str) -> None:
        """Add ``text`` to the vector store."""
        embedding = self.model.encode([text])
        if self.backend == "faiss" and faiss is not None:
            import numpy as np

            vector = np.array(embedding, dtype="float32")
            if self.index is None:
                self.index = faiss.IndexFlatL2(vector.shape[1])
            self.index.add(vector)
            self.texts.append(text)
        elif self.backend == "chroma" and chromadb is not None:
            if self._collection is None:
                self._client = chromadb.PersistentClient(path=str(self.storage_path))
                self._collection = self._client.get_or_create_collection("memory")
            self._collection.add(documents=[text])
        else:  # Fallback to simple list if backend missing
            self.texts.append(text)

    # ------------------------------------------------------------------
    def similar(self, query: str, k: int = 5) -> List[str]:
        """Return ``k`` texts most similar to ``query``."""
        embedding = self.model.encode([query])
        if self.backend == "faiss" and faiss is not None and self.index is not None:
            import numpy as np

            vector = np.array(embedding, dtype="float32")
            if self.index.ntotal == 0:
                return []
            distances, indices = self.index.search(vector, k)
            return [self.texts[i] for i in indices[0] if i < len(self.texts)]
        if (
            self.backend == "chroma"
            and chromadb is not None
            and self._collection is not None
        ):
            result = self._collection.query(query_texts=[query], n_results=k)
            return result.get("documents", [[]])[0]
        # Fallback: cosine similarity in Python
        from numpy.linalg import norm
        import numpy as np

        if not self.texts:
            return []
        vecs = self.model.encode(self.texts)
        q = embedding[0]
        sims = [float(q @ v / (norm(q) * norm(v))) for v in vecs]
        topk = sorted(zip(self.texts, sims), key=lambda x: x[1], reverse=True)
        return [t for t, _ in topk[:k]]

    # ------------------------------------------------------------------
    def save(self) -> None:
        """Persist the vector store to disk."""
        self.storage_path.mkdir(parents=True, exist_ok=True)
        if self.backend == "faiss" and faiss is not None and self.index is not None:
            faiss.write_index(self.index, str(self.storage_path / "index.faiss"))
            (self.storage_path / "texts.json").write_text(
                json.dumps(self.texts, ensure_ascii=False), encoding="utf-8"
            )
        elif (
            self.backend == "chroma"
            and chromadb is not None
            and self._client is not None
        ):
            self._client.persist()
        else:  # simple list backend
            (self.storage_path / "texts.json").write_text(
                json.dumps(self.texts, ensure_ascii=False), encoding="utf-8"
            )

    # ------------------------------------------------------------------
    def load(self) -> None:
        """Load previously persisted vectors if available."""
        if self.backend == "faiss" and faiss is not None:
            index_file = self.storage_path / "index.faiss"
            texts_file = self.storage_path / "texts.json"
            if index_file.exists():
                self.index = faiss.read_index(str(index_file))
            if texts_file.exists():
                self.texts = json.loads(texts_file.read_text(encoding="utf-8"))
        elif self.backend == "chroma" and chromadb is not None:
            self._client = chromadb.PersistentClient(path=str(self.storage_path))
            self._collection = self._client.get_or_create_collection("memory")
        else:
            texts_file = self.storage_path / "texts.json"
            if texts_file.exists():
                self.texts = json.loads(texts_file.read_text(encoding="utf-8"))


__all__ = ["EmbeddingMemory"]
