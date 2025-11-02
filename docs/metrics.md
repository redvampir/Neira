<!-- neira:meta
id: NEI-20251101-120500-training-metrics-doc
intent: docs
summary: |
  Обновлено описание тренировочных метрик, добавлен пример работы AdaptiveScheduler
  и фиксации прогресса.
-->

# Метрики обучения

## Основные метрики

- `learning_success_rate` - процент успешных ответов (gauge)
- `learning_attempts` - общее количество попыток (counter)
- `current_difficulty` - текущий уровень сложности (gauge)

## Адаптивное обучение

Система автоматически корректирует сложность на основе успешности:

- Повышение сложности: успешность > 80% за последние 5 попыток
- Понижение сложности: успешность < 60% за последние 5 попыток

## Интеграция с Prometheus

Метрики доступны через стандартный /metrics endpoint.

## Persistence

Прогресс обучения сохраняется в JSON-файл, путь настраивается через NEIRA_DATA_DIR.

# Конфигурация обучения

## Переменные окружения

- `NEIRA_SUCCESS_THRESHOLD` (default: 0.8) - порог успешности для повышения сложности
- `NEIRA_FAILURE_THRESHOLD` (default: 0.6) - порог неудач для понижения сложности  
- `NEIRA_MIN_ATTEMPTS` (default: 5) - минимальное количество попыток для адаптации
- `NEIRA_DATA_DIR` (default: "data") - директория для сохранения прогресса

## API Metrics

```rust
// Основные метрики
learning_success_rate: gauge   // % успешных ответов
learning_attempts: counter     // Общее число попыток
current_difficulty: gauge      // Текущий уровень сложности  
```

## Примеры использования

```rust
let metrics = LearningMetrics::new();

// Запись попытки
metrics.record_attempt(true, 0.5).await;

// Сохранение прогресса
metrics.save_progress().await?;

// Адаптивная сложность  
let new_difficulty = metrics.adjust_difficulty().await;
```


## Интеграция с планировщиком

`AdaptiveScheduler` автоматически использует `adjust_difficulty()` и `save_progress()` при обновлении статистики. Пример цикла:

```rust
use std::sync::Arc;
use spinal_cord::training::{
    curriculum::RussianLiteracyCurriculum,
    metrics::LearningMetrics,
    scheduler::AdaptiveScheduler,
};

let metrics = Arc::new(LearningMetrics::new());
let curriculum = Arc::new(RussianLiteracyCurriculum::load_default()?);
let scheduler = AdaptiveScheduler::new(curriculum, metrics.clone());

let lesson = scheduler.next_lesson().await;
scheduler.complete_lesson(&lesson, 0.72).await?;
```

После вызова `complete_lesson` обновлённые статистики сохраняются в `${NEIRA_DATA_DIR}/learning_progress.json`, а метрики остаются доступными по `/metrics`.