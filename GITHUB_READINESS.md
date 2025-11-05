# GitHub Pre-Publish Checklist

**Дата**: 2025-11-05  
**Цель**: Проверка готовности Neira к публикации на GitHub

---

## ✅ Безопасность

### Секреты и ключи
- [x] `.env` добавлен в `.gitignore`
- [x] Создан `.env.example` с примерами (без реальных значений)
- [x] Проверено отсутствие секретов в коде (grep поиск)
- [x] Нет хардкоженных токенов/паролей
- [ ] **TODO**: Провести финальный поиск секретов перед push

### Файлы и директории
- [x] `.gitignore` обновлён (секреты, IDE, OS, зависимости)
- [x] `target/` игнорируется (артефакты сборки)
- [x] `node_modules/` игнорируется
- [x] Исходники компиляторов исключены (binutils, llvm, cargo, rust, nasm)

---

## 📝 Документация

### Основные файлы
- [x] `README.md` обновлён (версия 2.0, компактный)
- [x] `PROJECT_HANDOVER.md` создан (полное руководство)
- [x] `AGENTS.md` переработан (версия 2.0)
- [x] `LICENSE` существует (MIT)
- [x] `CHANGELOG.md` сгенерирован (824 записи)

### API документация
- [x] `docs/guides/websocket-events-guide.md` создан
- [x] `docs/guides/quick-start.md` обновлён
- [ ] **TODO**: Создать `docs/api/endpoints-v1.md`
- [ ] **TODO**: Создать `docs/api/openapi.yaml`

### Инструкции
- [x] `docs/DOCUMENTATION_GUIDE.md` создан
- [x] `docs/RESTRUCTURING_SUMMARY.md` создан
- [x] Устаревшие документы перемещены в `docs/archive/`

---

## 🔧 Код

### Компиляция
- [ ] **В процессе**: `cargo check` (проверяется сейчас)
- [ ] **TODO**: `cargo test --all` (запустить все тесты)
- [ ] **TODO**: `cargo clippy -- -D warnings` (линтер)
- [ ] **TODO**: `cargo fmt -- --check` (форматирование)

### API versioning (Task 4)
- [x] `ApiResponse<T>` struct создан
- [x] Helper функции `api_success`, `api_error`
- [x] Мигрированы handlers:
  - [x] `register_cell`
  - [x] `get_cell`
  - [x] `get_cell_latest`
  - [x] `voice_speak`
  - [x] `voice_transcribe`
- [ ] **TODO**: Мигрировать остальные handlers

### Комментарии
- [x] Упрощены в `main.rs` (минимализм)
- [x] Формат: `// NEI-YYYYMMDD: краткое описание`
- [x] Инструменты созданы (extract_changelog, minimize_comments)

---

## 🗂️ Структура

### Директории
```
neira/
├── .github/          # TODO: Создать workflows (CI/CD)
├── spinal_cord/      # ✅ Backend (Rust)
├── sensory_organs/   # ✅ Сенсорные модули
├── docs/             # ✅ Документация
├── tools/            # ✅ Утилиты (Python скрипты)
├── tests/            # ✅ Тесты
├── schemas/          # ✅ JSON Schema
├── .gitignore        # ✅ Обновлён
├── .env.example      # ✅ Создан
├── README.md         # ✅ Обновлён
├── LICENSE           # ✅ MIT
└── CHANGELOG.md      # ✅ Сгенерирован
```

### Что добавить перед публикацией
- [ ] `.github/workflows/rust.yml` — CI для Rust (cargo test, clippy)
- [ ] `.github/workflows/docs.yml` — автогенерация CHANGELOG
- [ ] `CONTRIBUTING.md` — правила контрибуций
- [ ] `CODE_OF_CONDUCT.md` — кодекс поведения

---

## 🚀 GitHub Actions (TODO)

### CI/CD пайплайны
```yaml
# .github/workflows/rust.yml
name: Rust CI
on: [push, pull_request]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --release
      - run: cargo test --all
      - run: cargo clippy -- -D warnings
```

---

## 📊 Размер репозитория

### Оценка
- Основной код: ~500 MB (без компиляторов)
- Компиляторы (исключены): ~15 GB
- target/ (игнорируется): ~2 GB

### Оптимизация
- [x] Исходники компиляторов вынесены в отдельный репо
- [x] `target/` в `.gitignore`
- [ ] **TODO**: Проверить, нет ли больших бинарников в истории Git

---

## ⚠️ Перед push на GitHub

### Критические проверки
```bash
# 1. Поиск секретов
rg -i "(password|secret|api_key|token|bearer).*=.*['\"]" --type rust

# 2. Проверка .env не коммитится
git status | grep -i ".env"

# 3. Размер репо
git count-objects -vH

# 4. История коммитов (нет больших файлов?)
git rev-list --objects --all | \
  git cat-file --batch-check='%(objecttype) %(objectname) %(objectsize) %(rest)' | \
  awk '/^blob/ {print substr($0,6)}' | \
  sort --numeric-sort --key=2 | \
  tail -20

# 5. Финальная компиляция
cargo build --release
cargo test --all
cargo clippy -- -D warnings
```

---

## 🎯 Рекомендации

### Что сделать ПЕРЕД публикацией
1. ✅ Завершить Task 4 (API versioning) — **в процессе**
2. 🔄 Запустить все тесты (`cargo test --all`)
3. 🔄 Запустить линтер (`cargo clippy`)
4. 🔄 Проверить форматирование (`cargo fmt -- --check`)
5. ❌ Создать `CONTRIBUTING.md`
6. ❌ Создать `.github/workflows/` (CI/CD)
7. ❌ Финальный поиск секретов

### Что можно сделать ПОСЛЕ публикации
- Создать `docs/api/endpoints-v1.md`
- Создать `docs/api/openapi.yaml`
- Настроить GitHub Actions для автоматической генерации CHANGELOG
- Добавить badges в README (build status, coverage)
- Создать GitHub Wiki

---

## ✅ Финальный чеклист

- [x] `.gitignore` обновлён
- [x] `.env.example` создан
- [x] Документация готова (README, PROJECT_HANDOVER, AGENTS)
- [x] Комментарии упрощены
- [x] CHANGELOG сгенерирован
- [ ] Компиляция успешна (`cargo check`)
- [ ] Тесты пройдены (`cargo test`)
- [ ] Линтер пройден (`cargo clippy`)
- [ ] Форматирование проверено (`cargo fmt`)
- [ ] Поиск секретов выполнен (финальный)
- [ ] API versioning завершён (Task 4)

---

**Статус**: 🟡 Почти готово (осталось: тесты, линтер, финализация Task 4)

**Следующие шаги**:
1. Дождаться завершения `cargo check`
2. Завершить миграцию handlers (Task 4)
3. Запустить `cargo test --all`
4. Запустить `cargo clippy`
5. Финальный поиск секретов
6. Готово к push! 🚀
