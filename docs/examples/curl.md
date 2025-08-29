# Примеры cURL

> Замените `AUTH=secret` при необходимости. Базовый адрес: http://127.0.0.1:3000

## Chat (JSON)
```bash
AUTH=secret
curl -sS -X POST http://127.0.0.1:3000/api/neira/chat \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $AUTH" \
  -d '{"node_id":"echo.chat","chat_id":"demo","session_id":null,"message":"hello","persist":false}'
```

## Chat Stream (SSE)
```bash
AUTH=secret
curl -sS -N -X POST http://127.0.0.1:3000/api/neira/chat/stream \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $AUTH" \
  -d '{"node_id":"echo.chat","chat_id":"demo","session_id":"sess-123","message":"stream me"}'
```

## Search (content-only)
```bash
AUTH=secret
curl -sS "http://127.0.0.1:3000/api/neira/chat/demo/sess-123/search?q=hello&regex=0&role=user&sort=desc&offset=0&limit=10"
```

## Masking: preset
```bash
AUTH=secret
curl -sS -X POST http://127.0.0.1:3000/api/neira/context/masking \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $AUTH" \
  -d '{"preset":"pii_strict"}'
```

## Capabilities (пока вручную)
Смотрите CAPABILITIES.md и используйте фразы:
- «Покажи статус способностей» — список гейтов
- «Разблокируй {capability}» — включение
- «Заблокируй {capability}» — выключение

