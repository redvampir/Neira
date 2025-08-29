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
    signals: [messages_saved, context_loads, context_misses, gz_rotate_count, index_compact_runs]

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
    notes: /metrics, базовые счётчики/гистограммы, warn для длинных SSE
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
    state: locked
    notes: оффлайн scripted training, только вручную

  homeostasis_budgets:
    state: locked
    notes: бюджеты CPU/IO/latency (док), в будущем back‑off по SLO

  anti_idle_core:
    state: experimental
    notes: каркас IdleDetection + Safety + Reporting без выполнения задач
    signals: [idle_state, idle_minutes_today]

  learning_microtasks:
    state: experimental
    notes: локальные учебные микрозадачи с квотами, без внешней сети
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
    notes: аудиозахват + STT с маскированием; высокий риск приватности

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
```
