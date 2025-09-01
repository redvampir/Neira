<!-- neira:meta
id: NEI-20250923-nervous-system-docs
intent: design
summary: Описание Нервной системы Neira: цели, компоненты (пробы/метрики/живость/вотчдог), интеграции, ENV и диагностика.
-->

<!-- neira:meta
id: NEI-20250214-watchdog-env-docs
intent: docs
summary: Добавлен раздел про WATCHDOG* переменные.
-->

<!-- neira:meta
id: NEI-20260214-loop-detector-docs
intent: docs
summary: Описан детектор повторов SSE и переменные LOOP_*.
-->

# Нервная система (Nervous System)

Цели
- Прозрачность и самонаблюдение: сбор RED/USE метрик, состояние очередей, живость SSE, тайминги узлов.
- Раннее обнаружение деградаций: watchdog’и, пороговые сигналы, рекомендации по лимитам.
- Единые интерфейсы интеграции: метрики, интроспекция, события для обучающих/фабричных подсистем.

Границы ответственности
- Сбор, агрегирование и экспорт метрик (`/metrics`).
- Применение «мягких» защит (рекомендации, предупреждения); «жёсткие» меры — в плоскости Control/Immune.
- Не изменяет бизнес‑логику узлов; только наблюдает и сигнализирует.

Компоненты
- Экспортёр метрик: Prometheus HTTP (`/metrics`).
- Пробы (probes):
  - Host Metrics: CPU/Mem (sysinfo), публикация `host_*` gauge.
  - IO Watcher: latency ввода/вывода, `io_*` histogram, событийные `system.io.*` записи.
  - Heartbeat/Живость: `sse_active` gauge, пульс через admin UI.
- Watchdogs: soft/hard таймауты выполнения анализа/потоков, счётчики и рекомендации по ENV.
- Loop Detector: анализирует поток SSE на повторы и низкую энтропию, публикует `loop_detected_total`.
- Интроспекция: `/api/neira/introspection/status` блоки `watchdogs`, `queues/backpressure`, `anti_idle`, `capabilities`.

Интеграции (хуки)
- Узлы (Analysis/Action/Chat):
  - публикуют `*_requests_total`, `*_errors_total`, histogram длительностей.
  - помечают «активность пользователя» (Anti‑Idle) при входных событиях.
- Фабрика/Органы:
  - публикуют `factory_*`, `organ_*` метрики (см. docs/reference/metrics.md).
  - добавляют статусы в интроспекцию (read‑only).

ENV (минимум)
- NERVOUS_SYSTEM_ENABLED (bool, default=true) — включить /metrics и сбор проб.
- PROBES_HOST_METRICS_ENABLED (bool, default=true) — включить host‑пробу.
- PROBES_IO_WATCHER_ENABLED (bool, default=false) — включить IO‑пробу; IO_WATCHER_THRESHOLD_MS — порог задержки.
- WATCHDOG_REASONING_SOFT_MS / WATCHDOG_REASONING_HARD_MS — таймауты.
- BACKPRESSURE_HIGH_WATERMARK / BACKPRESSURE_THROTTLE_MS — пороги и базовый сон; AUTO_BACKOFF_ENABLED/BP_MAX_BACKOFF_MS — авто‑бэкофф.

## WATCHDOG* переменные

Настройки сторожей времени задаются через окружение:

- `WATCHDOG_REASONING_SOFT_MS` — мягкий таймаут рассуждений (мс).
- `WATCHDOG_REASONING_HARD_MS` — жёсткий таймаут (мс).
- `WATCHDOG_SOFT_MS_<NODEID>` / `WATCHDOG_HARD_MS_<NODEID>` — переопределения для узлов (ID в UPPER_SNAKE_CASE).

Метрики (ссылки)
- См. [docs/reference/metrics.md](../reference/metrics.md): `host_*`, `io_*`, `sse_active`, [`throttle_events_total`](../reference/metrics.md#homeostasis--control-дополнение), `watchdog_*`, [`backpressure`](../reference/metrics.md#реестр-метрик-истина), а также блоки Anti‑Idle (`idle_*`).

Диагностика и SLO
- Базовые панели: загрузка CPU/Mem, длины очередей, время отклика узлов, число активных SSE, срабатывания watchdog.
- Алерты: «частые троттлинги» (throttle), «высокое давление» (backpressure), soft/hard timeout.
- Трассы: при `TRACE_ENABLED=1` — GET `/api/neira/trace/:request_id` для разбора конкретных запросов.

Связь с Homeostasis
- НС подаёт сигналы для Homeostasis (бюджеты, паузы/бэкофф), не принимает решений «убить» сам по себе.
  Решения применяются в Control Plane или Immune System по политикам.

См. также
- docs/design/homeostasis.md — принципы баланса/лимитов.
- docs/reference/env.md — полный список ENV.
- docs/reference/metrics.md — перечень метрик.
