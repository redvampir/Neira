# Commenting & Metadata Guide (neira:meta)

<!-- neira:meta
id: NEI-20250829-174731-simplified-block
intent: docs
summary: |
  Добавлен раздел упрощённого блока, уточнены критерии и ссылка на META_COVERAGE.md.
-->
<!-- neira:meta
id: NEI-20260413-commenting-rename
intent: docs
summary: Обновлён пример scope для каталога spinal_cord.
-->

См. [META_COVERAGE.md](META_COVERAGE.md) для определения, когда использовать полный или упрощённый блок.

Simplified block
- Назначение: минимальный метаблок для некритичных правок.
- Обязательные поля:
  - `id`
  - `intent`
  - `summary`
- Используйте при:
  - косметических правках
  - маленьких вспомогательных файлах
  - примерах

Пример минимального блока (Rust/JS/TS)
```rust
/* neira:meta
id: NEI-YYYYMMDD-HHMMSS-example
intent: docs
summary: |
  Короткое описание изменения.
*/
```

Purpose
- Provide lightweight, machine-readable metadata for significant changes.
- Make it easy to scan and parse context by searching for a stable marker.

When to add
- Add a neira:meta block only for notable changes:
  - New/changed endpoints, auth/safe-mode/permissions
  - New/changed env vars or metrics
  - Data format/schema/migration
  - Security or performance critical changes
- Skip for trivial refactors, typo fixes, formatting.
- Упрощённый блок не освобождает от полного шаблона при существенных изменениях.

Where to add
- Prefer a single block per touched file (top of file) OR
- Immediately above the primary changed function/class.

Format
- Use a block comment with the stable marker `neira:meta`.
- Keys in English (stable), free text in Russian (owner-friendly).
- Line width ≤ 120 chars.

Template (Rust / JS / TS block comment)
/* neira:meta
id: NEI-YYYYMMDD-HHMMSS-<slug>
intent: feature|fix|refactor|docs|perf|security|chore
scope: spinal_cord/<area>
summary: |
  Коротко по-русски: что изменили и зачем (1–2 строки).
links:
  - docs/api/spinal_cord.md#section
  - DECISIONS.md#ADR-XXX
env:
  - SOME_ENV_FLAG
metrics:
  - some_metric_name
endpoints:
  - POST /api/neira/chat/stream
feature_gate:
  name: training_pipeline
  state: locked|experimental|stable|deprecated
risks: low|medium|high
safe_mode:
  affects_write: true|false
  requires_admin: true|false
i18n:
  reviewer_note: |
    1–2 короткие фразы для владельца (по-русски).
*/

Conventions
- One block per file per change; update the same block on follow-ups.
- Keep values concise; use relative links to repo files.
- Prefer consistent ids (timestamp + short slug).

Discovery
- Search all metadata blocks: `rg -n "neira:meta"`

Owner-facing notes
- Separate, human-only comments are optional; keep code clean.
- In handover, add 2–3 bullets in Russian: что сделано, зачем, как проверить.
