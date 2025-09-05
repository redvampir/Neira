# Веб-интерфейс обучения Neira

<!-- neira:meta
id: NEI-20250904-120910-web-interface-cell-registry
intent: docs
summary: Добавлен пример регистрации клеток через CellRegistry.
-->
<!-- neira:meta
id: NEI-20241112-120000-web-interface-main-entry
intent: docs
summary: Уточнён входной модуль фронтенда.
-->

## Навигация

- [Обзор Нейры](README.md)
- [Клетки действий](action-cells.md)
- [Клетки анализа](analysis-cells.md)
- [Клетки памяти](memory-cells.md)
- [Архитектура анализа](analysis-architecture.md)
- [Системы поддержки](support-systems.md)
- [Личность Нейры](personality.md)
- [Шаблон клетки](cell-template.md)
- [Политика источников](source-policy.md)
- [Механизм саморазвивающейся системы](self-updating-system.md)

## Оглавление

- [Архитектура](#архитектура)
- [Режимы доступа](#режимы-доступа)
- [Функциональные модули](#функциональные-модули)
- [Безопасность и мониторинг](#безопасность-и-мониторинг)
- [Рекомендации по реализации](#рекомендации-по-реализации)

## Архитектура

- **Backend на Rust (Axum + tokio)** обслуживает REST и WebSocket API: `/api/neira/*` для чата и обучения, `/ws` для трансляции прогресса.
- **Хранилище** может начинаться с SQLite и при росте перейти на PostgreSQL; в логах и сегментах фиксируются `bookId`, `segmentId` и флаг `reliability`.
- **Frontend как PWA** на React/Vue подключается к API, устанавливается на смартфон или десктоп, кэширует статику и офлайн‑запросы.
  - Входной модуль: `src/main.js`.

## Режимы доступа

- **Локальный**: приложение взаимодействует с `http://localhost:3001` и имеет полный доступ к SynapseHub и логам.
- **Удалённый**: доступ через туннели (ngrok, Cloudflare) или собственный домен с HTTPS; аутентификация через JWT и ограничение прав клеток.

## Основные эндпоинты

Все REST-запросы проходят через префикс `/api/neira`.

| Метод | Маршрут                  | Назначение                               |
| ----- | ------------------------ | ---------------------------------------- |
| POST  | `/api/neira/interact`    | общий вход для пользовательских запросов |
| POST  | `/api/neira/analysis`    | выполнение `AnalysisCell`                |
| POST  | `/api/neira/action`      | запуск `ActionCell`                      |
| POST  | `/api/neira/personality` | переключение образа Нейры                |
| WS    | `/ws`                    | трансляция прогресса обучения            |

```rust
use neira::{cells::{ActionCell, AnalysisCell, MemoryCell}, CellRegistry};

fn init_registry() -> CellRegistry {
    let mut registry = CellRegistry::default();
    // регистрация тестовых клеток
    struct DummyAction;
    impl ActionCell for DummyAction {}
    struct DummyAnalysis;
    impl AnalysisCell for DummyAnalysis {}
    struct DummyMemory;
    impl MemoryCell for DummyMemory {}
    registry.register_action("dummy.action", Box::new(DummyAction));
    registry.register_analysis("dummy.analysis", Box::new(DummyAnalysis));
    registry.register_memory("dummy.memory", Box::new(DummyMemory));
    registry
}
```

## Функциональные модули

- **Чат / SynapseHub** — единая точка общения и управления памятью.
- **Панель обучения** — загрузка сегментов, просмотр `reliability`, постановка задач в очередь ручной проверки; работает с `TeacherClient` по [спецификации](training.md#api-teacherclient):

  Пример запроса:

  ```json
  {
    "bookId": "isbn:9780000000000",
    "segmentId": "ch01-0001",
    "prompt": "Summarize the following segment",
    "text": "<segment contents>",
    "temperature": 0.2
  }
  ```

  Пример ответа:

  ```json
  {
    "bookId": "isbn:9780000000000",
    "segmentId": "ch01-0001",
    "summary": "...",
    "reliability": "medium",
    "tokensUsed": 123
  }
  ```

- **Мониторинг** — визуализация очередей `TaskScheduler`, уведомления о ходе обучения через WebSocket.
- **Управление клеткими** — promotion/rollback версий и просмотр статуса `CellRegistry`.

## Безопасность и мониторинг

- HTTPS и короткоживущие JWT.
- Rate limiting и журнал событий SynapseHub.
- Использование `tracing`/`prometheus` для логов и метрик.

## Рекомендации по реализации

1. **Backend**: модули `synapse_hub`, `training`, `cell_management`, `scheduler_ws`; длительные задачи выносить в отдельные `tokio::spawn` воркеры.
2. **Frontend**: Service Worker для кэширования, очередь API‑запросов в офлайн‑режиме, WebSocket для прогресса.
3. **CLI/TUI**: опциональный интерфейс на тех же API для локального использования.
4. **Развёртывание**: начать с туннеля, затем перейти на собственный домен и Docker/Nginx при переходе в продакшен.
