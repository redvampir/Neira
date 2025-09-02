# API: POST /api/neira/chat

Описание запроса для отправки сообщения в чат-клетка (Chat Cell).

- Метод: `POST`
- Путь: `/api/neira/chat`
- Тип: `application/json`

Тело запроса:

```
{
  "cell_id": "echo.chat",          // обязательный: идентификатор чат-клетки
  "chat_id": "support",            // обязательный: идентификатор чата (папка контекста)
  "session_id": "2024-08-28-a",    // необязательный: ID сессии, чтобы подключить прошлый контекст
  "message": "Привет!",            // обязательный: входное сообщение
  "auth": "<token>",              // обязательный: токен доступа
  "persist": true,                 // необязательный: сохранять историю независимо от session_id
  "request_id": "abc-123"         // необязательный: идемпотентность запроса
}
```

Пояснения:
- Если `session_id` указан, Neira загружает историю сообщений этой сессии из `ContextStorage` и сохраняет новые сообщения в ту же сессию.
- Если `session_id` не указан, контекст прошлых бесед не подключается и новые сообщения не сохраняются (статусный обмен без следа).
- Ответ:

```
{
  "response": "Привет!",          // пример ответа (для EchoChatCell — это эхо)
  "used_context": true             // брался ли исторический контекст по session_id
}
```

Замечания по авторизации и лимитам:
- Заголовки не требуются, все поля передаются в JSON.
- Применяется поминутное ограничение запросов (см. конфигурацию `SynapseHub`).

Потоковые ответы (SSE):
- Путь: `POST /api/neira/chat/stream` — тело запроса идентично `POST /api/neira/chat`.
- События:
  - `meta` — JSON `{ "used_context": bool }`
  - `message` — частичные куски ответа
  - `done` — признак завершения

Управление сессиями:
- `POST /api/neira/chat/session/new` — выдаёт новый `session_id`. Тело: `{ "auth": "<token>", "prefix": "opt" }`.
- `DELETE /api/neira/chat/:chat_id/:session_id?auth=<token>` — удаляет файлы сессии и запись из индекса.
- `POST /api/neira/chat/:chat_id/:session_id/rename` — тело: `{ "auth": "<token>", "new_session_id": "..." }`.

Runtime‑маскирование PII:
- `POST /api/neira/context/masking` — тело: `{ "auth": "<token>", "enabled": true, "regex": ["..."], "roles": ["user","assistant"] }` — применяется без перезапуска.

Пагинация/фильтрация выдачи сессии:
- `GET /api/neira/chat/:chat_id/:session_id` — поддерживает query:
  - `offset`, `limit` — постраничная выборка по строкам
  - `since_id` — только сообщения с `message_id` больше указанного
  - `after_ts` — только сообщения с `timestamp_ms` больше указанного
