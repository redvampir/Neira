<!-- neira:meta
id: NEI-20251105-migration-plan
intent: docs
summary: |
  План пошаговой миграции с Web UI на Desktop + Mobile архитектуру.
-->

# Desktop + Mobile Migration Plan

**Статус**: В процессе  
**Дата начала**: 2025-11-05  
**Предполагаемая длительность**: 8-10 недель  

## Обзор

Переход от веб-интерфейса (HTML-страницы через Axum) к нативным Desktop и Mobile приложениям.

**Причины миграции**:
1. Проблемы с роутингом HTML-страниц в Rust backend
2. Необходимость использования аппаратных ресурсов пользователя для обучения
3. Нативные возможности (push-уведомления, оффлайн, биометрия)
4. Лучший UX для мобильных устройств

**Архитектурное решение**: ADR-004 (см. DECISIONS.md)

---

## Фазы миграции

### Phase 0: Подготовка (✅ ЗАВЕРШЕНО, 2025-11-05)

**Цель**: Зафиксировать решение, создать план, обсудить детали с пользователем.

#### Задачи
- [x] Создать ADR-004 в DECISIONS.md
- [x] Создать документ docs/architecture/desktop-mobile-ui.md
- [x] Пометить web_ui_legacy как deprecated в CAPABILITIES.md
- [x] Создать этот план миграции
- [x] **Обсудить с пользователем** — ✅ СОГЛАСОВАНО:
  - ✅ **Приоритет**: Desktop first (максимум вычислительных мощностей)
  - ✅ **MVP функции**: Обучение, создание органов, P2P связь между Neira-экземплярами
  - ✅ **Timeline**: 2 месяца (до начала января 2026)
  - ✅ **Тестирование**: Параллельно разработке (пользователь обучает Neira)
  - ✅ **Платформы**: Windows, Linux, Android (без Apple — ограничения App Store)
  - ✅ **Стек**: Tauri (минимальный размер, нативный performance, Rust backend)

#### Выходные артефакты
- ✅ ADR-004
- ✅ Архитектурный документ
- ✅ План миграции
- ✅ Согласованные приоритеты с пользователем

**Статус**: READY TO START Phase 1 🚀

---

### Phase 1: Backend API Stabilization (2 недели)

**Цель**: Превратить backend в чистый REST/WebSocket API сервис.

#### Задачи Backend Cleanup
- [ ] **Удалить HTML routes** из `spinal_cord/src/main.rs`:
  - `/admin` → удалить
  - `/training` → удалить
  - `/organs_page` → удалить
  - `.nest_service("/static", ...)` → удалить (кроме /static/consciousness для legacy)
  
- [ ] **Стандартизировать API endpoints**:
  - Все под `/api/v1/*`
  - Версионирование в URL
  - Единообразные ответы (status, data, error)

- [ ] **Документировать API** (OpenAPI/Swagger):
  ```bash
  # Создать openapi.yaml
  touch docs/api/openapi.yaml
  ```
  - Все endpoints
  - Request/Response schemas
  - Authentication
  - Error codes

- [ ] **Настроить CORS**:
  ```rust
  use tower_http::cors::{CorsLayer, Any};
  
  let cors = CorsLayer::new()
      .allow_origin(Any) // Dev only
      .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
      .allow_headers(Any);
  
  let app = Router::new()
      .layer(cors);
  ```

- [ ] **WebSocket endpoint для events**:
  ```rust
  // GET /api/v1/events (WebSocket upgrade)
  .route("/api/v1/events", get(websocket_handler))
  ```
  - Отправка consciousness events
  - Training progress
  - System alerts

#### Задачи API Testing
- [ ] Написать integration тесты для всех endpoints
- [ ] Проверить CORS в браузере
- [ ] Нагрузочное тестирование WebSocket
- [ ] Документировать rate limits

#### Выходные артефакты
- Backend без HTML routes
- OpenAPI спецификация
- Работающий WebSocket endpoint
- Integration тесты
- Updated README.md (API примеры)

---

### Phase 2: Desktop MVP (Tauri) (3 недели)

**Цель**: Работающее Desktop приложение с P2P и обучением.

#### Week 1: Setup & Backend Integration
- [ ] **Инициализация Tauri проекта**:
  ```bash
  cd f:\Neyra
  npm create tauri-app@latest neira-desktop
  cd neira-desktop
  ```
  
- [ ] **Backend launcher** (Rust side):
  ```rust
  // src-tauri/src/main.rs
  use std::process::Command;
  
  fn start_backend() -> Result<Child, io::Error> {
      Command::new("F:\\Neyra\\neira\\spinal_cord\\target\\release\\backend.exe")
          .spawn()
  }
  ```

- [ ] **Health check & auto-restart**:
  - Проверять http://localhost:9090/health каждые 10 сек
  - Перезапускать при падении

- [ ] **API client** (Frontend):
  ```typescript
  // src/api/client.ts
  class NeiraClient {
    baseUrl = 'http://localhost:9090/api/v1';
    
    async getConsciousnessStats() { /* ... */ }
    async createOrgan(template: OrganTemplate) { /* ... */ }
    subscribeEvents(): WebSocket { /* ... */ }
  }
  ```

- [ ] **P2P Integration** (см. docs/architecture/p2p-collaboration.md):
  - libp2p в backend
  - UI: список peers
  - Отправка/получение задач

#### Week 2: Core UI + Training Interface
- [ ] **Dashboard** (`src/pages/Dashboard.tsx`):
  - Consciousness stats
  - System health (CPU/GPU usage)
  - Connected peers list
  - Quick actions

- [ ] **Training Interface** (`src/pages/Training.tsx`):
  - Dataset upload (drag-and-drop)
  - Training start/stop/pause
  - Real-time progress (epochs, loss, accuracy)
  - GPU utilization monitoring
  - History/logs with filtering

- [ ] **System Resource Monitor**:
  - CPU/GPU temperature
  - Memory usage
  - Disk I/O
  - Network bandwidth

#### Week 3: Organs + P2P Tasks
- [ ] **Organs Management** (`src/pages/Organs.tsx`):
  - List organs (local + shared from peers)
  - Create from template
  - View status/logs
  - Install organ from peer

- [ ] **P2P Task Board** (`src/pages/Tasks.tsx`):
  - Active tasks (local + distributed)
  - Task queue
  - Send task to peer
  - Idle task settings

- [ ] **System Integration**:
  - Tray icon (minimize to tray)
  - Notifications (training done, peer connected)
  - Settings (backend port, auto-start, P2P config)
  - Dark/Light theme

- [ ] **Build & Distribute**:
  ```bash
  npm run tauri build
  # Windows: .msi installer в src-tauri/target/release/bundle
  # Linux: .deb/.AppImage в src-tauri/target/release/bundle
  ```

#### Выходные артефакты
- `neira-desktop` репозиторий
- Windows installer (.msi) + Linux packages (.deb, .AppImage)
- User guide (docs/guides/desktop-app.md)
- Screenshots + demo video

---

### Phase 3: Mobile MVP (Android) (3 недели)

**Цель**: Базовое Android приложение с удалённым подключением.

#### Week 1: Setup & Connection
- [ ] **Выбор стека** (согласовать с пользователем):
  - Option A: React Native (рекомендуется, переиспользование React кода)
  - Option B: Flutter (лучший performance)
  - Option C: Tauri Mobile (beta, ждём стабилизации)

- [ ] **Инициализация проекта**:
  ```bash
  # React Native
  npx react-native init NeiraAndroid
  cd NeiraAndroid
  ```

- [ ] **Connection UI**:
  - Ввод IP:PORT
  - Тест соединения
  - Сохранение профилей (home, work)

- [ ] **API client** (переиспользовать с Desktop):
  ```typescript
  const client = new NeiraClient({
    baseUrl: `http://${userIP}:9090/api/v1`
  });
  ```

#### Week 2: Core Screens
- [ ] **Dashboard** (mobile-friendly):
  - Cards layout
  - Swipe actions
  - Pull-to-refresh

- [ ] **Organs List**:
  - Список с фильтрами
  - Детали органа
  - Quick actions

- [ ] **Notifications**:
  - Firebase Cloud Messaging setup
  - Backend integration (отправка уведомлений)

#### Week 3: Security & Build
- [ ] **Secure storage**:
  ```typescript
  import * as Keychain from 'react-native-keychain';
  
  await Keychain.setGenericPassword('api_key', userApiKey);
  ```

- [ ] **TLS validation**:
  - Certificate pinning
  - Hostname verification

- [ ] **Build APK**:
  ```bash
  cd android
  ./gradlew assembleRelease
  # APK в android/app/build/outputs/apk/release
  ```

- [ ] **Testing**:
  - На реальном устройстве
  - Удалённое подключение через WiFi/VPN

#### Выходные артефакты
- `neira-mobile` репозиторий
- Android APK
- User guide (docs/guides/mobile-app.md)
- Screenshots

---

### Phase 4: Polish & Features (2 недели)

**Цель**: Улучшения UX, performance, дополнительные функции.

#### Desktop
- [ ] Горячие клавиши (Ctrl+Shift+N для нового органа и т.д.)
- [ ] Drag-and-drop для загрузки датасетов
- [ ] Log viewer с фильтрами
- [ ] Dark/Light theme toggle

#### Mobile
- [ ] Оффлайн-режим (кэш последних данных)
- [ ] Биометрия (Face ID / Fingerprint)
- [ ] Локализация (RU/EN)
- [ ] Swipe gestures

#### Shared
- [ ] E2E тесты (Playwright/Detox)
- [ ] Performance monitoring
- [ ] Crash reporting (Sentry)
- [ ] User analytics (opt-in)

#### Выходные артефакты
- Polished UI
- Performance reports
- Updated documentation

---

## Критерии завершения миграции

### Backend
- ✅ Все HTML routes удалены
- ✅ API полностью документирован
- ✅ WebSocket events работают
- ✅ CORS настроен
- ✅ Integration тесты проходят

### Desktop
- ✅ Backend автостарт
- ✅ Dashboard, Training, Organs экраны работают
- ✅ Tray icon и уведомления
- ✅ Windows installer доступен
- ✅ User guide написан

### Mobile
- ✅ Удалённое подключение работает
- ✅ Core screens реализованы
- ✅ Push-уведомления настроены
- ✅ APK собран и протестирован
- ✅ User guide написан

### Documentation
- ✅ ADR-004 создан
- ✅ Architecture doc обновлён
- ✅ API reference готов
- ✅ User guides написаны
- ✅ Migration notes для разработчиков

---

## Риски и митигация

| Риск | Вероятность | Воздействие | Митигация |
|------|-------------|-------------|-----------|
| Backend API нестабилен | Средняя | Высокое | Extensive testing, versioning |
| Tauri learning curve | Низкая | Среднее | Хорошая документация, примеры |
| Mobile SSL/TLS сложности | Средняя | Среднее | Let's Encrypt, detailed guide |
| User confusion при миграции | Высокая | Низкое | Clear communication, migration guide |
| Performance на старых устройствах | Средняя | Среднее | Optimization, minimum specs |

---

## Rollback Plan

Если миграция не удастся, можно вернуться к Web UI:

1. Git revert изменений в `spinal_cord/src/main.rs`
2. Восстановить HTML routes
3. Пометить Desktop/Mobile как experimental
4. Продолжать использовать `http://localhost:9090/static/consciousness/`

**Условия rollback**:
- Критические баги в API
- Невозможность достичь производительности Web UI
- Негативная обратная связь от пользователя

---

## Метрики успеха

### Adoption
- [ ] Desktop app установлен и используется ежедневно
- [ ] Mobile app используется для удалённого мониторинга

### Performance
- [ ] Desktop: < 100 MB RAM, < 5% CPU idle
- [ ] Mobile: < 50 MB RAM, < 200 MB APK size
- [ ] API latency: < 100ms p95

### Quality
- [ ] 0 critical bugs в production
- [ ] > 80% test coverage для API
- [ ] User satisfaction: положительная обратная связь

---

## Следующие шаги (сейчас)

1. **Обсудить с пользователем** (приоритет #1):
   - Desktop vs Mobile: что важнее?
   - Какие функции критичны для MVP?
   - Готов ли тестировать промежуточные версии?

2. **Начать Phase 1** (если согласие получено):
   ```bash
   # Очистка backend от HTML routes
   cd f:\Neyra\neira\spinal_cord
   git checkout -b feature/api-only-backend
   # Редактирование main.rs
   ```

3. **Создать репозитории**:
   ```bash
   cd f:\Neyra
   mkdir neira-desktop neira-mobile
   git init neira-desktop
   git init neira-mobile
   ```

---

## ✅ Согласованные решения (2025-11-05)

1. **Desktop**: ✅ Tauri (минимальный размер, нативный performance)
2. **Mobile**: ✅ React Native (большая экосистема, переиспользование React)
3. **iOS**: ❌ Нет (ограничения App Store для «вредоносного ПО»)
4. **Платформы**: ✅ Windows, Linux, Android
5. **Backend deployment**: ✅ LAN + P2P между экземплярами Neira
6. **Authentication**: ⏳ Позже (сначала trusted network)
7. **Timeline**: ✅ 2 месяца (декабрь 2025 — январь 2026)
8. **MVP функции**:
   - ✅ Обучение с максимальным использованием GPU/CPU
   - ✅ Создание и управление органами
   - ✅ P2P связь между Neira на разных устройствах
   - ✅ Совместное решение задач в простое
9. **Исходники компиляторов**: ✅ Перемещены в F:\Neyra\compiler-sources (см. ADR-005)
10. **Neira Language**: ✅ Параллельная разработка собственного IR/DSL (см. docs/architecture/neira-language.md)

---

## 🚀 Параллельные треки разработки

### Track A: Desktop + Mobile (основной)
- Phase 1: Backend API cleanup (2 недели)
- Phase 2: Desktop MVP (3 недели)
- Phase 3: Mobile MVP (3 недели)

### Track B: Neira Language (параллельно)
- Week 1-2: IR basics (instruction set + interpreter)
- Week 3-4: Rust → IR compiler (simple functions)
- Week 5-6: Training integration (datasets + lessons)
- Week 7-8: Self-modification experiments

**Синергия**: Desktop app будет использовать IR для hot-reload органов!

---

**Автор**: Neira AI Assistant  
**Дата создания**: 2025-11-05  
**Последнее обновление**: 2025-11-05 (добавлен Neira Language track)  
**Связанные документы**: ADR-004, ADR-005, docs/architecture/desktop-mobile-ui.md, docs/architecture/neira-language.md  
**Статус**: 🚀 APPROVED — Phase 1 + IR development начинаются параллельно
