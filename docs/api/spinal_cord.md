<!-- neira:meta
id: NEI-20260427-101500-spinal-api-rename
intent: docs
summary: |
  Переименовали справочник Backend API в "Spinal Cord API" и перенесли файл в `docs/api/`. Старый путь оставлен как редирект.
-->

# Spinal Cord API Quick Reference

## Auth and Scopes

- Auth header: `Authorization: Bearer <token>` or `x-auth-token: <token>`.
- Scopes per token: `read`, `write`, `admin`.
  - `write` required for mutating endpoints (session new/delete/rename/import, chat when writing history).
  - `admin` required for masking updates.
  - In Safe Mode, any write requires `admin`.

## Chat

- POST `/api/neira/chat`
  - Body: `{ cell_id, chat_id, session_id?, message, auth?, persist?, request_id?, source?, thread_id? }`
  - Response: `{ response, used_context, session_id?, idempotent }`
  - Headers: `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Used`, `X-RateLimit-Window=minute`, `X-RateLimit-Key`.
  - Notes: `persist=true` may auto-create session (see `PERSIST_REQUIRE_SESSION_ID`). `request_id` enables idempotency.

- POST `/api/neira/chat/stream`
  - SSE events: `meta` (first), many `message`, periodic `progress`, final `done`.
  - `meta`: `{ used_context, session_id?, idempotent, source?, thread_id?, rate_limit: { limit, remaining, used, window, key }, budget_tokens? }`.
  - Cancel: POST `/api/neira/chat/stream/cancel` with `{ auth, chat_id, session_id }` (requires `write`).
  - Token budget (optional): передайте `budget_tokens` в теле запроса (или используйте ENV `REASONING_TOKEN_BUDGET`). В `progress` периодически приходит `{"budget_remaining": N}`. При достижении нуля поток мягко завершится; метрика `budget_hits_total`.

## Sessions

- POST `/api/neira/chat/session/new` (requires `write`)
- DELETE `/api/neira/chat/:chat_id/:session_id` (requires `write`)
- POST `/api/neira/chat/:chat_id/:session_id/rename` (requires `write`)

## Context Import/Export

- GET `/api/neira/chat/:chat_id/export`
- POST `/api/neira/chat/:chat_id/import/:session_id` (requires `write`)
  - Body: NDJSON of ChatMessage entries.

## Search

- GET `/api/neira/chat/:chat_id/:session_id/search`
  - Params: `q`, `regex=0|1`, `prefix=0|1`, `since_id`, `after_ts`, `offset`, `limit`, `role=user|assistant`, `sort=asc|desc`.
  - Returns NDJSON (ChatMessage). Match is applied to `content` only. Sorted by `timestamp_ms`.

## Masking

- POST `/api/neira/context/masking` (requires `admin`)
  - Body: `{ auth?, enabled?, regex?[], roles?[], preset? }` - `preset` loads regex from files in `MASK_PRESETS_DIR`.
- GET `/api/neira/context/masking/config`
- POST `/api/neira/context/masking/dry_run` with `{ text, regex?[], roles?[] }`

## Metrics

- GET `/metrics` (when `NERVOUS_SYSTEM_ENABLED=true`).
- Notable metrics: `sessions_active` gauge, `sessions_autocreated_total`, `sessions_closed_total`, `requests_idempotent_hits`, `context_bytes_written`, `gz_rotate_count`, `index_compact_runs`, `sse_active` gauge, `safe_mode` gauge.

## Environment (highlights)

- Idempotency: `IDEMPOTENT_PERSIST`, `IDEMPOTENT_STORE_DIR`, `IDEMPOTENT_TTL_SECS`.
- Persist policy: `PERSIST_REQUIRE_SESSION_ID`.
- Index: `INDEX_KW_TTL_DAYS`, `INDEX_COMPACT_INTERVAL_MS`.
- SSE/logging: `SSE_WARN_AFTER_MS`, `NERVOUS_SYSTEM_JSON_LOGS`.
- Masking presets: `MASK_PRESETS_DIR`.

## Control Plane (admin)

- POST `/api/neira/control/pause`
  - Body: `{ auth, reason?, request_id?, drain_active_streams? }`
  - Effect: ставит все новые задания на паузу; опционально «сливает» активные SSE‑стримы (`drain_active_streams=true`).
  - Returns: `{ paused: true, reason, paused_since_ts_ms }`.

- POST `/api/neira/control/resume`
  - Body: `{ auth, request_id? }`
  - Effect: снимает глобальную паузу.
  - Returns: `{ paused: false }`.

- POST `/api/neira/control/kill`
  - Body: `{ auth, grace_ms?, request_id? }`
  - Effect: инициирует graceful shutdown; по таймауту `grace_ms` — принудительный выход.
  - Returns: `{ stopping: true, grace_ms }`.

- GET `/api/neira/control/status`
  - Returns: `{ paused, paused_for_ms, paused_since_ts_ms, reason, active_tasks, backpressure, queues: { fast, standard, long } }`.

- GET `/api/neira/inspect/snapshot`
  - Query: `include=metrics,context,logs`
  - Returns: `{ ok, file }` — путь к JSON‑срезу; при `include=metrics` прикладывает Prometheus‑метрики; при `include=context` — индекс файлов контекста.

- GET `/api/neira/trace/:request_id`
  - При `TRACE_ENABLED=1` возвращает накопленные события по `request_id` (chat/analysis start/done и др.). Объём ограничен `TRACE_MAX_EVENTS`.

Notes
- Все операции требуют `admin` и журналируются (tracing). Гейтятся фичами: `control_pause_resume`, `control_kill_switch`, `inspect_snapshot`, `trace_requests`.

## Очереди и давление (Queues)

- GET `/api/neira/queues/status`
  - Returns: `{ active_streams, backpressure, queues: { fast, standard, long } }`.
  - Используется для UI/дашбордов и внешнего мониторинга.

## Dev‑маршруты (только при `DEV_ROUTES_ENABLED=1` и `auth=admin`)

- GET `/api/neira/dev/stream/long`
  - Длинный детерминированный SSE‑стрим для тестов дренажа.
  - Управление длительностью: `SSE_DEV_DELAY_MS`, `SSE_DEV_TOKENS` (ENV).

- GET `/api/neira/dev/analysis/long?ms=<duration>&auth=<admin>`
  - «Длинный» анализ для проверки watchdog soft/hard, без реальных клеток.

## Snapshot: опции

- GET `/api/neira/inspect/snapshot?include=metrics,context,logs&request_id=<id>&zip=1&level=INFO`
  - Сохраняет JSON и, при `zip=1`, упаковывает в ZIP: `snapshot.json`, `logs-tail.log`, `trace.json` (если `request_id` задан и трассы включены).
  - `logs` — добавляет хвост `logs/backend.log` (маскируется `mask_preview`), фильтр по уровню через `level=`.
