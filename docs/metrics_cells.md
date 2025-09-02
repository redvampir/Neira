# Клетки метрик и диагностики

## Навигация
- [Обзор Нейры](README.md)
- [Клетки действий](cells/action-cells.md)
- [Клетки анализа](cells/analysis-cells.md)
- [Клетки памяти](cells/memory-cells.md)
- [Архитектура анализа](system/analysis-architecture.md)
- [Поддерживающие системы](system/support-systems.md)
- [Личность Нейры](meta/personality.md)
- [Шаблон клетки](cells/cell-template.md)
- [Политика источников](system/source-policy.md)
- [Система самообновления](system/self-updating-system.md)

## Оглавление
- [MetricsCollectorCell](#metricscollectorcell)
- [DiagnosticsCell](#diagnosticscell)
- [Механизм автоисправления](#механизм-автоисправления)
- [Как отключить или ограничить мониторинг](#как-отключить-или-ограничить-мониторинг)

---

### MetricsCollectorCell

`MetricsCollectorCell` получает записи метрик `QualityMetrics` от других
клеток и пересылает их через неблокирующий канал. Клетка инкрементирует
счётчики `metrics_collector_cell_requests_total` и
`metrics_collector_cell_errors_total`, что позволяет наблюдать активность
и возможные ошибки доставки метрик.

### DiagnosticsCell

`DiagnosticsCell` подписывается на поток записей метрик, анализирует их и
реагирует при превышении допустимых значений. Для простого правила
используется поле `credibility`: значения ниже 0.5 считаются ошибкой.
Клетка ведёт счётчик ошибок и при достижении порога генерирует предупреждение
и запускает механизм автоисправления.

### Механизм автоисправления

При превышении порога `DiagnosticsCell` вызывает функцию `attempt_fix`.
Она передаётся при создании клетки и должна вернуть `true`, если проблему
удалось устранить автоматически. В случае неудачи формируется запрос
разработчику (`DeveloperRequest`) с описанием проблемы. Это позволяет
сначала пробовать быстрые исправления, а затем привлекать человека при
необходимости.

### Как отключить или ограничить мониторинг

Сбор метрик можно полностью отключить или ограничить через переменные
окружения:

- `NERVOUS_SYSTEM_ENABLED=false` — выключает публикацию всех метрик и
  эндпоинт `/metrics`.
- `PROBES_HOST_METRICS_ENABLED=false` — отключает метрики хоста.
- `PROBES_IO_WATCHER_ENABLED=false` — отключает наблюдатель задержек
  ввода‑вывода.

Пример файла `.env`:

```
NERVOUS_SYSTEM_ENABLED=false
PROBES_HOST_METRICS_ENABLED=false
PROBES_IO_WATCHER_ENABLED=false
```

