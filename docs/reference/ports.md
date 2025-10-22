<!-- neira:meta
id: NEI-20250904-120700-ports
intent: docs
summary: Сводка стандартных портов и переменных окружения для сервисов Neira.
-->
<!-- neira:meta
id: NEI-20260413-ports-rename
intent: docs
summary: Обновлены названия сервисов и команда запуска spinal_cord.
-->

# Стандартные порты

| Сервис       | Переменная         | Значение по умолчанию   |
| ------------ | ------------------ | ----------------------- |
| HTTP spinal_cord | `NEIRA_BIND_ADDR`  | `127.0.0.1:3000`        |
| Factory API  | `FACTORY_BASE_URL` | `http://localhost:3000` |

Для смены порта используйте `NEIRA_BIND_ADDR`, например `0.0.0.0:4000` или команду `npm run spinal_cord:dev --port 4000`.
