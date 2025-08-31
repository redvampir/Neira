<!-- neira:meta
id: NEI-20250830-Roadmap-Cleanup
intent: docs
summary: Чистовая дорожная карта Stage 0 → Stage 1: цели, DoD, интерфейсы, homeostasis/control, persona. Актуализированы ссылки и гейты. -->

# Дорожная карта (Stage 0 → Stage 1)

Цель: довести минимально живую систему до устойчивого ядра и безопасно расширять её экспериментами при сохранении контроля и обратимости.

Содержание
- Stage 0 — Core Stable (минимум для живости и контроля)
- Stage 1 — Experimental Growth (аккуратное расширение)
- Выпуск и раскатка
- Документация и ссылки
- Предложения по реализации (дополнение)
- Интерфейсы (UI/UX) для старта и развития
- Архитектурные опоры (совместимые заимствования)
- Личность (Persona) — ядро и ростки

## Stage 0 — Core Stable
- [x] Chat I/O — стабильный API и SSE‑поток (meta/progress/cancel), rate‑limit заголовки.
- [x] Context Storage — append‑only + экспорт/импорт NDJSON, компакции по расписанию.
- [x] Masking/PII — пресеты, dry‑run, админ‑политики.
- [x] Idempotency — `request_id` + TTL, детерминированные повторы, `requests_idempotent_hits`.
- [x] Safe‑mode — все записи требуют admin; включение отражается в метрике `safe_mode`.
- [x] Metrics/Observability — RED/USE, `/metrics`, структурные JSON‑логи и корреляция `request_id`/`session_id`.
- [x] Control Plane (admin) — pause/resume/kill/status; snapshot и (ограниченно) trace.

Доп. инструменты (реализовано)
- [x] Watchdogs (soft/hard) для рассуждений; пер‑узловые таймауты через ENV; лейблы в метриках.
- [x] Loop‑detect в SSE и мягкое завершение потока при «заедании».
- [x] Трассы запросов (`TRACE_ENABLED`) и `GET /api/neira/trace/:request_id`.
- [x] Snapshot: tail логов с маскированием, экспорт трасс, упаковка в ZIP.
- [x] Очереди/давление: `GET /api/neira/queues/status`; мягкий троттлинг при backpressure.
- [x] Dev‑маршруты: длинный SSE и «длинный анализ» для тестов дренажа и watchdog.
- [x] SSE‑бюджет токенов (per‑request/ENV) с прогрессом и `budget_hits_total`.
- [x] Webhook уведомления о hard‑таймаутах (`INCIDENT_WEBHOOK_URL`).
- [x] CLI‑диагностика (`diagnose`) — быстрый статус/метрики.

Реализовано (срез 1)
- Пауза/резюм — блокировка новых chat/analysis/stream/session/import, дренаж SSE по флагу.
- Статус — paused_for_ms/paused_since_ts_ms, active_tasks, backpressure и длины очередей.
- Snapshot — include=metrics,context; файл сохраняется в CONTROL_SNAPSHOT_DIR.
- Kill — graceful shutdown с форс‑таймаутом.
- Логи — file sink (tracing‑appender), audit событий pause/resume/kill/snapshot.
- SSE dev delay — `SSE_DEV_DELAY_MS` для тестов дренажа.
- Адрес сервера — `NEIRA_BIND_ADDR` для параллельных тестов.

### Definition of Done (Stage 0)
- API и SSE соответствуют спецификации; rate‑limit прозрачен.
- Контекст и индекс устойчивы к рестарту; есть компакции и экспорт/импорт.
- Маскирование включаемо политикой, есть `dry_run`; safe‑mode принуждает write=admin.
- Идемпотентность — повтор POST на `/api/neira/chat` отдаёт идентичный ответ.
- Включены базовые метрики (см. docs/reference/metrics.md), `/metrics` работает.
- Control Plane — доступны pause/resume/kill/status; snapshot формируется.

## Stage 1 - Experimental Growth
- [x] Probes capability (read-only) — сбор хост‑метрик и длительностей.
- [x] Trace requests — трасса по `request_id` (experimental).
- [x] Introspection status (HTTP) — базовая самодиагностика.
- [x] Runtime Extensibility — read‑only листинги плагинов/инструментов (exec = LOCKED).
- Probes capability (read‑only) — сбор хост‑метрик и длительностей.
- Introspection status (HTTP) — базовая самодиагностика.
- Anti‑Idle микрозадачи — `learning_microtasks` и `reflection_journal` (экспериментально).
- Homeostasis budgets — автотюнинг конкурентности/батчей/бюджетов рассуждений (experimental).
- Trace requests — трасса по `request_id` (experimental).
- Runtime Extensibility — список плагинов/инструментов (exec = LOCKED).

### Переход к stable (примерные критерии)
- Ошибки/латентность в пределах SLO на окне времени.
- Нет нарушений политик/безопасности.
- Влияние на ресурсы в пределах бюджета.

## Выпуск и раскатка
- Стейджинг возможностей через CAPABILITIES: locked → experimental → stable.
- Включение/выключение — простыми фразами владельца; всегда указывать safeguards и план отката.
- Рискованные изменения по умолчанию в safe‑mode и `dry_run`.

## Документация и ссылки
- API: docs/api/backend.md, docs/api/chat.md
- Homeostasis & Control: docs/design/homeostasis.md
- Метрики: docs/reference/metrics.md, docs/reference/persona-metrics.md
- ENV: docs/reference/env.md
- Способности/флаги: CAPABILITIES.md (+ docs/meta/capabilities-*.md)
  - Обновлены блоки Control/Persona в CAPABILITIES.md (статусы: control_*=stable, trace_requests/homeostasis_budgets=experimental).

---

## Предложения по реализации (дополнение)

Stage 0 — минимальный пакет
- Метрики RED/USE и ключевые gauges (`sse_active`, `safe_mode`); `/metrics` обязателен.
- Идемпотентность по `request_id`; счётчик `requests_idempotent_hits`.
- «Чёрная доска» контекста: append‑only + компактирование индекса; экспорт/импорт.
- Safe‑mode/маскирование: пресеты, `dry_run`, write=admin.
- Квоты/Rate limiting: X‑RateLimit* на Chat I/O.
- Структурные JSON‑логи: корреляция `request_id`/`session_id`.
- Фичефлаги: актуально в CAPABILITIES.md с рисками и откатом.

Stage 1 — экспериментальный пакет
- Локальные «рефлексы» узлов: приоритетные очереди, быстрые реакции.
- Адаптивные бюджеты и backoff по метрикам/ошибкам.
- Anti‑Idle микрозадачи: `learning_microtasks`, `reflection_journal` (флагами).
- Runtime extensibility (read‑only): каталог плагинов и UI‑инструментов.
- Проекции памяти: события → индексы (LSM/векторный), мягкие компакции.

Exit‑критерии/откаты
- Для каждой фичи: метрики успеха, лимиты риска, быстрый откат флагом, план перехода в stable.

---

## Интерфейсы (UI/UX) для старта и развития

Stage 0 — базовый интерфейс
- Чат (Web/CLI) поверх `/api/neira/chat` и `/api/neira/chat/stream`:
  - SSE‑поток, Markdown/код/диффы; выбор `chat_id`/`session_id`.
  - Быстрый фидбэк качества: 👍/👎, 1–5, теги причин — пишется в контекст.
- Управление сессиями/контекстом: создать/переименовать/удалить; поиск; экспорт/импорт (NDJSON).
- Панель владельца (Control Plane): safe‑mode, маскирование с `dry_run`, фичефлаги, метрики `/metrics`.
- Логи/наблюдаемость: JSON‑логи, привязка к `request_id`/`session_id`.
- Idempotency и квоты: поле `request_id` в UI, показ `X‑RateLimit-*`.
- Безопасность: токены/роли (`read`/`write`/`admin`), индикатор safe‑mode, аудит.

Stage 1 — расширение
- Инспекция графа узлов (read‑only): состояния/метрики/логи per узел.
- Anti‑Idle микрозадачи: очередь `learning_microtasks`, кнопки «рефлексия/задача», просмотр `JOURNALING.md`.
- Адаптивные бюджеты: визуализация троттлинга/backoff, мягкие лимиты по политике.
- Эксперименты/A‑B: включение experimental‑фич на долю трафика с авто‑откатом.
- Runtime Extensibility (read‑only): каталог плагинов/инструментов (исполнение LOCKED).

Ссылки: docs/api/backend.md, docs/reference/metrics.md, JOURNALING.md, CAPABILITIES.md.

---

## Архитектурные опоры (совместимые заимствования)
- Гомеостаз: динамические лимиты/троттлинг по метрикам.
- «Чёрная доска»/стигмергия: события + индекс вместо жёсткой оркестрации.
- Память как метаболизм: append‑only события → проекции (LSM/индексы).
- Иммунная система: quarantine/integrity/safe‑mode как обратимые защиты.
- Нервные рефлексы: приоритетные очереди и локальные реакции на узлах.
- Фичефлаги: locked → experimental → stable с откатами.
- Контракты и версии: семвер шаблонов узлов + адаптеры.
- Наблюдаемость: RED/USE, структурные логи, трассировка.
- Энергобюджеты: квоты/бакеты для узлов/сессий.
- Многоуровневая память: рабочая/эпизодическая/семантическая.
- Расширяемость‑как‑симбиоз: песочницы/скрипты, capability negotiation.
- Саморефлексия: журнал и anti‑idle микрозадачи в окна простоя.

---

## Личность (Persona) — ядро и ростки

Stage 0 (Core Stable)
- `persona_kernel` — инварианты ценностей (честность, уважение, безопасность, полезность, воспроизводимость, краткость).
- `persona_roles_minimal` — роли coder/editor/architect.
- `persona_style_neutral` — стиль по умолчанию (интенсивность «teen» = 0).
- Метрики: `role_switches_total`, (по возможности) `style_adherence`.

Stage 1 (Experimental Growth)
- `persona_reflection` — предложения микрокоррекций (review→canary→stable).
- `tone_state` — эфемерный тон/настроение (auto‑reset; не затрагивает ценности).
- `persona_style_teen` — экспериментальная окраска (регулятор 0–3).
- Творческие студии: `studio_artflow`, `studio_soundweaver`, `studio_storynodes` — LOCKED→EXPERIMENTAL.
- Метрики: `persona_drift_score`, `style_adherence`, `reflection_journal_entries`, `proposals_accepted_total`, `proposals_reverted_total`.

Ссылки: docs/meta/persona-kernel.md, docs/reference/persona-metrics.md, CAPABILITIES.md

---

### Stage 1 — Checklist (Normalized)
- [x] Probes capability (read‑only) — сбор хост‑метрик и длительностей.
- [x] Trace requests — трасса по `request_id` (experimental).
- [x] Introspection status (HTTP) — базовая самодиагностика.
- [x] Runtime Extensibility — read‑only листинги плагинов/инструментов (exec = LOCKED).
- [ ] Anti‑Idle микрозадачи — `learning_microtasks` и `reflection_journal` (каркас готов, запуска нет).
- [ ] Homeostasis budgets — обратные давления/паузы/лимиты в обработке запросов (experimental).
- [ ] Factory (Adapter) — FabricatorNode + SelectorNode, только dry‑run/HITL (experimental).
- [ ] OrganTemplate/OrganBuilder — сборка органов (dry‑run → canary) с интеграцией NS/IS (experimental).
- [ ] Training Orchestrator (HITL) — мини‑циклы стабилизации узлов до Experimental/Stable.

---

## Universal System Roadmap (high‑level)

Factory & Organs
- Adapter→Script→WASM (exec под флагами); Organ Builder v0 (dry‑run→canary→experimental); schemas (factory‑spec/organ‑template/policy); natural‑language команды → FactorySpec/Policy.

Policies & Safety
- Policy Engine: роли/approvals/risk; JSON‑ошибки; гейты матрицей “кто/что/когда”; safety rails Script/WASM (CPU/Mem/IO/no‑net), whitelist источников.

HITL Learning
- Orchestrator: итерации/лимиты/критерии конвергенции; training_*; автозапуск при draft→canary; curriculum/skill graph; тест‑стенды.

Memory & Representations
- Единая STM/эпизодическая/семантическая память; индексы/векторы; объяснимость; связь с обучением (прогресс, рефлексия, планы).

Sensing & Acting
- Зрение v1→v3 (ч/б→серый→RGB); Моторика v1 (примитивы→композиции); Слух/речь (STT/TTS под флагами); Командный узел.

Observability & Ops
- Интроспекция+: policy/approvals/pending; органы/фабрика (состояния/рекомендации); Admin UI: dashboard и diff dry‑run; инциденты/алерты.
- Homeostasis: пулы/бюджеты/бэкофф; снапшоты/чекпойнты; масштабирование (позже).

Consistency & Taxonomy
- Идентификаторы kind.namespace.name[:version], единые статусы; док‑связность Nervous/Immune/Factory/Policy/Organs/Lifecycle.

---

## Execution Plan — Voice v1 via Factory (Adapter‑only)

1) StateStore & Recovery
- Добавить NodeState/Model артефакты и сервис автоподхвата (discovery→adapter.register→restore config/state).
- Интроспекция: показать восстановленные узлы/версии и состояние recovery.

2) Upgrade Hooks + Blue/Green
- Простые `migrate_state(old→new)` + shadow‑run и сравнение метрик; роуты планового upgrade/rollback (интеграция с фабрикой).

3) Runtime Config Overlay
- Узлы принимают `NodeConfig` overlay; фабрика/Policy меняют параметры на лету; Admin UI: patch‑форма.

4) Snapshot++ & Audit
- В snapshot включить templates,state,models; журнал событий с diff dry‑run/спецификаций.

5) Voice v1 pipeline
- Узлы: `analysis.text_normalize.v1`, `analysis.text_to_phonemes.v1`, `action.speak_adapter.v1`.
- Орган: `organ.voice.v1` (normalize→phonemes→speak_adapter); dry‑run→canary; метрики organ_build_* и factory_*.
