<!-- neira:meta
id: NEI-20251010-organ-builder-cap-doc
intent: docs
summary: добавлены примеры активации орган-билдера.
-->
<!-- neira:meta
id: NEI-20250207-capabilities-sample-organs
intent: docs
summary: добавлены ссылки на примеры шаблонов органов.
-->
<!-- neira:meta
id: NEI-20250214-120500-lymph-filter-capability
intent: docs
summary: Добавлен компонент «Лимфатический фильтр» в перечень способностей.
-->
<!-- neira:meta
id: NEI-20270615-lymphatic-capability-update
intent: docs
summary: Уточнён статус и метрики события duplicate_found.
-->
<!-- neira:meta
id: NEI-20270318-120000-capabilities-training
intent: docs
summary: |-
  Переведены training_pipeline и learning_microtasks в experimental, описана
  роль TrainingOrchestrator и очереди микрозадач.
-->

# Neira Capabilities & Feature Gates

Purpose

- Stage new abilities safely. Start minimal, unlock gradually.
- Prevent confusion and resource thrash by enabling only what’s ready.

States

- locked: implemented but disabled by default
- experimental: enabled with safeguards/limits, monitored
- stable: enabled by default in perform mode
- deprecated: slated for removal

Owner Activation Phrases (RU)

- Разблокируй {capability}
- Включи {capability}
- Выключи / Заблокируй {capability}
- Покажи статус способностей

Assistant Policy

- Treat these phrases as intent to flip the gate (after quick risk check).
- Always report back the new state, safeguards, and rollback.

Graduation Criteria (move experimental → stable)

- Error rate/latency within SLO for N runs/time window
- No safety/policy violations observed
- Resource impact within budget

Examples (YAML)

```yaml
capabilities:
  training_pipeline:
    state: locked
    notes: offline scripted training
  self_edit:
    state: locked
    notes: modify own modules under policies
  probes_autotune:
    state: experimental
    notes: tune pools/batches via capability probes
  quarantine_auto:
    state: stable
    notes: enforce safe-mode write=admin
```

## Factory & Organs

````
capabilities:
  factory_adapter:
    state: experimental
    notes: Fabricator (Adapter only) — dry‑run/HITL, без исполнения кода
    signals: [factory_cells_created_total, factory_dryrun_requests_total]

  factory_script:
    state: locked
    notes: Rhai backend в песочнице, лимиты CPU/Mem/IO, политики

  factory_wasm:
    state: locked
    notes: WASI backend, лимиты, без внешней сети

organs_builder:
  state: experimental
  notes: Сборка органов из OrganTemplate (dry-run→canary→experimental)
  signals: [organ_build_attempts_total, organ_build_failures_total, organ_build_status_queries_total, organ_build_duration_ms, organ_status_not_found_total]

Примеры шаблонов органа: [examples/organs/organ.echo.v1.json](examples/organs/organ.echo.v1.json), [examples/organs/organ.reverse.v1.json](examples/organs/organ.reverse.v1.json).

### Пример
- «Разблокируй organs_builder» — включает капабилити на уровне experimental

## Persona & Control (дополнение)

```yaml
capabilities:
  # Control & Homeostasis
  control_pause_resume:
    state: stable
    notes: глобальная пауза/возобновление задач (admin)
    signals: [paused_state, pause_events_total]

  control_kill_switch:
    state: stable
    notes: аварийная остановка с grace-периодом (admin)
    signals: [kill_switch_total]

  inspect_snapshot:
    state: stable
    notes: сбор snapshot-срезов (логи/контекст/метрики) для анализа (admin)
    signals: [snapshots_created_total]

  trace_requests:
    state: experimental
    notes: генерация трасс по request_id (узлы/тайминги/метки)
    signals: [traces_generated_total]

  homeostasis_budgets:
    state: experimental
    notes: автотюнинг конкурентности/батчей/времени рассуждений; backpressure/backoff
    signals: [throttle_events_total, retry_backoff_applied_total]

  # Persona
  persona_kernel:
    state: stable
    notes: ядро личности (инварианты ценностей)
    signals: [persona_drift_score]

  persona_roles_minimal:
    state: stable
    notes: базовые роли coder/editor/architect
    signals: [role_switches_total]

  persona_style_neutral:
    state: stable
    notes: нейтрально‑дружелюбный стиль по умолчанию
    signals: [style_adherence]

  persona_style_teen:
    state: experimental
    notes: «подростковая» окраска; интенсивность 0–3 (0 — выкл.)
    safeguards: explicit opt‑in; auto‑reset on critical tasks
    signals: [style_adherence]

  persona_reflection:
    state: experimental
    notes: предложения микрокоррекций ядра/политик через JOURNALING (review→canary)
    safeguards: dry-run, audit trail, rollback plan
    signals: [reflection_journal_entries, proposals_accepted_total, proposals_reverted_total]

  tone_state:
    state: experimental
    notes: эфемерное настроение/тон; не трогает ценности
    safeguards: auto-reset; capped impact on latency
    signals: [style_adherence]

  studio_artflow:
    state: locked
    notes: генеративная графика (песочница)

  studio_soundweaver:
    state: locked
    notes: процедурная музыка/звук (песочница)

  studio_storycells:
    state: locked
    notes: интерактивные микро‑истории (песочница)

  roleplay_mode:
    state: locked
    notes: ролевая симуляция личности; только явно и с дисклеймером
````

## Текущие способности (стартовый набор)

```yaml
capabilities:
  communication_chat_io:
    state: stable
    notes: JSON API + SSE (meta/progress/cancel), rate-limit headers
    signals: [chat_requests_total, chat_errors_total, sse_active]

  memory_context_storage:
    state: stable
    notes: ndjson + index.json + ротация/gzip; search(content, since_id, after_ts, role, sort, offset/limit)
    signals:
      [
        messages_saved,
        context_loads,
        context_misses,
        gz_rotate_count,
        index_compact_runs,
      ]

  masking_pii:
    state: stable
    notes: пресеты, dry-run, runtime‑toggle; admin‑скоуп
    signals: [logs]

  idempotency:
    state: stable
    notes: file JSONL + TTL, интеграция в hub по request_id
    signals: [requests_idempotent_hits]

  immune_safe_mode:
    state: stable
    notes: write=admin, ручная активация, карантин‑каркас
    signals: [safe_mode]

  nervous_metrics_core:
    state: stable
    notes: /metrics, базовые счётчики/гистограммы, warn для длинных SSE; ядро Системы раздражителей
    signals: [docs/reference/metrics.md]

  probes_capability:
    state: experimental
    notes: лёгкие read‑only пробы CPU/Mem/IO; метрики без автотюнинга
    signals: [probes_*]

  introspection_status:
    state: experimental
    notes: перечень активных способностей/режимов (из CAPABILITIES.md); затем HTTP‑эндпоинт
    signals: [logs]

  journaling:
    state: experimental
    notes: журнал «что/зачем/как проверяли» с ссылкой на neira:meta id
    signals: [docs/JOURNALING.md]

  self_edit:
    state: locked
    notes: каркас под политиками; не включать

  training_pipeline:
    state: experimental
    notes: scripted training (ручной запуск + TrainingOrchestrator под флагами)

  homeostasis_budgets:
    state: locked
    notes: бюджеты CPU/IO/latency (док), в будущем back‑off по SLO

  anti_idle_core:
    state: experimental
    notes: каркас IdleDetection + Safety + Reporting без выполнения задач
    signals: [idle_state, idle_minutes_today]

  learning_microtasks:
    state: experimental
    notes: локальные учебные микрозадачи (AntiIdleMicrotaskService + автообучение)
    signals: [auto_tasks_started, auto_tasks_completed]

  reflection_journal:
    state: experimental
    notes: дневник мыслей/вопросов, краткие отчёты
    signals: [approvals_pending]

  income_generation:
    state: locked
    notes: оффлайн‑типовые задачи с оценкой ценности; включать позже

  gaming_playground:
    state: locked
    notes: безопасная песочница игр (оффлайн), лимиты сессий

  organ_vision_readonly:
    state: locked
    notes: захват экрана (белый список окон) + OCR + редакция; read‑only

  organ_vision_active:
    state: locked
    notes: активные действия на экране; только после зрелости политик/аудита

  organ_hearing:
    state: locked
    notes: |
      Аудиозахват + STT с маскированием; часть Системы раздражителей.
      Включается фразой «Разблокируй organ_hearing».
    signals: [stimuli_events_total]

  organ_voice:
    state: experimental
    notes: TTS для озвучивания отчётов/ответов

  organ_motor:
    state: locked
    notes: управление мышью/клавиатурой; только whitelisted и с согласием

  organ_fs:
    state: experimental
    notes: наблюдение за файловой системой (read‑only) в ограниченных каталогах

  organ_net_probe:
    state: locked
    notes: безопасные HEAD/GET для проверок доступности; без POST
  lymphatic_filter:
    state: experimental
    notes: фильтрация входящих сигналов через лимфатический фильтр; событие `lymphatic.duplicate_found`
```
