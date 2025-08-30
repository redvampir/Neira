<!-- neira:meta
id: NEI-20250915-adaptive-storage-backend-env
intent: docs
summary: Контекстное хранилище теперь подбирает лимиты по диску; переменные можно переопределить.
-->

<!-- neira:meta
id: NEI-20250922-analysis-queue-env
intent: docs
summary: Добавлены переменные управления порогами очередей анализа.
-->

Backend environment variables

Note
- Источником истины по переменным окружения является `docs/reference/env.md`.
- Этот файл сохраняет обзор и пример `.env`, но при расхождениях доверяйте справочнику.

- CONTEXT_DIR: base dir for chat history (default: context)
- CONTEXT_MAX_LINES: max lines kept in a file before trimming (default: adaptive via storage_metrics.json)
- CONTEXT_MAX_BYTES: max bytes per file before trimming (default: adaptive via storage_metrics.json)
- CONTEXT_DAILY_ROTATION: if true, rotate files daily with -YYYYMMDD suffix (default: true)
- CONTEXT_ARCHIVE_GZ: if true, gzip previous days’ files (default: true)
- CONTEXT_FLUSH_MS: buffered write flush interval in ms; 0 disables buffering (default: 0)
- MASK_PII: enable masking of PII like emails/phones (default: true)
- MASK_REGEX: semicolon-separated custom regex to mask (default: empty)
- MASK_ROLES: comma-separated roles to mask: user,assistant,system (default: user)
- CHAT_RATE_LIMIT_PER_MIN: max chat requests per minute per key (default: 120)
- CHAT_RATE_KEY: rate limit key, either "auth" or "chat" (default: auth)
- IO_WATCHER_THRESHOLD_MS: latency threshold in ms for triggering diagnostics (default: 100)
- NERVOUS_SYSTEM_ENABLED: enable Prometheus metrics and nervous system (default: true)
- PROBES_HOST_METRICS_ENABLED: enable host metrics collection (default: true)
- PROBES_IO_WATCHER_ENABLED: enable keyboard/display latency watcher (default: false, deprecated alias: IO_WATCHER_ENABLED)
 - ANALYSIS_QUEUE_FAST_MS: override boundary between fast and standard analysis queues in ms (default: adaptive)
 - ANALYSIS_QUEUE_LONG_MS: override boundary between standard and long analysis queues in ms (default: adaptive)
 - ANALYSIS_QUEUE_RECALC_MIN: number of new analysis requests before thresholds recompute (default: 100)
- INTEGRITY_ROOT: base dir for integrity config and files (default: current working directory; set explicitly if the service runs outside `backend/`)
- INTEGRITY_CONFIG_PATH: path to integrity config file relative to INTEGRITY_ROOT or absolute (default: config/integrity.json)
- INTEGRITY_CHECK_INTERVAL_MS: integrity check interval in ms (default: 60000)

When `CONTEXT_MAX_LINES` or `CONTEXT_MAX_BYTES` are not set, the service
estimates safe limits based on disk space and message size telemetry. Results
are stored in `<CONTEXT_DIR>/storage_metrics.json` and updated over time.

Idempotency and persist policy
- IDEMPOTENT_PERSIST: enable persistent idempotency storage for request_id (default: false)
- IDEMPOTENT_STORE_DIR: dir for idempotent store file (default: context)
- IDEMPOTENT_TTL_SECS: TTL seconds for cached responses (default: 86400)
- PERSIST_REQUIRE_SESSION_ID: if true, disallow persist=true without session_id (default: false)

Index maintenance
- INDEX_KW_TTL_DAYS: TTL days for keywords in index.json before compaction (default: 90)
- INDEX_COMPACT_INTERVAL_MS: interval for background compaction job (default: 300000)

SSE and logging
- SSE_WARN_AFTER_MS: warn if a single SSE stream exceeds this duration (default: 60000)
- NERVOUS_SYSTEM_JSON_LOGS: enable JSON logs for structured logging (default: false)

Masking presets
- MASK_PRESETS_DIR: directory with regex preset files named <preset>.txt (default: config/mask_presets)

## Автоматическое определение `INTEGRITY_ROOT`

Сервис пытается определить корневой каталог конфигурации автоматически. Узел
[BasePathResolverNode](../docs/nodes/action-nodes.md#basepathresolvernode) поднимается от текущего
исполняемого файла вверх по иерархии, пока не найдёт `config/integrity.json`, и сохраняет путь в память.
Затем [InitConfigNode](../docs/nodes/action-nodes.md#initconfignode) устанавливает переменную
`INTEGRITY_ROOT`, если она не задана вручную. Укажите её явно только при запуске вне репозитория или при нестандартной
структуре каталогов.

How to use
- Create a .env file in repo root or `backend/` and set variables.
- The app loads env via dotenv at startup; env vars override .env.

Example .env
CONTEXT_DIR=./context
CONTEXT_MAX_LINES=1000
CONTEXT_MAX_BYTES=2000000
CONTEXT_DAILY_ROTATION=true
CONTEXT_ARCHIVE_GZ=true
CONTEXT_FLUSH_MS=0
MASK_PII=true
MASK_REGEX=\b\d{16}\b;secret\w+
MASK_ROLES=user,assistant
CHAT_RATE_LIMIT_PER_MIN=60
CHAT_RATE_KEY=auth
PROBES_IO_WATCHER_ENABLED=false
IO_WATCHER_THRESHOLD_MS=100
IDEMPOTENT_PERSIST=true
IDEMPOTENT_STORE_DIR=./context
IDEMPOTENT_TTL_SECS=86400
PERSIST_REQUIRE_SESSION_ID=false
INDEX_KW_TTL_DAYS=90
INDEX_COMPACT_INTERVAL_MS=300000
SSE_WARN_AFTER_MS=60000
NERVOUS_SYSTEM_JSON_LOGS=false
MASK_PRESETS_DIR=./config/mask_presets
