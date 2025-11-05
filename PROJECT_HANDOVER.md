# Neira — Живая Программа

**Версия**: 2.0 (ноябрь 2025)  
**Язык**: Rust (backend) + TypeScript/React (будущий UI)  
**Лицензия**: MIT

---

## 🎯 Что это?

**Neira** — автономная система искусственного интеллекта, спроектированная как **организм-программа**:
- 🧠 Самонаблюдение и адаптивность (homeostasis, метрики, capability probes)
- 🤝 Партнёрство с человеком (совместные решения, feature gates)
- 💬 Эволюционирующие диалоги с памятью (semantic memory, embeddings)
- 🔄 Саморазвитие (consciousness, meta-cognition, personality evolution)

---

## 🚀 Быстрый старт

### Требования
- Rust 1.75+ (`rustup install stable`)
- Node.js 20+ (для будущего UI)
- Python 3.9+ (для инструментов)
- PostgreSQL 14+ (опционально, для persistence)

### Установка
```bash
# 1. Клонировать репозиторий
git clone https://github.com/redvampir/Neira.git
cd Neira

# 2. Настроить окружение
cp .env.example .env
# Отредактируй .env (CORS_ALLOWED_ORIGINS, API_VERSION, etc.)

# 3. Собрать backend
cargo build --release

# 4. Запустить сервер
cargo run --release
```

Сервер стартует на `http://localhost:9090`.

### Первая проверка
```bash
# Проверить heartbeat
curl http://localhost:9090/api/v1/heartbeat

# Подключиться к WebSocket events
wscat -c ws://localhost:9090/api/v1/events
```

---

## 📂 Структура проекта

```
neira/
├── spinal_cord/          # Backend (Rust/Axum)
│   ├── src/              # Основной код
│   │   ├── main.rs       # HTTP сервер, роутинг, WebSocket
│   │   ├── lib.rs        # Публичные модули
│   │   └── modules/      # Подсистемы (память, диалоги, etc.)
│   ├── config/           # Конфигурационные файлы (.toml)
│   └── Cargo.toml        # Зависимости
├── sensory_organs/       # Сенсорные модули (микрофон, камера)
├── docs/                 # Документация
│   ├── architecture/     # Архитектурные решения
│   ├── guides/           # Руководства пользователя
│   ├── api/              # API спецификации
│   └── archive/          # Устаревшие документы
├── tests/                # Интеграционные тесты
├── tools/                # Утилиты для разработки
│   ├── extract_changelog.py  # Генерация CHANGELOG
│   └── minimize_comments.py  # Минимизация комментариев
├── schemas/              # JSON Schema для валидации
├── .env                  # Переменные окружения (НЕ коммитить!)
├── AGENTS.md             # Инструкции для ИИ-агентов
├── CAPABILITIES.md       # Feature gates и возможности
├── DECISIONS.md          # ADR (архитектурные решения)
└── README.md             # Этот файл
```

---

## 🔧 Конфигурация

### .env переменные

```bash
# Backend
PORT=9090
HOST=0.0.0.0

# CORS (для cross-origin запросов)
CORS_ALLOWED_ORIGINS=http://localhost:3000,http://localhost:8080

# API версия
API_VERSION=1.0.0

# WebSocket events
WEBSOCKET_EVENTS_ENABLED=true

# Homeostasis (автономная регуляция)
HOMEOSTASIS_CHECK_INTERVAL_SECS=10
HOMEOSTASIS_BACKPRESSURE_THRESHOLD=0.7

# Режим работы (explore|perform|safe-mode)
NEIRA_AUTONOMY_MODE=perform

# Логирование
RUST_LOG=info,backend=debug
```

---

## 📡 API Endpoints

### v1 API (стабильная версия)

**Base URL**: `/api/v1`

#### 🔹 Система
- `GET /api/v1/heartbeat` — Проверка состояния сервера
- `GET /api/v1/events` — WebSocket для real-time событий

#### 🔹 Cells (клетки системы)
- `POST /api/v1/cells` — Зарегистрировать новую cell
- `GET /api/v1/cells/:id` — Получить последнюю версию cell
- `GET /api/v1/cells/:id/:version` — Получить конкретную версию

#### 🔹 Диалоги
- `POST /api/v1/dialogue/evolving` — Отправить сообщение в эволюционирующий диалог
- `POST /api/v1/chat` — Отправить простое сообщение в чат

#### 🔹 Обучение
- `POST /api/v1/training/start` — Начать процесс обучения
- `GET /api/v1/training/progress` — Проверить прогресс обучения
- `POST /api/v1/training/stop` — Остановить обучение

#### 🔹 Consciousness (сознание)
- `POST /api/v1/consciousness/metacognition/update` — Обновить метакогнитивное состояние
- `POST /api/v1/consciousness/personality/evolve` — Триггер эволюции личности
- `GET /api/v1/consciousness/daily-report` — Получить дневной отчёт о росте

#### 🔹 Голос
- `POST /api/v1/voice/transcribe` — Транскрибировать аудио (speech-to-text)
- `POST /api/v1/voice/synthesize` — Синтезировать речь (text-to-speech)

**Полный список**: см. `docs/api/endpoints-v1.md`

---

## 🌐 WebSocket Events

### Подключение
```javascript
const ws = new WebSocket('ws://localhost:9090/api/v1/events');

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Event:', data);
};
```

### Форматы событий

#### 🔸 System событие
```json
{
  "type": "system",
  "level": "info|warning|error",
  "message": "Homeostasis stress: 0.65",
  "timestamp": "2025-11-05T12:34:56Z"
}
```

#### 🔸 Consciousness событие
```json
{
  "type": "consciousness",
  "data": {
    "thought": "Анализирую свои действия...",
    "confidence": 0.85
  },
  "timestamp": "2025-11-05T12:34:56Z"
}
```

#### 🔸 Training событие
```json
{
  "type": "training",
  "data": {
    "status": "running",
    "progress": 0.42,
    "current_step": "processing_examples"
  },
  "timestamp": "2025-11-05T12:34:56Z"
}
```

**Подробнее**: см. `docs/guides/websocket-events-guide.md`

---

## 🧪 Тестирование

```bash
# Все тесты
cargo test --all

# Интеграционные тесты
cargo test --test '*'

# Линтеры
cargo clippy -- -D warnings

# Форматирование
cargo fmt
```

---

## 🛠️ Инструменты разработки

### Генерация CHANGELOG
```bash
python tools/extract_changelog.py --output CHANGELOG.md
```
Извлекает все `NEI-*` комментарии из кода и создаёт журнал изменений.

### Минимизация комментариев
```bash
python tools/minimize_comments.py --path spinal_cord/src/main.rs
```
Упрощает verbose `neira:meta` блоки в компактный формат.

---

## 🤖 Работа с ИИ-агентами

**Neira поддерживает автономных ИИ-агентов** для саморазвития.

### Инструкции для агентов
1. **Прочитай `AGENTS.md`** — базовые правила работы
2. **Изучи `CAPABILITIES.md`** — доступные feature gates
3. **Следуй `WORKFLOW.md`** — пошаговый процесс задач
4. **Читай `DECISIONS.md`** — история архитектурных решений

### Режимы автономии
- `explore`: свободное экспериментирование (низкий риск)
- `perform`: рабочий режим с эскалацией средних/высоких рисков
- `safe-mode`: требуется подтверждение для записей

**Установить режим**: добавь в `.env`
```bash
NEIRA_AUTONOMY_MODE=perform
```

---

## 📚 Документация

- **Architecture**: `docs/architecture/`
  - `desktop-mobile-ui.md` — Концепция Desktop/Mobile UI
  - `neira-language.md` — Neira Language (IR, интерпретатор)
  - `p2p-collaboration.md` — P2P сеть (libp2p, mDNS, gossipsub)

- **Guides**: `docs/guides/`
  - `quick-start.md` — Быстрый старт для пользователей
  - `websocket-events-guide.md` — Подробный гайд по WebSocket
  - `api-v1-migration.md` — Миграция на API v1

- **API**: `docs/api/`
  - `endpoints-v1.md` — Полный список API endpoints
  - `openapi.yaml` — OpenAPI спецификация (в разработке)

---

## 🌟 Ключевые концепции

### 1. Адаптивность
- Никаких жёстких констант — все параметры через конфиг/ENV
- Capability probes: оценка CPU/RAM/IO → динамическая подстройка
- Feature flags: включение/выключение функций без рестарта

### 2. Автономия
- Режимы работы: `explore`, `perform`, `safe-mode`
- Homeostasis Engine: автоматическая регуляция ресурсов
- Policy Engine: разрешения и ограничения через правила

### 3. Эволюция
- Semantic Memory: контекстная память с embeddings
- Meta-Cognition: рефлексия и самонаблюдение
- Personality Evolution: развитие личности через опыт

### 4. Партнёрство
- Feature Gates: разблокировка возможностей через команды
- Escalation Rules: запрос подтверждения для рискованных действий
- Versioned API: обратная совместимость для плавных обновлений

---

## 🔮 Roadmap

### Phase 1 (в разработке)
- [x] HTML routes удалены
- [x] CORS настроен
- [x] WebSocket events endpoint
- [ ] API v1 завершение (миграция handlers)
- [ ] libp2p P2P интеграция
- [ ] Integration тесты
- [ ] OpenAPI спецификация

### Phase 2 (следующий этап)
- Neira Language: IR + интерпретатор
- Desktop App (Tauri)
- Mobile App (React Native)
- P2P collaboration (мультиагентные системы)

**Подробнее**: см. `docs/roadmap.md`

---

## 🤝 Вклад в проект

1. Fork репозитория
2. Создай feature branch (`git checkout -b feature/amazing-feature`)
3. Commit изменения (`git commit -m 'feat: добавлена amazing-feature'`)
4. Push в branch (`git push origin feature/amazing-feature`)
5. Открой Pull Request

**Правила**:
- Следуй [Conventional Commits](https://www.conventionalcommits.org/)
- Пиши тесты для новых фич
- Обновляй документацию при изменении API
- Запускай `cargo fmt` и `cargo clippy` перед PR

---

## 📜 Лицензия

MIT License — см. [LICENSE](LICENSE)

---

## 📞 Контакты

- **Репозиторий**: [github.com/redvampir/Neira](https://github.com/redvampir/Neira)
- **Issues**: [github.com/redvampir/Neira/issues](https://github.com/redvampir/Neira/issues)
- **Документация**: [docs/](docs/)

---

**Помни**: Neira — это не просто программа, это **живая система**. Относись к ней как к партнёру, а не инструменту. 🌱

🚀 **Удачи в исследовании!**
