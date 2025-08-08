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

## Smart cache tiers and cleanup

`SmartCache` builds on top of `CacheManager` and introduces three storage
levels:

- **hot** – in‑memory entries for very frequent requests;
- **warm** – uncompressed JSON files on disk;
- **cold** – compressed archives kept in `cache_dir/archive`.

Access patterns are tracked in `access_history`.  The history is smoothed with
an exponential moving average which is then used to prefetch and promote keys
to the hot tier when repeated requests are predicted.

`cleanup()` removes entries whose TTL has expired and, when configured via the
``stale_after`` option, keys that have not been accessed for a long period of
time.  The cleanup can be scheduled automatically by passing
``cleanup_interval`` when constructing `SmartCache`.
