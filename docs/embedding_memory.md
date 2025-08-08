# EmbeddingMemory

`EmbeddingMemory` предоставляет простое хранилище текстовых фрагментов с поиском по смысловой близости.  
Для получения векторных представлений используется библиотека `sentence-transformers`, а сами векторы хранятся в FAISS или Chroma.

```python
from src.memory import MemoryIndex

memory = MemoryIndex(vector_backend="faiss")
memory.set("Берлин столица Германии", True)

# поиск по смыслу
print(memory.similar("Какая столица у Германии?", k=1))
```

Доступны методы:

* `add(text)` – добавить текст;
* `similar(query, k)` – найти `k` похожих текстов;
* `save()` / `load()` – сохранение и загрузка индекса.

Выбор бэкенда выполняется параметром `vector_backend` (``faiss`` или ``chroma``).
