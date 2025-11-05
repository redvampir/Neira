<!-- neira:meta
id: NEI-20251105-desktop-mobile-arch
intent: docs
summary: |
  Архитектурный документ Desktop + Mobile UI для Neira.
  Backend только API, UI — нативные приложения.
-->

# Desktop + Mobile UI Architecture

## Motivation

Web-интерфейс через Axum показал проблемы:
- Конфликты static routes и fallback
- Сложность отладки роутинга HTML-страниц
- Ограничения браузера для доступа к аппаратным ресурсам

Neira рассчитана на использование вычислительных мощностей пользователя для обучения и адаптации. Нативные приложения лучше подходят для этой задачи.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                       Backend (Rust)                        │
│                    spinal_cord/backend                      │
│                                                             │
│  ┌───────────────────────────────────────────────────────┐ │
│  │           REST/WebSocket API                          │ │
│  │  /api/neira/consciousness/*                           │ │
│  │  /organs/*                                            │ │
│  │  /metrics                                             │ │
│  │  /events (SSE/WebSocket)                             │ │
│  └───────────────────────────────────────────────────────┘ │
│                                                             │
│  ┌───────────────────────────────────────────────────────┐ │
│  │           Core Systems                                │ │
│  │  • Consciousness                                      │ │
│  │  • Organs Builder                                     │ │
│  │  • Training Pipeline                                  │ │
│  │  • Memory Cells                                       │ │
│  └───────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                            │
                            │ HTTP/WS
          ┌─────────────────┴─────────────────┐
          │                                   │
    ┌─────▼──────┐                    ┌──────▼──────┐
    │  Desktop   │                    │   Mobile    │
    │    App     │                    │    APK      │
    │  (Tauri)   │                    │  (Android)  │
    │            │                    │             │
    │ localhost: │                    │  Remote IP: │
    │    9090    │                    │ 192.168.x.x │
    └────────────┘                    └─────────────┘
```

## Backend (spinal_cord) — API Only

### Responsibilities
- Предоставлять REST/WebSocket API для всех операций
- Управлять Consciousness, Organs, Memory
- Выполнять тяжёлые вычисления (обучение, анализ)
- Экспортировать метрики и логи

### Changes Required
1. **Удалить HTML-роутинг**
   - Убрать `/admin`, `/training`, `/organs_page` routes
   - Убрать ServeDir для `/static` (кроме legacy API docs)
   - Оставить только `/api/*` endpoints

2. **Стабилизировать API**
   - Документировать все endpoints в OpenAPI/Swagger
   - Добавить versioning (v1, v2)
   - CORS настройки для cross-origin запросов

3. **WebSocket/SSE для реального времени**
   - События consciousness
   - Прогресс обучения
   - Логи и метрики

### Deployment
```bash
# Локальный запуск
cd spinal_cord
cargo build --release
./target/release/backend

# Системный сервис (Windows Service / systemd)
# Desktop приложение будет автоматически запускать backend
```

## Desktop App (Tauri)

### Stack Options
**Option A: Tauri (recommended)**
- Rust + WebView (system browser)
- Минимальный размер (~5 MB installer)
- Полный доступ к системным API через Rust backend
- Cross-platform: Windows, macOS, Linux

**Option B: Electron**
- Node.js + Chromium
- Больше размер (~100 MB), но проще для JS-разработчиков

### Features
- **Мониторинг Backend**
  - Автостарт backend при запуске приложения
  - Health checks и автоматический перезапуск
  - Показ логов и ошибок

- **UI Модули**
  - Dashboard (Consciousness stats)
  - Training Interface (загрузка датасетов, прогресс)
  - Organs Management (создание, статус, удаление)
  - Admin Panel (конфигурация, логи)

- **Системные возможности**
  - Трей-иконка и уведомления
  - Горячие клавиши
  - Локальное хранение настроек

### Directory Structure
```
neira-desktop/
├── src-tauri/           # Rust backend для Tauri
│   ├── Cargo.toml
│   └── src/
│       └── main.rs      # Backend launcher, system integration
├── src/                 # Frontend (React/Vue/Svelte)
│   ├── components/
│   ├── pages/
│   └── api/             # API client для spinal_cord
├── package.json
└── tauri.conf.json
```

### API Client Example
```typescript
// src/api/client.ts
class NeiraClient {
  private baseUrl = 'http://localhost:9090';

  async getConsciousnessStats() {
    const res = await fetch(`${this.baseUrl}/api/neira/consciousness/stats`);
    return res.json();
  }

  async createOrgan(template: OrganTemplate) {
    const res = await fetch(`${this.baseUrl}/organs/build`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ organ_template: template, dryrun: false })
    });
    return res.json();
  }

  subscribeEvents(): EventSource {
    return new EventSource(`${this.baseUrl}/events`);
  }
}
```

## Mobile App (Android APK)

### Stack Options
**Option A: Tauri Mobile** (beta, coming soon)
- Переиспользование кода Desktop версии
- Rust + Mobile WebView

**Option B: React Native**
- Cross-platform (Android + iOS)
- Большая экосистема библиотек

**Option C: Flutter**
- Dart + Native compilation
- Хороший performance для сложных UI

### Features
- **Удалённое подключение**
  - Ввод IP:PORT backend'а
  - Сохранение профилей (home server, work machine)
  - TLS/SSL для безопасного соединения

- **Push-уведомления**
  - Завершение обучения
  - Критические ошибки
  - Новые события consciousness

- **Оффлайн-режим**
  - Кэш последних данных
  - Очередь команд для синхронизации

- **Биометрия**
  - Face ID / Fingerprint для доступа
  - Защита конфигурации

### Directory Structure
```
neira-mobile/
├── android/
├── ios/                  # Optional
├── src/
│   ├── screens/
│   ├── components/
│   └── api/              # Same client as desktop
├── package.json
└── app.json
```

## Security Considerations

### Backend API
1. **Authentication**
   - JWT tokens для API
   - API keys для mobile clients
   - Rate limiting

2. **CORS**
   ```rust
   .layer(CorsLayer::permissive()) // Dev only!
   // Prod:
   .layer(CorsLayer::new()
       .allow_origin("http://localhost:*".parse::<HeaderValue>()?)
       .allow_methods([Method::GET, Method::POST]))
   ```

3. **TLS/SSL**
   - Self-signed cert для локальной сети
   - Let's Encrypt для публичного доступа

### Mobile
1. **Защита credentials**
   - Хранение API keys в Keychain (iOS) / Keystore (Android)
   - Шифрование локального кэша

2. **Network Security**
   - Certificate pinning для TLS
   - Валидация hostname

## Development Plan

### Phase 1: Backend API Stabilization (Week 1-2)
- [ ] Удалить HTML routes из main.rs
- [ ] Документировать API в OpenAPI
- [ ] Добавить versioning (/api/v1/*)
- [ ] Настроить CORS
- [ ] Добавить WebSocket endpoint для events
- [ ] Тесты API endpoints

### Phase 2: Desktop MVP (Week 3-5)
- [ ] Инициализация Tauri проекта
- [ ] Backend launcher (автостарт spinal_cord)
- [ ] Базовый UI: Dashboard, Training, Organs
- [ ] API client integration
- [ ] Build pipeline (Windows installer)

### Phase 3: Mobile MVP (Week 6-8)
- [ ] Выбор стека (React Native / Flutter)
- [ ] Инициализация проекта
- [ ] Remote connection UI
- [ ] Core screens (Dashboard, Organs)
- [ ] Push notifications setup
- [ ] Build APK

### Phase 4: Features & Polish (Week 9+)
- [ ] Desktop: трей, уведомления, горячие клавиши
- [ ] Mobile: оффлайн-режим, биометрия
- [ ] Shared UI components library
- [ ] E2E тесты
- [ ] Performance optimization
- [ ] User documentation

## Technology Stack Summary

| Component | Technology | Justification |
|-----------|------------|---------------|
| Backend | Rust (Axum) | Уже работает, высокая производительность |
| Desktop | Tauri | Минимальный размер, Rust integration, cross-platform |
| Mobile | React Native | Проверенное решение, большая экосистема |
| UI Framework | React | Общий код между Desktop/Mobile |
| API Protocol | REST + WebSocket | Простота + real-time |
| Auth | JWT | Стандарт, stateless |

## Migration from Web UI

Текущие HTML страницы останутся как legacy reference:
```
static/
├── admin.html         → Archived
├── training.html      → Archived
├── organs.html        → Archived
└── consciousness/     → Мигрировать в Desktop/Mobile
    └── index.html
```

Все функциональность будет переписана в нативных приложениях с улучшенным UX.

## Monitoring & Metrics

Backend экспортирует метрики, Desktop/Mobile показывают их в UI:
- Consciousness: thoughts_total, decisions_made, growth_score
- Training: datasets_loaded, epochs_completed, loss
- System: cpu_usage, memory_usage, gpu_utilization

## Next Steps

1. **Обсудить с пользователем**:
   - Приоритет Desktop vs Mobile?
   - Требования к оффлайн-режиму?
   - Какие функции критичны для MVP?

2. **Создать репозитории**:
   - `neira` (backend, уже есть)
   - `neira-desktop` (Tauri app)
   - `neira-mobile` (React Native app)

3. **Начать с Backend API stabilization**:
   - Это база для обоих UI
   - Параллельно можно прототипировать Desktop

## References
- [Tauri Documentation](https://tauri.app/v1/guides/)
- [React Native Setup](https://reactnative.dev/docs/environment-setup)
- [Axum WebSocket Example](https://github.com/tokio-rs/axum/blob/main/examples/websockets/src/main.rs)
- ADR-004: Desktop + Mobile UI Architecture
