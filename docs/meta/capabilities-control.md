<!-- neira:meta
id: NEI-20250830-Capabilities-Control
intent: docs
summary: |
  Контрольный контур и гомеостаз: capability‑флаги для паузы/остановки, срезов/трасс и автотюнинга бюджетов. Дополнение к CAPABILITIES.md.
-->

# Control & Homeostasis Capabilities (дополнение)

```yaml
capabilities:
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
```

См. также: docs/design/homeostasis.md, docs/api/backend.md, docs/reference/metrics.md
