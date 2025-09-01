# Пример использования

Последовательность обработки запроса в Neira:

1. **Пользовательский запрос** — отправляется через CLI или API.
2. **InteractionHub** — принимает сообщение и определяет, какой узел активировать.
3. **AnalysisNode** — анализирует намерение и формирует план действий.
4. **MemoryNode** — извлекает или обновляет связанные записи.
5. **ActionNode** — выполняет команду (генерация кода, вывод данных и т.д.).
6. **Ответ** — результат возвращается пользователю вместе с трассировкой.

Трассировка оперирует идентификаторами узлов.

```bash
# запрос
curl -X POST http://localhost:4000/api/neira/interact \
     -H 'Content-Type: application/json' \
     -d '{"message":"Список задач"}'

# ответ
{
  "reply": "Задачи: [\"task1\", \"task2\"]",
  "trace": [
    {"id": "AnalysisNode/main", "status": "ok"},
    {"id": "MemoryNode/tasks", "status": "hit"},
    {"id": "ActionNode/list", "result": ["task1", "task2"]}
  ]
}
```

## Требования к окружению

- Linux x86_64, 4 ядра CPU и 8 ГБ RAM.
- Node.js 20 LTS.
- Rust 1.75.

## Запуск модулей

```bash
# установка зависимостей
npm install
# подготовка окружения
npm run setup
# запуск API сервера на http://localhost:4000
npm run dev
```

## Маршруты API

- `POST /api/neira/interact` — общий вход для пользовательских запросов.
- `POST /api/neira/analysis` — выполнение конкретного `AnalysisNode`.
- `POST /api/neira/action` — запуск `ActionNode`.

## Organ Builder CLI

```bash
# запуск сборки органа из шаблона
cargo run -p backend --bin organ_builder -- build organ.json
# проверка статуса
cargo run -p backend --bin organ_builder -- status organ-1
# отмена сборки
cargo run -p backend --bin organ_builder -- cancel organ-1
```

По умолчанию используется адрес `http://127.0.0.1:3000`. Иной URL можно
передать последним аргументом или через переменную окружения
`NEIRA_API_BASE`.
