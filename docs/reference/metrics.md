<!-- neira:meta
id: NEI-20251010-organ-builder-metrics
intent: docs
summary: добавлена метрика organ_build_duration_ms и статусные запросы.
-->
# Реестр Метрик (Истина)

| Имя | Тип | Единицы | Где инкрементируется | Назначение |
|---|---|---|---|---|
| chat_requests_total | counter | req | SynapseHub | Входящие чат‑запросы |
| chat_errors_total | counter | err | SynapseHub | Ошибки авторизации/валидации/лимитов |
| chat_response_time_ms | histogram | ms | SynapseHub | Время ответа чат‑клетки |
| analysis_requests_total | counter | req | SynapseHub | Входящие анализ‑запросы |
| analysis_errors_total | counter | err | SynapseHub | Ошибки анализа/тайм‑ауты/отмена |
| analysis_cell_request_duration_ms | histogram | ms | SynapseHub | Длительность анализа (сред/квантили) |
| chat_cell_requests_total | counter | req | EchoChatCell | Вызовы чат‑клетки |
| chat_cell_errors_total | counter | err | EchoChatCell | Ошибки чат‑клетки |
| chat_cell_request_duration_ms | histogram | ms | EchoChatCell | Длительность обработки клеткой |
| messages_saved | counter | msg | FileContextStorage | Сохранённые сообщения |
| context_loads | counter | op | FileContextStorage | Загрузки контекста |
| context_misses | counter | op | FileContextStorage | Промахи загрузки |
| context_bytes_written | counter | bytes | FileContextStorage | Записанные байты контекста |
| gz_rotate_count | counter | ops | FileContextStorage | Архивные ротации gz |
| sessions_created_total | counter | ops | Hub/Session | Созданные сессии |
| sessions_deleted_total | counter | ops | Session delete | Удалённые сессии |
| sessions_closed_total | counter | ops | Session delete | Закрытия сессий (для отчётов) |
| sessions_active | gauge | count | Hub/Session init+ops | Активные сессии |
| sessions_autocreated_total | counter | ops | Hub (persist auto) | Автосозданные сессии |
| requests_idempotent_hits | counter | ops | Hub (LRU+file) | Кэш‑попадания идемпотентных ответов |
| index_compact_runs | counter | ops | Compaction job | Запуски компактера |
| sse_active | gauge | count | SSE stream | Активные SSE потоки |
| backpressure | gauge | count | BackpressureProbe | Суммарная длина очередей |
| throttle_events_total | counter | events | BackpressureProbe | События троттлинга при backpressure |
| safe_mode | gauge | 0/1 | Hub | Статус безопасного режима |
| idle_state | gauge | 0..3 | Anti-Idle | Текущее состояние простоя |
| idle_minutes_today | counter | min | Anti-Idle | Минуты простоя за день |
| auto_tasks_started | counter | tasks | Anti-Idle | Запущено авто‑задач |
| auto_tasks_completed | counter | tasks | Anti-Idle | Завершено авто‑задач |
| auto_tasks_blocked | counter | tasks | Anti-Idle | Заблокировано SafetyController |
| approvals_pending | gauge | count | Anti-Idle | Запросы на одобрение в очереди |
| autonomous_time_spent_seconds | counter | s | Anti-Idle | Секунды автономной работы |
| microtask_queue_depth | gauge | count | Anti-Idle | Глубина очереди микрозадач |
| immune_alerts_total{severity} | counter | alerts | Immune System | Алерты иммунной системы (лейбл severity) |

## Связь с дорожной картой (Embryo / Stage 0–1)

- Stage 0 (Core Stable) — базовые метрики, активны по умолчанию:
  - chat_requests_total, chat_errors_total, chat_response_time_ms
  - messages_saved, context_loads, context_misses, context_bytes_written, gz_rotate_count, index_compact_runs
  - sessions_created_total, sessions_deleted_total, sessions_closed_total, sessions_active
  - requests_idempotent_hits, sse_active, safe_mode

- Stage 1 (Experimental Growth) — включаются при открытии соответствующих гейтов (см. CAPABILITIES.md):
  - idle_state, idle_minutes_today, autonomous_time_spent_seconds
  - auto_tasks_started, auto_tasks_completed, auto_tasks_blocked, microtask_queue_depth
  - approvals_pending

См. также:
- Дорожная карта: docs/roadmap.md
- Anti‑Idle System: docs/design/anti-idle-system.md
- Способности и гейты: CAPABILITIES.md

Примечание: именование согласовано с кодом (backend/src). При добавлении новых метрик — обновляйте эту таблицу.

---

## Homeostasis & Control (дополнение)

- throttle_events_total (counter): число случаев троттлинга/снижения агрессии.
- retry_backoff_applied_total (counter): повторы с backoff.
- watchdog_timeouts_total (counter): срабатывания soft/hard сторожей.
- loop_detected_total (counter): обнаруженные циклы/повторы в рассуждениях.
- paused_state (gauge 0/1): глобальная пауза активна.
- pause_events_total (counter): число переключений паузы.
- kill_switch_total (counter): активации аварийной остановки.
- snapshots_created_total (counter): сформированные snapshot‑срезы.
- traces_generated_total (counter): сгенерированные трассы по request_id.
- pause_drain_events_total (counter): операции дренажа активных SSE при паузе.
- loop_detected_total (counter): срабатывания детектора повторов в SSE.
- budget_hits_total (counter): срабатывания лимита токенов для SSE.
- backpressure (gauge): суммарная длина очередей.
- throttle_events_total (counter): события троттлинга при backpressure.
- watchdog_timeouts_total{kind=soft|hard} (counter): срабатывания сторожей рассуждений.

См. также: docs/design/homeostasis.md
- pause_reason_total{reason} (counter): количество пауз по причинам (использовать осторожно из-за кардинальности).

---

## Immune System

<!-- neira:meta
id: NEI-20250720-immune-alert-metric-docs
intent: docs
summary: документирован счётчик immune_alerts_total с лейблом severity.
-->
- immune_alerts_total{severity} (counter): алерты иммунной системы (лейбл severity).
<!-- neira:meta
id: NEI-20250505-000000-immune-action-metrics-docs
intent: docs
summary: документированы immune_actions_total и immune_action_failures_total.
-->
- immune_actions_total{action=quarantine|safe_mode|integrity_check|init_config} (counter): успешные иммунные действия.
- immune_action_failures_total{action=quarantine|safe_mode|integrity_check|init_config} (counter): неудачные иммунные действия.

---

## Factory & Organs (draft)

<!-- neira:meta
id: NEI-20250215-factory-auto-metrics
intent: docs
summary: документированы метрики auto_heal и auto_rollback.
-->
<!-- neira:meta
id: NEI-20250310-factory-auto-failure-metrics-docs
intent: docs
summary: добавлены счётчики неудачных auto_heal и auto_rollback.
-->
<!-- neira:meta
id: NEI-20250320-factory-auto-response-duration
intent: docs
summary: документирована метрика factory_auto_response_duration_ms с лейблом action.
-->
<!-- neira:meta
id: NEI-20250607-factory-disabled-gauge-docs
intent: docs
summary: документирован gauge factory_cells_disabled.
-->
<!-- neira:meta
id: NEI-20250704-factory-state-transition-metric-docs
intent: docs
summary: документирован счётчик factory_state_transitions_total с лейблами from/to.
-->
| metric | type | unit | scope | description |
|---|---|---|---|---|
| factory_cells_created_total | counter | cells | Factory | Создано клеток (всего) |
| factory_cells_active | gauge | cells | Factory | Активные фабричные клетки |
| factory_cells_disabled | gauge | cells | Factory | Отключённые фабричные клетки |
| factory_exec_errors_total | counter | errors | Factory | Ошибки исполнения (backend) |
| factory_dryrun_requests_total | counter | req | Factory | Запросы dry‑run |
| factory_approvals_total | counter | ops | Factory | Подтверждения HITL |
| factory_rollbacks_total | counter | ops | Factory | Откаты клеток |
| factory_state_transitions_total{from,to} | counter | ops | Factory | Переходы состояний (лейблы from,to) |
| factory_auto_heals_total | counter | ops | Factory | Авто‑отключения по тревоге |
| factory_auto_rollbacks_total | counter | ops | Factory | Авто‑откат по тревоге |
| factory_auto_heal_failures_total | counter | ops | Factory | Неудачные авто‑отключения |
| factory_auto_rollback_failures_total | counter | ops | Factory | Неудачные авто‑откаты |
| factory_auto_response_duration_ms{action=heal\|rollback} | histogram | ms | Factory | Длительность auto_heal/auto_rollback (лейбл action: тип реакции) |
| organ_build_attempts_total | counter | ops | OrganBuilder | Попытки сборки органов |
| organ_build_failures_total | counter | ops | OrganBuilder | Ошибки сборки органов |
| organ_build_status_queries_total | counter | ops | OrganBuilder | Запросы статуса органа |
<!-- neira:meta
id: NEI-20250317-organ-status-error-metric
intent: docs
summary: document organ_build_status_errors_total metric.
-->
| organ_build_status_errors_total | counter | ops | OrganBuilder | Ошибки обновления статуса |
| organ_build_duration_ms | histogram | ms | OrganBuilder | Время от Draft до Stable |
| organ_status_not_found_total | counter | ops | OrganBuilder | Запросы статуса к несуществующим органам |
| organ_build_restored_total | counter | ops | OrganBuilder | Восстановленные органы при запуске |
| training_iterations_total | counter | iters | Training | Итерации обучения новых клеток |
| training_converged_total | counter | iters | Training | Конвергировали до стабильности |
