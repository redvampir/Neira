<!-- neira:meta
id: NEI-20250915-adaptive-storage-env-docs
intent: docs
summary: Обновлено описание CONTEXT_MAX_LINES/CONTEXT_MAX_BYTES: адаптивные лимиты со storage_metrics.json.
-->

<!-- neira:meta
id: NEI-20250922-analysis-queue-env-docs
intent: docs
summary: Добавлены переменные для адаптивных порогов очередей анализа.
-->
<!-- neira:meta
id: NEI-20251010-organ-builder-env-docs
intent: docs
summary: описаны ORGANS_BUILDER_ENABLED и ORGANS_BUILDER_TEMPLATES_DIR.
-->
<!-- neira:meta
id: NEI-20251015-organ-builder-ttl-docs
intent: docs
summary: добавлена переменная ORGANS_BUILDER_TTL_SECS для очистки шаблонов.
-->

<!-- neira:meta
id: NEI-20251116-vite-api-url-env-doc
intent: docs
summary: Добавлена переменная VITE_API_URL для фронтенда.
-->

# ENV Reference (Истина)

| Ключ                         | Тип             | По умолчанию          | Где используется        | Влияние                                                          |
| ---------------------------- | --------------- | --------------------- | ----------------------- | ---------------------------------------------------------------- |
| CONTEXT_DIR                  | string          | context               | backend context storage | База для истории чатов                                           |
| CONTEXT_MAX_LINES            | int             | adaptive              | storage trim            | Ограничение строк (автоподбор, можно переопределить)             |
| CONTEXT_MAX_BYTES            | int             | adaptive              | storage trim            | Ограничение размера файла (автоподбор, можно переопределить)     |
| CONTEXT_DAILY_ROTATION       | bool            | true                  | storage rotation        | Ротация по дням                                                  |
| CONTEXT_ARCHIVE_GZ           | bool            | true                  | storage rotation        | Архивирование .gz прошлых дней                                   |
| CONTEXT_FLUSH_MS             | int             | 0                     | storage buffering       | Буферизованная запись, 0=выкл                                    |
| MASK_PII                     | bool            | true                  | storage masking         | Маскирование PII по умолчанию                                    |
| MASK_REGEX                   | string list (;) | —                     | storage masking         | Кастомные regex для маскирования                                 |
| MASK_ROLES                   | string list (,) | user                  | storage masking         | Роли для маскирования                                            |
| NODE_TEMPLATES_DIR           | string          | ./templates           | backend init            | Каталог шаблонов узлов                                           |
| CHAT_RATE_LIMIT_PER_MIN      | int             | 120                   | hub rate limit          | Лимит запросов в минуту                                          |
| CHAT_RATE_KEY                | enum            | auth                  | hub rate limit          | Ключ лимита: auth/chat/session                                   |
| IO_WATCHER_THRESHOLD_MS      | int             | 100                   | nervous probes          | Порог латентности для проб                                       |
| ANALYSIS_QUEUE_FAST_MS       | int             | adaptive              | queue thresholds        | Граница Fast/Standard очереди                                    |
| ANALYSIS_QUEUE_LONG_MS       | int             | adaptive              | queue thresholds        | Граница Standard/Long очереди                                    |
| ANALYSIS_QUEUE_RECALC_MIN    | int             | 100                   | queue thresholds        | Новые запросы для пересчёта                                      |
| NERVOUS_SYSTEM_ENABLED       | bool            | true                  | metrics/init            | Вклюить /metrics и «нервную» подсистему                          |
| PROBES_HOST_METRICS_ENABLED  | bool            | true                  | nervous probes          | Включить сбор хост‑метрик                                        |
| PROBES_IO_WATCHER_ENABLED    | bool            | false                 | nervous probes          | Включить наблюдатель ввода/вывода                                |
| INTEGRITY_ROOT               | string          | cwd                   | immune/integrity        | Корень для конфигов интегрити                                    |
| INTEGRITY_CONFIG_PATH        | string          | config/integrity.json | immune/integrity        | Путь к конфигу                                                   |
| INTEGRITY_CHECK_INTERVAL_MS  | int             | 60000                 | immune/integrity        | Интервал проверки                                                |
| IDEMPOTENT_PERSIST           | bool            | false                 | hub idempotency         | Персистентный idempotent‑кэш                                     |
| IDEMPOTENT_STORE_DIR         | string          | context               | idem store              | Каталог файла кэша                                               |
| IDEMPOTENT_TTL_SECS          | int             | 86400                 | idem store              | TTL ответов                                                      |
| PERSIST_REQUIRE_SESSION_ID   | bool            | false                 | hub policies            | Запрет persist без session_id                                    |
| INDEX_KW_TTL_DAYS            | int             | 90                    | index compaction        | TTL ключевых слов в index.json                                   |
| INDEX_COMPACT_INTERVAL_MS    | int             | 300000                | compaction job          | Интервал фоновой чистки                                          |
| SSE_WARN_AFTER_MS            | int             | 60000                 | SSE                     | Варнинг при долгом стриме                                        |
| NERVOUS_SYSTEM_JSON_LOGS     | bool            | false                 | logging                 | JSON‑логи включить                                               |
| MASK_PRESETS_DIR             | string          | config/mask_presets   | masking                 | Каталог пресетов масок                                           |
| ORGANS_BUILDER_ENABLED       | bool            | false                 | organ builder           | Включить модуль; при запуске восстанавливает статусы из каталога |
| ORGANS_BUILDER_TEMPLATES_DIR | string          | organ_templates       | organ builder           | Каталог шаблонов органов (все \*.json загружаются как stable)    |
| ORGANS_BUILDER_TTL_SECS      | int             | 3600                  | organ builder           | Время хранения шаблонов после стабилизации (сек)                 |
| VITE_API_URL                 | string          | —                     | frontend requests       | Базовый URL API для фронтенда                                    |

Лимиты `CONTEXT_MAX_LINES` и `CONTEXT_MAX_BYTES` при отсутствии в окружении
оцениваются автоматически на основе свободного места диска и средней длины
сообщения. Метрики сохраняются и обновляются в `<CONTEXT_DIR>/storage_metrics.json`.

### Anti‑Idle System

| Ключ                                  | Тип    | По умолчанию   | Где используется | Влияние                       |
| ------------------------------------- | ------ | -------------- | ---------------- | ----------------------------- |
| IDLE_THRESHOLD_SECONDS                | int    | 30             | idle detection   | Порог простоя (сек)           |
| DEEP_IDLE_THRESHOLD_MINUTES           | int    | 30             | idle detection   | Глубокий простой (мин)        |
| IDLE_MICRO_TASK_MAX_DURATION          | string | 10min          | anti-idle limits | Максимум одной микрозадачи    |
| IDLE_SESSION_MAX_DURATION             | string | 30min          | anti-idle limits | Максимум одной сессии         |
| IDLE_DAILY_AUTONOMOUS_LIMIT           | string | 4hours         | anti-idle limits | Дневной лимит автономии       |
| IDLE_LEARNING_SESSION_LIMIT           | string | 20min          | learning         | Лимит учебной сессии          |
| IDLE_MONEY_SESSION_LIMIT              | string | 15min          | income           | Лимит «заработка»             |
| IDLE_REFLECTION_SESSION_LIMIT         | string | 5min           | reflection       | Лимит размышлений             |
| IDLE_REQUIRE_APPROVAL_FOR_NEW_DOMAINS | bool   | true           | safety           | Одобрение новых доменов задач |
| IDLE_REPORT_FREQUENCY                 | enum   | on_user_return | reporting        | Частота отчётов               |
| IDLE_DETAILED_LOGS                    | bool   | true           | reporting        | Детальные логи                |

### Backpressure Auto Backoff

| Переменная           | Тип  | Дефолт | Раздел       | Описание                                                     |
| -------------------- | ---- | ------ | ------------ | ------------------------------------------------------------ |
| AUTO_BACKOFF_ENABLED | bool | false  | backpressure | Включить авто‑бэкофф сверх базового сна при высоком давлении |
| BP_MAX_BACKOFF_MS    | int  | 200    | backpressure | Максимальный дополнительный сон (мс) при авто‑бэкоффе        |

Примечание: значения по умолчанию сверены с кодом (backend/src/\*). При расхождениях — источник истины этот файл.

---

## Homeostasis & Control (дополнение)

| var                            | type   | default            | area        | description                                                                                 |
| ------------------------------ | ------ | ------------------ | ----------- | ------------------------------------------------------------------------------------------- |
| HOMEOSTASIS_ENABLED            | bool   | false              | homeostasis | Включить автотюнинг бюджетов (experimental)                                                 |
| HOMEOSTASIS_RECALC_INTERVAL_MS | int    | 10000              | homeostasis | Интервал пересчёта лимитов                                                                  |
| BUDGET_DEFAULT_CONCURRENCY     | int    | adaptive           | budgets     | Базовый параллелизм (ceil/floor задаются политикой)                                         |
| BUDGET_DEFAULT_BATCH           | int    | adaptive           | budgets     | Базовый размер батчей                                                                       |
| REASONING_TIME_BUDGET_MS       | int    | adaptive           | budgets     | Мягкий лимит времени рассуждений                                                            |
| WATCHDOG_REASONING_SOFT_MS     | int    | 30000              | watchdog    | Soft‑таймаут (деградация/упрощение плана)                                                   |
| WATCHDOG_REASONING_HARD_MS     | int    | 120000             | watchdog    | Hard‑таймаут (прерывание шага)                                                              |
| LOOP_DETECT_ENABLED            | bool   | true               | watchdog    | Включить детектор циклов/повторов                                                           |
| LOOP_WINDOW_TOKENS             | int    | 256                | watchdog    | Окно анализа повторов                                                                       |
| LOOP_REPEAT_THRESHOLD          | float  | 0.6                | watchdog    | Порог повторяемости (0..1)                                                                  |
| CONTROL_ALLOW_PAUSE            | bool   | true               | control     | Разрешить pause/resume (admin)                                                              |
| CONTROL_ALLOW_KILL             | bool   | true               | control     | Разрешить аварийную остановку (admin)                                                       |
| CONTROL_SNAPSHOT_DIR           | string | ./snapshots        | control     | Каталог для snapshot‑срезов                                                                 |
| TRACE_ENABLED                  | bool   | false              | control     | Включить генерацию трасс по request_id                                                      |
| NEIRA_BIND_ADDR                | string | 127.0.0.1:3000     | http        | Адрес/порт HTTP‑сервера (для тестов можно указать уникальный порт, например `0.0.0.0:4000`) |
| SSE_DEV_DELAY_MS               | int    | 0                  | http/sse    | Искусственная задержка между SSE‑сообщениями (для тестов дренажа)                           |
| NEIRA_ADMIN_TOKEN              | string | -                  | auth        | Токен администратора (dev)                                                                  |
| NEIRA_WRITE_TOKEN              | string | -                  | auth        | Токен записи (dev)                                                                          |
| NEIRA_READ_TOKEN               | string | -                  | auth        | Токен чтения (dev)                                                                          |
| TRACE_MAX_EVENTS               | int    | 200                | control     | Лимит событий на `request_id`                                                               |
| LOGS_TAIL_LINES                | int    | 200                | control     | Хвост логов, который включать в snapshot                                                    |
| BACKPRESSURE_HIGH_WATERMARK    | int    | 100                | throttle    | Порог для мягкого троттлинга                                                                |
| BACKPRESSURE_THROTTLE_MS       | int    | 0                  | throttle    | Задержка при превышении порога (мс)                                                         |
| REASONING_TOKEN_BUDGET         | int    | 0                  | http/sse    | Лимит «токенов» для SSE‑стрима (0 — без лимита)                                             |
| DEV_ROUTES_ENABLED             | bool   | false              | dev         | Включить dev‑эндпоинты (SSE/analysis)                                                       |
| SSE_DEV_TOKENS                 | int    | 200                | dev         | Количество сообщений в dev‑стриме                                                           |
| WATCHDOG_REASONING_SOFT_MS     | int    | 30000              | watchdog    | Soft‑таймаут рассуждений (общий)                                                            |
| WATCHDOG_REASONING_HARD_MS     | int    | global_time_budget | watchdog    | Hard‑таймаут рассуждений (общий)                                                            |
| WATCHDOG*SOFT_MS*<NODEID>      | int    | -                  | watchdog    | Пер‑узловой soft‑таймаут (ID: UPPER*CASE, не алф/цифры → `*`)                               |
| WATCHDOG*HARD_MS*<NODEID>      | int    | -                  | watchdog    | Пер‑узловой hard‑таймаут                                                                    |
| AUTO_REQUEUE_ON_SOFT           | bool   | false              | watchdog    | Авто‑переочередить в Long при soft‑таймауте (вернуть Draft сразу)                           |
| INCIDENT_WEBHOOK_URL           | string | -                  | alerts      | Вебхук уведомлений о hard‑таймаутах/инцидентах                                              |

Примечание: значения daptive задаются автоматически по пробам и метрикам; ENV служит как потолок/пол и аварийные дефолты.

---

### Anti‑Idle System (normalized)

| Переменная                            | Тип    | Дефолт         | Раздел           | Описание                                                  |
| ------------------------------------- | ------ | -------------- | ---------------- | --------------------------------------------------------- |
| ANTI_IDLE_ENABLED                     | bool   | true           | anti-idle core   | Включить каркас Anti‑Idle (только метрики, без автозадач) |
| IDLE_THRESHOLD_SECONDS                | int    | 30             | idle detection   | Порог простоя (сек)                                       |
| LONG_IDLE_THRESHOLD_MINUTES           | int    | 5              | idle detection   | Длительный простой (мин)                                  |
| DEEP_IDLE_THRESHOLD_MINUTES           | int    | 30             | idle detection   | Глубокий простой (мин)                                    |
| IDLE_MICRO_TASK_MAX_DURATION          | string | 10min          | anti-idle limits | Максимум одной микрозадачи                                |
| IDLE_SESSION_MAX_DURATION             | string | 30min          | anti-idle limits | Максимум одной сессии                                     |
| IDLE_DAILY_AUTONOMOUS_LIMIT           | string | 4hours         | anti-idle limits | Дневной лимит автономии                                   |
| IDLE_LEARNING_SESSION_LIMIT           | string | 20min          | learning         | Лимит учебной сессии                                      |
| IDLE_MONEY_SESSION_LIMIT              | string | 15min          | income           | Лимит «заработка»                                         |
| IDLE_REFLECTION_SESSION_LIMIT         | string | 5min           | reflection       | Лимит размышлений                                         |
| IDLE_REQUIRE_APPROVAL_FOR_NEW_DOMAINS | bool   | true           | safety           | Одобрение новых доменов задач                             |
| IDLE_REPORT_FREQUENCY                 | enum   | on_user_return | reporting        | Частота отчётов                                           |
| IDLE_DETAILED_LOGS                    | bool   | true           | reporting        | Детальные логи                                            |
