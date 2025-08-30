<!-- neira:meta
id: NEI-20250923-env-anti-idle
intent: docs
summary: Добавлен флаг ANTI_IDLE_ENABLED и его описание (addendum к ENV).
-->

# Anti‑Idle ENV Addendum

Purpose
- Временное дополнение к `docs/reference/env.md` до синхронизации таблиц.

| Переменная | Тип | Дефолт | Раздел | Описание |
|---|---|---|---|---|
| ANTI_IDLE_ENABLED | bool | true | anti-idle core | Включает каркас Anti‑Idle (только метрики, без автозадач). |

Notes
- Каркас Anti‑Idle обновляет: `idle_state` (gauge: 0..3), `idle_minutes_today` (counter), `microtask_queue_depth` (gauge: 0), `time_since_activity_seconds` (gauge).
- Пороговые значения читаются из ENV: `IDLE_THRESHOLD_SECONDS` (сек), `LONG_IDLE_THRESHOLD_MINUTES` (мин), `DEEP_IDLE_THRESHOLD_MINUTES` (мин).
