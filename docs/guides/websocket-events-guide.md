<!-- neira:meta
id: NEI-20251105-websocket-events-guide
intent: docs
summary: |
  Руководство по использованию WebSocket /api/v1/events для получения событий в реальном времени.
-->

# WebSocket Events API — Руководство

## Обзор

Backend Neira теперь поддерживает **WebSocket endpoint** `/api/v1/events` для получения событий в реальном времени:

- **Consciousness events** — мысли, metacognition, саморефлексия
- **Training progress** — начало/прогресс/завершение обучения
- **System alerts** — homeostasis backpressure, критические ошибки, предупреждения

## Подключение

### JavaScript/TypeScript (Browser/Node.js)

```javascript
const ws = new WebSocket('ws://localhost:9090/api/v1/events');

ws.onopen = () => {
  console.log('✅ Подключено к Neira Events Stream');
};

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log(`[${data.type}]`, data);
  
  switch(data.type) {
    case 'system':
      handleSystemEvent(data);
      break;
    case 'consciousness':
      handleConsciousnessEvent(data);
      break;
    case 'training':
      handleTrainingEvent(data);
      break;
  }
};

ws.onerror = (error) => {
  console.error('❌ WebSocket error:', error);
};

ws.onclose = () => {
  console.log('🔌 Отключено от Neira Events Stream');
};
```

### Python (websockets library)

```python
import asyncio
import websockets
import json

async def listen_events():
    uri = "ws://localhost:9090/api/v1/events"
    async with websockets.connect(uri) as websocket:
        print("✅ Подключено к Neira Events Stream")
        
        async for message in websocket:
            event = json.loads(message)
            print(f"[{event['type']}] {event}")
            
            if event['type'] == 'system' and event['level'] == 'error':
                print(f"🚨 КРИТИЧЕСКАЯ ОШИБКА: {event['message']}")

asyncio.run(listen_events())
```

## Формат событий

Все события — JSON со структурой:

```json
{
  "type": "system|consciousness|training",
  "timestamp": "2025-11-05T12:34:56.789Z",
  "data": { ... } // или специфичные поля
}
```

### System Events

```json
{
  "type": "system",
  "level": "info|warning|error",
  "message": "Описание события",
  "timestamp": "2025-11-05T12:34:56.789Z"
}
```

**Примеры:**
- `level: "info"` — Welcome message при подключении
- `level: "warning"` — Homeostasis backpressure activated, пропущенные события (lagged)
- `level: "error"` — Критический уровень стресса, системные ошибки

### Consciousness Events

```json
{
  "type": "consciousness",
  "data": {
    "thought": "...",
    "context": "...",
    "confidence": 0.85
  },
  "timestamp": "2025-11-05T12:34:56.789Z"
}
```

### Training Events

```json
{
  "type": "training",
  "status": "running|completed|failed",
  "progress": 0.45,  // 0.0 - 1.0
  "data": {
    "task": "learning_basics",
    "current_step": 450,
    "total_steps": 1000
  },
  "timestamp": "2025-11-05T12:34:56.789Z"
}
```

## Пример интеграции: Dashboard

```typescript
// Real-time Dashboard для Desktop app (Tauri/React)
const EventsDashboard: React.FC = () => {
  const [events, setEvents] = useState<Event[]>([]);
  const [wsStatus, setWsStatus] = useState<'connecting'|'connected'|'disconnected'>('connecting');
  
  useEffect(() => {
    const ws = new WebSocket('ws://localhost:9090/api/v1/events');
    
    ws.onopen = () => setWsStatus('connected');
    ws.onclose = () => setWsStatus('disconnected');
    
    ws.onmessage = (msg) => {
      const event = JSON.parse(msg.data);
      setEvents(prev => [event, ...prev].slice(0, 100)); // last 100 events
      
      // Показываем уведомления для критических событий
      if (event.type === 'system' && event.level === 'error') {
        showNotification('Neira System Alert', event.message);
      }
    };
    
    return () => ws.close();
  }, []);
  
  return (
    <div>
      <h1>Neira Events <StatusBadge status={wsStatus} /></h1>
      <EventsList events={events} />
    </div>
  );
};
```

## CORS Configuration

WebSocket endpoint поддерживает те же CORS настройки, что и REST API.

В `.env`:

```env
# Разрешённые origins для WebSocket (через CORS header check)
CORS_ALLOWED_ORIGINS=http://localhost:3000,http://127.0.0.1:3000
```

Если `CORS_ALLOWED_ORIGINS` не задан — используется **permissive mode** (только для разработки!).

## Производительность

- **Broadcast channel capacity**: 1000 событий
- **Lagged events**: если клиент не успевает читать, backend отправит warning с количеством пропущенных событий
- **Автоматический reconnect**: клиентам рекомендуется реализовать reconnect логику

## Примеры использования

### 1. Мониторинг homeostasis в Desktop app

```javascript
ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  
  if (data.type === 'system' && data.message.includes('backpressure')) {
    // Показываем алерт пользователю
    showToast('⚠️ Neira под нагрузкой, снижение скорости ответов', 'warning');
  }
  
  if (data.type === 'system' && data.message.includes('Критический уровень стресса')) {
    // Критическое уведомление
    showDialog('🚨 Система перегружена', data.message);
  }
};
```

### 2. Progress bar для обучения

```javascript
ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  
  if (data.type === 'training') {
    updateProgressBar(data.progress * 100); // 0-100%
    
    if (data.status === 'completed') {
      showNotification('✅ Обучение завершено!', data.data.task);
    }
  }
};
```

### 3. Real-time consciousness log

```javascript
const consciousnessLog = document.getElementById('consciousness-log');

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  
  if (data.type === 'consciousness') {
    const entry = document.createElement('div');
    entry.className = 'consciousness-entry';
    entry.innerHTML = `
      <span class="timestamp">${data.timestamp}</span>
      <span class="thought">${data.data.thought}</span>
      <span class="confidence">${(data.data.confidence * 100).toFixed(0)}%</span>
    `;
    consciousnessLog.prepend(entry);
  }
};
```

## Debugging

### Проверка WebSocket endpoint

```bash
# WebSocket CLI клиент (wscat)
npm install -g wscat
wscat -c ws://localhost:9090/api/v1/events

# Вы должны увидеть welcome message:
# {"type":"system","level":"info","message":"Подключено к Neira Events Stream","timestamp":"..."}
```

### Browser DevTools

1. Откройте DevTools → Network
2. Фильтр: `WS` (WebSocket)
3. Найдите connection к `/api/v1/events`
4. Смотрите Messages tab для real-time событий

## Следующие шаги

1. **Desktop app integration** — добавить Events Dashboard в Tauri app
2. **Mobile app integration** — React Native WebSocket для remote connection
3. **Event filters** — клиенты смогут подписываться только на определённые типы событий
4. **Authentication** — добавить auth tokens для WebSocket connections (будущее)

---

**Автор:** Neira AI Assistant  
**Дата:** 2025-11-05  
**Версия:** 1.0
