<!-- neira:meta
id: NEI-20251105-api-v1-migration
intent: docs
summary: |
  Руководство по миграции на /api/v1/* endpoints и стандартизированный формат ответов.
-->

# API v1 Migration Guide

## Обзор изменений

Backend Neira теперь поддерживает **версионированные API endpoints** под `/api/v1/*` с **единообразным форматом ответов**.

### Формат ответа

Все новые `/api/v1/*` endpoints возвращают JSON:

```json
{
  "status": "success",
  "data": { ... },
  "error": null
}
```

При ошибке:

```json
{
  "status": "error",
  "data": null,
  "error": "Описание ошибки"
}
```

## Маппинг endpoints

| Legacy Path                          | New Path (v1)                   | Статус    |
|--------------------------------------|---------------------------------|-----------|
| `/cells`                             | `/api/v1/cells`                 | ✅ Ready  |
| `/organs`                            | `/api/v1/organs`                | ✅ Ready  |
| `/voice/speak`                       | `/api/v1/voice/speak`           | ✅ Ready  |
| `/api/neira/chat`                    | `/api/v1/chat`                  | ✅ Ready  |
| `/api/neira/training/run`            | `/api/v1/training/run`          | ✅ Ready  |
| `/api/neira/consciousness/thought`   | `/api/v1/consciousness/thought` | ✅ Ready  |
| `/api/neira/homeostasis/status`      | `/api/v1/homeostasis/status`    | ✅ Ready  |
| N/A                                  | `/api/v1/events` (WebSocket)    | ✅ New    |

### WebSocket Events (NEW!)

```
ws://localhost:9090/api/v1/events
```

Real-time события: consciousness, training, system alerts.  
См. [websocket-events-guide.md](./websocket-events-guide.md)

## Обратная совместимость

**Legacy paths сохранены** (без изменений):
- `/cells`, `/organs`, `/voice/*` — работают как раньше
- `/api/neira/*` — работают как раньше

**Deprecated:** Legacy paths будут удалены в v2.0 (Q2 2026).

## Примеры миграции

### Before (Legacy)

```javascript
// Legacy endpoint
const response = await fetch('http://localhost:9090/api/neira/chat', {
  method: 'POST',
  body: JSON.stringify({ message: 'Привет!' }),
});

// Формат ответа зависит от endpoint
const data = await response.json(); // может быть string, object, array...
```

### After (v1)

```javascript
// Versioned endpoint
const response = await fetch('http://localhost:9090/api/v1/chat', {
  method: 'POST',
  body: JSON.stringify({ message: 'Привет!' }),
});

// Единообразный формат
const result = await response.json();
if (result.status === 'success') {
  console.log('Ответ:', result.data);
} else {
  console.error('Ошибка:', result.error);
}
```

## TypeScript Types

```typescript
// Стандартный API ответ
interface ApiResponse<T = any> {
  status: 'success' | 'error';
  data?: T;
  error?: string;
}

// Пример использования
async function chatRequest(message: string): Promise<ApiResponse<string>> {
  const res = await fetch('/api/v1/chat', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ message }),
  });
  
  return await res.json();
}

// Type-safe вызов
const result = await chatRequest('Привет!');
if (result.status === 'success') {
  console.log(result.data); // TypeScript знает, что это string
}
```

## Endpoints Catalog (v1)

### Cells API
- `POST /api/v1/cells` — регистрация cell
- `GET /api/v1/cells/:id` — последняя версия
- `GET /api/v1/cells/:id/:version` — конкретная версия

### Factory API
- `POST /api/v1/factory/cells/dryrun` — dry-run создания
- `POST /api/v1/factory/cells` — создать cell
- `POST /api/v1/factory/cells/:fid/approve` — approve
- `POST /api/v1/factory/cells/:fid/disable` — disable
- `POST /api/v1/factory/cells/:fid/rollback` — rollback

### Voice API
- `POST /api/v1/voice/speak` — TTS
- `POST /api/v1/voice/transcribe` — STT

### Organs API
- `GET /api/v1/organs` — список organs
- `POST /api/v1/organs/build` — билд нового organ
- `DELETE /api/v1/organs/:id/build` — отмена билда
- `POST /api/v1/organs/:id/rebuild` — rebuild
- `GET /api/v1/organs/:id/status` — статус
- `POST /api/v1/organs/:id/status` — обновить статус
- `GET /api/v1/organs/:id/stream` — SSE stream

### Analysis API
- `POST /api/v1/analysis` — анализ
- `POST /api/v1/analysis/resume` — resume анализа

### Chat API
- `POST /api/v1/chat` — отправить сообщение
- `POST /api/v1/chat/stream` — streaming chat
- `POST /api/v1/chat/session/new` — новая сессия
- `POST /api/v1/chat/stream/cancel` — отмена stream
- `GET /api/v1/chat/:chat_id/export` — экспорт
- `POST /api/v1/chat/:chat_id/import/:session_id` — импорт
- `GET /api/v1/chat/:chat_id/index` — индекс сессий
- `GET /api/v1/chat/:chat_id/:session_id` — получить сессию
- `DELETE /api/v1/chat/:chat_id/:session_id` — удалить сессию
- `POST /api/v1/chat/:chat_id/:session_id/rename` — переименовать
- `GET /api/v1/chat/:chat_id/:session_id/search` — поиск

### Training API
- `POST /api/v1/training/run` — запуск тренировки
- `GET /api/v1/training/status` — статус
- `GET /api/v1/training/stream` — SSE progress

### Context API
- `POST /api/v1/context/masking` — обновить masking
- `GET /api/v1/context/masking/config` — конфиг masking
- `POST /api/v1/context/masking/dry_run` — dry-run

### Probes API
- `POST /api/v1/probes/:name/toggle` — toggle probe

### Homeostasis API
- `GET /api/v1/homeostasis/status` — статус stress/backpressure

### Dialogue API
- `POST /api/v1/dialogue/evolving` — evolving dialogue
- `GET /api/v1/dialogue/stats` — статистика роста

### Consciousness API
- `POST /api/v1/consciousness/thought` — record thought
- `GET /api/v1/consciousness/stats` — метакогнитивная статистика
- `GET /api/v1/consciousness/growth-report` — отчёт роста
- `GET /api/v1/consciousness/personality` — текущая личность
- `POST /api/v1/consciousness/personality/snapshot` — snapshot

### WebSocket API
- `GET /api/v1/events` — WebSocket real-time events

## Проверка миграции

```bash
# Проверить v1 endpoint
curl http://localhost:9090/api/v1/homeostasis/status

# Ответ:
# {"status":"success","data":{"stress_level":0.1,"budgets":{...}}}
```

## Следующие шаги

1. **Desktop app** будет использовать только `/api/v1/*`
2. **Mobile app** (React Native) также только v1
3. **Legacy clients** продолжат работать до Q2 2026
4. **OpenAPI spec** будет создана для auto-generated клиентов

---

**Автор:** Neira AI Assistant  
**Дата:** 2025-11-05  
**Версия:** 1.0
