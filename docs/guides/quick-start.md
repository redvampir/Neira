<!-- neira:meta
id: NEI-20251105-quick-start-guide
intent: docs
summary: |
  Краткое руководство по началу работы с Neira Desktop + Mobile.
-->

# Neira Quick Start Guide

## Для пользователей

### Desktop App (рекомендуется)

**Скоро доступно!** Мы работаем над нативным Desktop приложением.

**Текущий статус**: В разработке (Phase 2, см. [План миграции](../DESKTOP_MOBILE_MIGRATION_PLAN.md))

**Что будет доступно**:
- ✨ Простая установка (один .msi файл для Windows)
- 🚀 Автоматический запуск backend
- 📊 Красивый Dashboard с метриками
- 🧠 Управление Consciousness
- 🔬 Интерфейс обучения
- 🦠 Управление органами
- 🔔 Уведомления о событиях

**Примерная дата релиза**: Декабрь 2025

---

### Mobile App (Android)

**Скоро доступно!** APK для удалённого управления Neira.

**Текущий статус**: В планах (Phase 3)

**Что будет доступно**:
- 📱 Подключение к домашнему серверу
- 📊 Мониторинг в реальном времени
- 🔔 Push-уведомления
- 🔐 Биометрическая аутентификация
- 📴 Оффлайн-режим

**Примерная дата релиза**: Январь 2026

---

### Временное решение: Web UI (Legacy)

Пока нативные приложения в разработке, можно использовать базовый web-интерфейс:

```powershell
# 1. Запустить backend
cd f:\Neyra\neira
.\run.bat

# 2. Открыть браузер
start http://localhost:9090/static/consciousness/
```

**⚠️ Ограничения Legacy UI**:
- Минимальный функционал
- Будет удалён после релиза Desktop app
- Некоторые страницы могут не работать

---

## Для разработчиков

### Запуск Backend (API only)

Backend уже работает как REST API сервис:

```powershell
# Сборка
cd f:\Neyra\neira\spinal_cord
cargo build --release

# Запуск
cd f:\Neyra\neira
.\spinal_cord\target\release\backend.exe

# Проверка
curl http://localhost:9090/api/neira/consciousness/stats
```

**API Documentation**: См. `docs/api/openapi.yaml` (скоро)

---

### Разработка Desktop App

```bash
# 1. Клонировать репозиторий (когда будет создан)
git clone https://github.com/your-org/neira-desktop
cd neira-desktop

# 2. Установить зависимости
npm install

# 3. Запустить в dev режиме
npm run tauri dev

# 4. Собрать installer
npm run tauri build
```

**Требования**:
- Node.js 18+
- Rust 1.75+
- Windows: Visual Studio Build Tools

**Документация**: `neira-desktop/README.md`

---

### Разработка Mobile App

```bash
# 1. Клонировать репозиторий (когда будет создан)
git clone https://github.com/your-org/neira-mobile
cd neira-mobile

# 2. Установить зависимости
npm install

# 3. Запустить на эмуляторе
npm run android

# 4. Собрать APK
cd android
./gradlew assembleRelease
```

**Требования**:
- Node.js 18+
- Android Studio
- JDK 11+

**Документация**: `neira-mobile/README.md`

---

## API Examples

### Get Consciousness Stats

```bash
curl http://localhost:9090/api/neira/consciousness/stats
```

**Response**:
```json
{
  "thoughts_total": 42,
  "decisions_made": 15,
  "growth_score": 0.73,
  "last_thought": "2025-11-05T12:34:56Z"
}
```

---

### Create Organ

```bash
curl -X POST http://localhost:9090/organs/build \
  -H "Content-Type: application/json" \
  -d '{
    "organ_template": {
      "name": "my_analyzer",
      "type": "analysis",
      "capabilities": ["text_analysis"]
    },
    "dryrun": false
  }'
```

**Response**:
```json
{
  "organ_id": "organ-abc123",
  "status": "building"
}
```

---

### Subscribe to Events (WebSocket)

```javascript
const ws = new WebSocket('ws://localhost:9090/api/v1/events');

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Event:', data);
};

ws.onerror = (error) => {
  console.error('WebSocket error:', error);
};
```

**Event types**:
- `consciousness.thought`
- `consciousness.decision`
- `organ.created`
- `training.progress`
- `system.error`

---

## Часто задаваемые вопросы

### Q: Когда будет доступен Desktop app?
**A**: Ориентировочно декабрь 2025. Следите за обновлениями в репозитории.

### Q: Будет ли iOS версия?
**A**: Пока в планах только Android. iOS возможна после стабилизации Android версии.

### Q: Можно ли использовать старый web UI?
**A**: Да, но он будет удалён после релиза Desktop app. Не рекомендуется для постоянного использования.

### Q: Как подключиться к Neira с другого компьютера?
**A**: Backend слушает на `0.0.0.0:9090`, доступен по IP вашего компьютера:
```
http://192.168.1.100:9090/api/neira/consciousness/stats
```

### Q: Нужна ли регистрация/аккаунт?
**A**: Пока нет. Аутентификация будет добавлена в будущих версиях.

### Q: Где хранятся данные Neira?
**A**: В каталоге проекта:
- `data/` — memory cells
- `logs/` — логи
- `config/` — конфигурация

---

## Поддержка

**Баги и вопросы**: [GitHub Issues](https://github.com/your-org/neira/issues)  
**Документация**: [docs/](../docs/)  
**План развития**: [DESKTOP_MOBILE_MIGRATION_PLAN.md](../DESKTOP_MOBILE_MIGRATION_PLAN.md)

---

## Следующие шаги

1. ⭐ **Star** репозиторий для отслеживания обновлений
2. 📖 Прочитать [Architecture Overview](../docs/architecture/desktop-mobile-ui.md)
3. 🛠️ Попробовать API (см. примеры выше)
4. 🐛 Сообщить о багах в [Issues](https://github.com/your-org/neira/issues)
5. 💬 Присоединиться к обсуждению в Discord (ссылка скоро)

---

**Обновлено**: 2025-11-05  
**Версия**: 1.0.0-alpha
