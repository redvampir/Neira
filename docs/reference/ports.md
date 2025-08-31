<!-- neira:meta
id: NEI-20250831-ports
intent: docs
summary: Сводка стандартных портов и переменных окружения для сервисов Neira.
-->

# Стандартные порты

| Сервис       | Переменная         | Значение по умолчанию   |
| ------------ | ------------------ | ----------------------- |
| HTTP backend | `NEIRA_BIND_ADDR`  | `127.0.0.1:3000`        |
| Factory API  | `FACTORY_BASE_URL` | `http://localhost:3000` |

Для смены порта используйте `NEIRA_BIND_ADDR`, например `0.0.0.0:4000` или команду `npm run backend:dev --port 4000`.
