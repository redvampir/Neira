# Узлы метрик и диагностики

## Навигация
- [Обзор Нейры](README.md)
- [Узлы действий](nodes/action-nodes.md)
- [Узлы анализа](nodes/analysis-nodes.md)
- [Узлы памяти](nodes/memory-nodes.md)
- [Архитектура анализа](system/analysis-architecture.md)
- [Поддерживающие системы](system/support-systems.md)
- [Личность Нейры](meta/personality.md)
- [Шаблон узла](nodes/node-template.md)
- [Политика источников](system/source-policy.md)
- [Система самообновления](system/self-updating-system.md)

## Оглавление
- [MetricsCollectorNode](#metricscollectornode)
- [DiagnosticsNode](#diagnosticsnode)
- [Механизм автоисправления](#механизм-автоисправления)
- [Как отключить или ограничить мониторинг](#как-отключить-или-ограничить-мониторинг)

---

### MetricsCollectorNode

`MetricsCollectorNode` получает записи метрик `QualityMetrics` от других
узлов и пересылает их через неблокирующий канал. Узел инкрементирует
счётчики `metrics_collector_node_requests_total` и
`metrics_collector_node_errors_total`, что позволяет наблюдать активность
и возможные ошибки доставки метрик.

### DiagnosticsNode

`DiagnosticsNode` подписывается на поток записей метрик, анализирует их и
реагирует при превышении допустимых значений. Для простого правила
используется поле `credibility`: значения ниже 0.5 считаются ошибкой.
Узел ведёт счётчик ошибок и при достижении порога генерирует предупреждение
и запускает механизм автоисправления.

### Механизм автоисправления

При превышении порога `DiagnosticsNode` вызывает функцию `attempt_fix`.
Она передаётся при создании узла и должна вернуть `true`, если проблему
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

