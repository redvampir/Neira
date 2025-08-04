# Cache invalidation

`CacheManager` stores JSON files inside the `.cache` directory and mirrors the
content in memory. It is used for:

- loading books (`Neyra.load_book`)
- analysing books (`memory.knowledge_base.analyze_book`)
- generating scenes (`Neyra._create_scene`)

## Invalidation rules

- Entries related to book files (`load_book` and `analyze_book`) include the
  file modification time. If the file on disk changes, the cached value is
  ignored and recomputed automatically.
- Scene generation is cached by the description string and persists until
  explicitly cleared.
- Use `CacheManager.invalidate(key)` to drop a specific entry or
  `CacheManager.invalidate()` to clear the whole cache.

All cache files live in the project root under `.cache/`.
