Backend environment variables

- CONTEXT_DIR: base dir for chat history (default: context)
- CONTEXT_MAX_LINES: max lines kept in a file before trimming (default: 500)
- CONTEXT_MAX_BYTES: max bytes per file before trimming (default: 1_000_000)
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
- INTEGRITY_ROOT: base dir for integrity config and files (default: current working directory; set explicitly if the service runs outside `backend/`)
- INTEGRITY_CONFIG_PATH: path to integrity config file relative to INTEGRITY_ROOT or absolute (default: config/integrity.json)
- INTEGRITY_CHECK_INTERVAL_MS: integrity check interval in ms (default: 60000)

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

