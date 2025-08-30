<!-- neira:meta
id: NEI-20250915-adaptive-storage-env-docs
intent: docs
summary: Обновлено описание CONTEXT_MAX_LINES/CONTEXT_MAX_BYTES: адаптивные лимиты со storage_metrics.json.
-->

# ENV Reference (Истина)

| Ключ | Тип | По умолчанию | Где используется | Влияние |
|---|---|---|---|---|
| CONTEXT_DIR | string | context | backend context storage | База для истории чатов |
| CONTEXT_MAX_LINES | int | adaptive | storage trim | Ограничение строк (автоподбор, можно переопределить) |
| CONTEXT_MAX_BYTES | int | adaptive | storage trim | Ограничение размера файла (автоподбор, можно переопределить) |
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

Лимиты `CONTEXT_MAX_LINES` и `CONTEXT_MAX_BYTES` при отсутствии в окружении
оцениваются автоматически на основе свободного места диска и средней длины
сообщения. Метрики сохраняются и обновляются в `<CONTEXT_DIR>/storage_metrics.json`.

### Anti‑Idle System
| Ключ | Тип | По умолчанию | Где используется | Влияние |
|---|---|---|---|---|
| IDLE_THRESHOLD_SECONDS | int | 30 | idle detection | Порог простоя (сек) |
| DEEP_IDLE_THRESHOLD_MINUTES | int | 30 | idle detection | Глубокий простой (мин) |
| IDLE_MICRO_TASK_MAX_DURATION | string | 10min | anti-idle limits | Максимум одной микрозадачи |
| IDLE_SESSION_MAX_DURATION | string | 30min | anti-idle limits | Максимум одной сессии |
| IDLE_DAILY_AUTONOMOUS_LIMIT | string | 4hours | anti-idle limits | Дневной лимит автономии |
| IDLE_LEARNING_SESSION_LIMIT | string | 20min | learning | Лимит учебной сессии |
| IDLE_MONEY_SESSION_LIMIT | string | 15min | income | Лимит «заработка» |
| IDLE_REFLECTION_SESSION_LIMIT | string | 5min | reflection | Лимит размышлений |
| IDLE_REQUIRE_APPROVAL_FOR_NEW_DOMAINS | bool | true | safety | Одобрение новых доменов задач |
| IDLE_REPORT_FREQUENCY | enum | on_user_return | reporting | Частота отчётов |
| IDLE_DETAILED_LOGS | bool | true | reporting | Детальные логи |

Примечание: значения по умолчанию сверены с кодом (backend/src/*). При расхождениях — источник истины этот файл.
