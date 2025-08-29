<!-- neira:meta
id: NEI-20250830-Docs-API-Backend-Move
intent: docs
summary: |
  Переместили справочник Backend API в раздел `docs/api/` и добавили якорь для единой системы навигации. Старый путь оставлен как редирект.
-->

# Backend API Quick Reference

## Auth and Scopes

- Auth header: `Authorization: Bearer <token>` or `x-auth-token: <token>`.
- Scopes per token: `read`, `write`, `admin`.
  - `write` required for mutating endpoints (session new/delete/rename/import, chat when writing history).
  - `admin` required for masking updates.
  - In Safe Mode, any write requires `admin`.

## Chat

- POST `/api/neira/chat`
  - Body: `{ node_id, chat_id, session_id?, message, auth?, persist?, request_id?, source?, thread_id? }`
  - Response: `{ response, used_context, session_id?, idempotent }`
  - Headers: `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Used`, `X-RateLimit-Window=minute`, `X-RateLimit-Key`.
  - Notes: `persist=true` may auto-create session (see `PERSIST_REQUIRE_SESSION_ID`). `request_id` enables idempotency.

- POST `/api/neira/chat/stream`
  - SSE events: `meta` (first), many `message`, periodic `progress`, final `done`.
  - `meta`: `{ used_context, session_id?, idempotent, source?, thread_id?, rate_limit: { limit, remaining, used, window, key } }`.
  - Cancel: POST `/api/neira/chat/stream/cancel` with `{ auth, chat_id, session_id }` (requires `write`).

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

