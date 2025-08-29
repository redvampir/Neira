# ENV Reference (Истина)

| Ключ | Тип | По умолчанию | Где используется | Влияние |
|---|---|---|---|---|
| CONTEXT_DIR | string | context | backend context storage | База для истории чатов |
| CONTEXT_MAX_LINES | int | 500 | storage trim | Ограничение строк при тримме |
| CONTEXT_MAX_BYTES | int | 1_000_000 | storage trim | Ограничение размера файла |
| CONTEXT_DAILY_ROTATION | bool | true | storage rotation | Ротация по дням |
| CONTEXT_ARCHIVE_GZ | bool | true | storage rotation | Архивирование .gz прошлых дней |
| CONTEXT_FLUSH_MS | int | 0 | storage buffering | Буферизованная запись, 0=выкл |
| MASK_PII | bool | true | storage masking | Маскирование PII по умолчанию |
| MASK_REGEX | string list (;) | — | storage masking | Кастомные regex для маскирования |
| MASK_ROLES | string list (,) | user | storage masking | Роли для маскирования |
| NODE_TEMPLATES_DIR | string | ./templates | backend init | Каталог шаблонов узлов |
| CHAT_RATE_LIMIT_PER_MIN | int | 120 | hub rate limit | Лимит запросов в минуту |
| CHAT_RATE_KEY | enum | auth | hub rate limit | Ключ лимита: auth/chat/session |
| IO_WATCHER_THRESHOLD_MS | int | 100 | nervous probes | Порог латентности для проб |
| NERVOUS_SYSTEM_ENABLED | bool | true | metrics/init | Включить /metrics и «нервную» подсистему |
| PROBES_HOST_METRICS_ENABLED | bool | true | nervous probes | Включить сбор хост‑метрик |
| PROBES_IO_WATCHER_ENABLED | bool | false | nervous probes | Включить наблюдатель ввода/вывода |
| INTEGRITY_ROOT | string | cwd | immune/integrity | Корень для конфигов интегрити |
| INTEGRITY_CONFIG_PATH | string | config/integrity.json | immune/integrity | Путь к конфигу |
| INTEGRITY_CHECK_INTERVAL_MS | int | 60000 | immune/integrity | Интервал проверки |
| IDEMPOTENT_PERSIST | bool | false | hub idempotency | Персистентный idempotent‑кэш |
| IDEMPOTENT_STORE_DIR | string | context | idem store | Каталог файла кэша |
| IDEMPOTENT_TTL_SECS | int | 86400 | idem store | TTL ответов |
| PERSIST_REQUIRE_SESSION_ID | bool | false | hub policies | Запрет persist без session_id |
| INDEX_KW_TTL_DAYS | int | 90 | index compaction | TTL ключевых слов в index.json |
| INDEX_COMPACT_INTERVAL_MS | int | 300000 | compaction job | Интервал фоновой чистки |
| SSE_WARN_AFTER_MS | int | 60000 | SSE | Варнинг при долгом стриме |
| NERVOUS_SYSTEM_JSON_LOGS | bool | false | logging | JSON‑логи включить |
| MASK_PRESETS_DIR | string | config/mask_presets | masking | Каталог пресетов масок |

Примечание: значения по умолчанию сверены с кодом (backend/src/*). При расхождениях — источник истины этот файл.

