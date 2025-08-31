<!-- neira:meta
id: NEI-20250830-Homeostasis-Adaptive-Control
intent: docs
summary: |
  Контур гомеостаза и адаптивного управления: автоподстройка лимитов под железо/нагрузку/политики, сторожевые таймеры и контроль владельца (pause/resume/kill + snapshot/trace). Гейтинг через capabilities.
-->

# Homeostasis & Adaptive Control

Цель: без «жёстких констант» и ручной перенастройки — подстраивать поведение узлов под реальные ресурсы и цели, сохраняя живучесть и контроль владельца.

Основные компоненты
- Probes (нервная система): измерения CPU/Mem/IO/сети, пульс SSE, длительности узлов.
- Budgets (бюджеты): динамические лимиты конкурентности, размера батчей, времени рассуждений.
- Backpressure & Backoff: обратное давление и экспоненциальные повторы с джиттером.
- Watchdogs (сторожи): soft/hard таймауты, детектор циклов/повторов, эскалация в quarantine.
- Control Plane (владелец): pause/resume/kill, snapshot, trace, статус.

Как это работает (схема)
1) Calibration: при старте — короткие пробы (host, диски, сеть) → стартовые лимиты.
2) Feedback: периодически (T секунд) пересчёт лимитов по p95/ошибкам/нагрузке.
3) Local-first: решения на уровне узлов; глобальные политики лишь задают рамки (ceilings/floors).
4) Safeguards: при аномалиях — понижаем агрессию, включаем backoff/квоты, возможно quarantine.
5) Control: владелец может в любой момент поставить «паузу», снять срез (snapshot) и продолжить/остановить.

Бюджеты (примеры «крутилок»)
- concurrency_limit_{class}: макс. параллелизм для классов узлов (chat, analysis, storage, io).
- batch_size_{class}: размер батчей для операций, авто‑тюнинг по ошибкам/латентности.
- reasoning_time_budget_ms: мягкий лимит на «размышления» с деградацией после soft‑порога.
- memory_window_bytes/lines: скользящее окно контекста под I/O и задержки.

Watchdogs
- soft_timeout_ms: предупреждение, переключение в упрощённый план.
- hard_timeout_ms: прерывание шага, сохранение следов, quarantine при повторе.
- loop_detector: окно повторов токенов/шаблонов, порог для срабатывания.

Control Plane (эндпоинты, admin)
- POST /api/neira/control/pause {auth, reason?}
- POST /api/neira/control/resume {auth}
- POST /api/neira/control/kill {auth, grace_ms?}
- GET  /api/neira/control/status → { paused, reason?, since_ms, active_tasks, backpressure }
- GET  /api/neira/inspect/snapshot?include=logs,context,metrics → архив/NDJSON
- GET  /api/neira/trace/:request_id → трасса узлов/тайминги/метки

Capabilities (см. CAPABILITIES.md)
- homeostasis_budgets (experimental): автотюнинг бюджетов и backoff.
- control_pause_resume (stable): глобальная пауза/возобновление.
- control_kill_switch (stable): экстренная остановка с грацией и аудитом.
- inspect_snapshot (stable): сбор среза состояния.
- trace_requests (experimental): генерация трасс по request_id.

Метрики (см. docs/reference/metrics.md)
- throttle_events_total, retry_backoff_applied_total
- watchdog_timeouts_total, loop_detected_total
- paused_state (gauge), pause_events_total, kill_switch_total
- snapshots_created_total, traces_generated_total

ENV (см. docs/reference/env.md)
- HOMEOSTASIS_ENABLED, HOMEOSTASIS_RECALC_INTERVAL_MS
- WATCHDOG_REASONING_SOFT_MS, WATCHDOG_REASONING_HARD_MS
- LOOP_DETECT_ENABLED, LOOP_WINDOW_TOKENS, LOOP_REPEAT_THRESHOLD
- CONTROL_ALLOW_PAUSE, CONTROL_ALLOW_KILL, CONTROL_SNAPSHOT_DIR, TRACE_ENABLED

Связи
- Anti‑Idle System: планирование микрозадач в окна простоя (минимум вмешательств).
- Organ Systems: «орган гомеостаза» координирует локальные правила узлов.
- Roadmap: Stage 0 — control_pause/kill/snapshot; Stage 1 — homeostasis_budgets/trace.



��. �����: design/anti-idle-system.md, design/nervous-system.md

