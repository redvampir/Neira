# Дорожная карта (Stage 0 → Stage 1)

Цель: запустить минимально живую, самонаблюдаемую и безопасную Нейру, затем поэтапно включать способности.

Оглавление
- Stage 0 — Core Stable
- Stage 1 — Experimental Growth
- Политика включения (Гейты)
- Контроль исполнения
- См. также

## Stage 0 — Core Stable
- Коммуникация (Chat I/O) — stable
- Память (Context Storage) — stable
- Маскирование (PII) — stable
- Идемпотентность — stable
- Иммунная система (safe‑mode write=admin) — stable
- Нервная система (метрики, /metrics) — stable

### Definition of Done (Stage 0)
- API и SSE работают; rate‑limit заголовки отдаются
- Хранение контекста, поиск по content; ротация/gzip; компакция индекса
- Маскирование пресетами, dry‑run; admin‑политика
- Идемпотентность: повтор с request_id возвращает идентичный ответ
- Safe‑mode: записи требуют admin; карантин‑каркас присутствует
- Метрики соответствуют docs/reference/metrics.md, экспонируются на /metrics
  - Ключевые: chat_* (requests/errors/latency), messages_saved, context_* (loads/misses/bytes), gz_rotate_count,
    index_compact_runs, sessions_* (created/deleted/closed/active), requests_idempotent_hits, sse_active, safe_mode

### Runtime Extensibility (связь с docs/design/runtime-extensibility.md)
- Развёрнут каркас (plugins/scripts, plugins/wasm, plugins/index.json)
- Доступно только чтение: `GET /api/neira/plugins`, `GET /api/neira/ui/tools`
- UI‑события принимаются в dry‑run (без исполнения)
- Гейты — LOCKED: `ui_tools_registry`, `runtime_scripting_rhai`, `runtime_wasm_plugins`

### Anti‑Idle System (связь с docs/design/anti-idle-system.md)
- Активен каркас `anti_idle_core` в safe‑defaults: фиксируется `idle_state`, journaling размышлений
- Выполнение микрозадач отключено; только отчёты и dry‑run
- Гейты Anti‑Idle — LOCKED/EXPERIMENTAL (см. CAPABILITIES.md)

## Stage 1 — Experimental Growth
- Пробы возможностей (probes_capability) — experimental (read‑only)
- Интроспекция (introspection_status) — experimental (из CAPABILITIES.md; затем HTTP)
- Журналирование решений — experimental (см. JOURNALING.md)
- Домостаз (budgets/back‑off) — locked (только документ)
- Self‑edit, Training — locked

### Метрики Stage 1
- idle_state, idle_minutes_today, autonomous_time_spent_seconds
- auto_tasks_started, auto_tasks_completed, auto_tasks_blocked, microtask_queue_depth
- approvals_pending

### Runtime Extensibility
- Разблокировка гейтов (выборочно):
  - `ui_tools_registry` → experimental: отдача UI‑дескрипторов зарегистрированных скриптов
  - `runtime_scripting_rhai` → experimental: обработка UI‑событий скриптами (лимиты/таймауты)
- `runtime_wasm_plugins` остаётся LOCKED до стабилизации скриптового слоя
- Пример: инструмент «Карандаш» (annotations) как безопасный скриптовый плагин

### Anti‑Idle System
- Разблокировка `learning_microtasks` и/или `reflection_journal` (experimental, без внешней сети)
- Критерии выхода в stable — см. docs/reference/metrics.md и CAPABILITIES.md (сигналы/SLO)

### Критерии выпуска experimental → stable
- Ошибки/латентность в SLO за окно времени
- Отсутствие нарушений политик/безопасности
- Влияние на ресурсы в пределах бюджета

## Политика включения (Гейты)
- По умолчанию активны только stable‑семена; остальные — locked/experimental
- Разблокировка фразами: «Разблокируй {capability}», «Покажи статус способностей»
- Не включать одновременно несколько сложных способностей; двигаться шагами
- На разблокировке/блокировке: сообщать риски, safeguards и путь отката

## Контроль исполнения
- Синхронизировать CAPABILITIES.md со статусом способностей
- Обновлять docs/reference/env.md и docs/reference/metrics.md при изменениях
- Использовать neira:meta в файлах кода для фиксации intent/ссылок/сигналов
- В конце каждой задачи — 1–10 предложений улучшений (backlog)

## См. также
- Anti‑Idle System: docs/design/anti-idle-system.md
- Runtime Extensibility: docs/design/runtime-extensibility.md
- Organ Systems: docs/design/organ-systems.md
- Способности и гейты: CAPABILITIES.md


<!-- neira:meta
id: NEI-20250830-Roadmap-Proposals
intent: docs
summary: |
  Добавлены предложения по реализации для Stage 0 и Stage 1 с измеримыми результатами, метриками и фичефлагами. Фокус — сохранить «живую архитектуру» через локальные правила, обратимость и наблюдаемость.
-->

## Предложения по реализации (дополнение)

Ниже — минимальные пакеты работ, согласованные с «живой» архитектурой (децентрализация решений, обратимость, эмерджентность), метриками и staged rollout.

### Stage 0 — Минимальный пакет (Core Stable)
- Метрики RED/USE: базовые счётчики и p95/p99 латентности по Chat I/O, контексту и SSE; `/metrics` обязателен.
  - DoD: метрики из docs/reference/metrics.md доступны; алерты на ошибки/латентность; `sse_active`, `safe_mode` — gauge.
- Идемпотентность по `request_id`: сохранение ключа c TTL, повтор — `idempotent=true`; счётчик `requests_idempotent_hits`.
  - DoD: повторный POST на `/api/neira/chat` возвращает идентичный ответ; хедеры rate limit присутствуют.
- Чёрная доска контекста: единообразный append-only лог + компактирование индекса.
  - DoD: стабильный экспорт/импорт; растут `index_compact_runs`, `context_bytes_written`; нет потерь при рестарте.
- Safe mode и политики записи: в safe-mode все мутации требуют `admin`; маскирование — через пресеты, `dry_run` обязателен.
  - DoD: переключение safe-mode отражается в метрике; попытки записи без `admin` — отказ и аудит‑лог.
- Квоты/Rate limiting: per `chat_id`/`session_id` token bucket, хедеры X-RateLimit* как в API справочнике.
  - DoD: предсказуемое ограничение нагрузки; корреляция с `request_id` в логах.
- Структурные JSON‑логи: корреляция `request_id`/`session_id`, включаемые через `NERVOUS_SYSTEM_JSON_LOGS`.
  - DoD: трассируемость сквозных запросов, связка логов и метрик.
- Фичефлаги/способности: текущее состояние в CAPABILITIES.md; включение/выключение простыми фразами владельца.
  - DoD: каждая новая возможность имеет gate, описание рисков, safeguards и rollback.

Safeguards (Stage 0)
- По умолчанию всё потенциально разрушительное — в safe-mode и `dry_run`.
- Откаты через фичефлаг; изменения обратимы; миграции данных — только с экспортом/резервной копией.

### Stage 1 — Экспериментальный пакет (Experimental Growth)
- Локальные «рефлексы» узлов: приоритетные очереди и быстрые реакции без центрального оркестратора.
  - DoD: метрики `microtask_queue_depth`, `auto_tasks_started/completed/blocked`; снижение p95 для горячих путей.
- Адаптивные бюджеты и backoff: динамическая подстройка лимитов CPU/IO/частоты по метрикам и ошибкам.
  - DoD: видны `throttle_events`, `retry_backoff_applied`; стабилизация ошибок до целевых SLO.
- Anti‑Idle микрозадачи: `learning_microtasks` и `reflection_journal` как фоновые активности.
  - DoD: включаются флагом; новые метрики из раздела Stage 1; отсутствие влияния на p95 основных путей.
- Runtime extensibility (read‑only/limited): список плагинов и UI‑инструментов, без выполнения кода по умолчанию.
  - DoD: эндпоинты перечисления доступны; выполнение скриптов/wasm — LOCKED; capability negotiation описан.
- Проекции памяти: из событийного лога в индексы (LSM/векторный) с мягкими компакциями.
  - DoD: согласованность eventual; документированы режимы деградации; рост `index_compact_runs` под контролем.

Exit‑критерии и откаты (Stage 1)
- Каждая возможность имеет: метрики успеха, лимиты риска, условие быстрого отката (флаг) и план перевода в stable.
- При деградации SLO — автоматический откат на предыдущую политику/лимиты.

Связи и источники
- API: docs/api/backend.md, docs/api/chat.md
- Метрики: docs/reference/metrics.md
- Anti‑Idle: docs/design/anti-idle-system.md
- Runtime Extensibility: docs/design/runtime-extensibility.md
- Способности/флаги: CAPABILITIES.md

## Интерфейсы (UI/UX) для старта и развития

### Stage 0 — Базовый интерфейс (Core Stable)
- Чат (Web/CLI) поверх `POST /api/neira/chat` и `POST /api/neira/chat/stream`:
  - SSE‑поток, Markdown/код/диффы, вложения как ссылки, выбор `chat_id`/`session_id`.
  - Быстрый фидбэк качества: 👍/👎, шкала 1–5, теги причины — сохраняется в контекст.
- Управление сессиями и контекстом:
  - Создание/переимен./удаление сессий, поиск по истории, экспорт/импорт NDJSON.
- Панель владельца (Control Plane):
  - Переключатели safe‑mode, пресеты маскирования с `dry_run`, просмотр rate‑limit и лимитов узлов.
  - Фичефлаги/способности из `CAPABILITIES.md` (locked/experimental/stable) с карточкой риска/отката.
  - Метрики и здоровье: обзор `/metrics` (RED/USE), liveness/readiness, корреляция `request_id`.
- Журналирование и наблюдаемость:
  - Включаемые JSON‑логи, привязка фидбэка к `request_id`/`session_id`, быстрый «снимок» контекста запроса.
- Идемпотентность и квоты:
  - Поле `request_id` в UI, показ `X-RateLimit-*`, мягкое предупреждение о повторе/лимитах.
- Безопасность:
  - Токены/роли (`read`/`write`/`admin`), индикатор safe‑mode (on/off), аудит действий.

### Stage 1 — Расширение (Experimental Growth)
- Инспекция графа узлов (read‑only): список узлов, состояние, базовые метрики/логи на узел, без вмешательства.
- Anti‑Idle микрозадачи: очередь `learning_microtasks`, кнопки «предложить рефлексию/задачу», просмотр `JOURNALING.md`.
- Адаптивные бюджеты: визуализация троттлинга/backoff, настройка мягких лимитов по политике, safeguards — быстрый откат.
- Эксперименты и A/B: включение experimental‑фич на долю трафика с авто‑откатом по метрикам.
- Runtime Extensibility (read‑only): каталог плагинов/инструментов; выполнение кода — `LOCKED` по умолчанию.

Связи
- API: `docs/api/backend.md`, `docs/api/chat.md`
- Метрики и здоровье: `docs/reference/metrics.md`
- Журналы/рефлексия: `JOURNALING.md`
- Способности/флаги: `CAPABILITIES.md`
- Дизайн: `docs/design/anti-idle-system.md`, `docs/design/runtime-extensibility.md`

## Архитектурные опоры (совместимые заимствования из «мира машин»)
- Гомеостаз (обратная связь): динамические лимиты/троттлинг по метрикам.
- «Чёрная доска»/стигмергия: событийный слой и индекс вместо жёсткой оркестрации.
- Память как метаболизм: append‑only события → проекции (LSM/индексы).
- Иммунная система: quarantine/integrity/safe‑mode как обратимые защиты.
- Нервные рефлексы: приоритетные очереди и локальные реакции на узлах.
- Фичефлаги/способности: locked → experimental → stable с откатами.
- Контракты и версии: семвер шаблонов узлов + адаптеры.
- Наблюдаемость: RED/USE, структурные логи, трассировка графов.
- Энергобюджеты: квоты/бакеты как «АТФ» для узлов/сессий.
- Многоуровневая память: рабочая/эпизодическая/семантическая (кэш→индекс→долгосрочная).
- Расширяемость‑как‑симбиоз: плагины/скрипты в песочнице с negotiation способностей.
- Саморефлексия: журнал и anti‑idle микрозадачи в окна простоя.

## Личность (Persona) — ядро и ростки

### Stage 0 (Core Stable)
- Включено ядро личности `persona_kernel` (инварианты).
- Минимальные роли `persona_roles_minimal` (coder/editor/architect).
- Стиль по умолчанию `persona_style_neutral` (интенсивность «teen» = 0).
- Метрики базовые: `role_switches_total`, при наличии — `style_adherence`.

### Stage 1 (Experimental Growth)
- Рефлексия личности `persona_reflection` (proposals → review → canary → stable).
- Эфемерный тон `tone_state` (auto-reset; не трогает ценности).
- Стиль `persona_style_teen` (регулятор 0–3) — экспериментально.
- Творческие студии: `studio_artflow`, `studio_soundweaver`, `studio_storynodes` — LOCKED→EXPERIMENTAL.
- Метрики: `persona_drift_score`, `style_adherence`, `reflection_journal_entries`, `proposals_accepted_total`, `proposals_reverted_total`.

Ссылки: docs/meta/persona-kernel.md, docs/reference/persona-metrics.md, CAPABILITIES.md.
