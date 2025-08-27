# Тестирование

Каталог `tests/` содержит файлы для всех модулей MVP, перечисленных в `docs/mvp-modules.md`. Файлы с суффиксом `_test.rs` хранят примеры модульных тестов на Rust, а файлы `*.test.ts` — тесты на TypeScript.

Базовые сценарии для узлов Action, Analysis, Memory и Support реализованы в соответствующих Rust-тестах.

## Запуск тестов

Для запуска Rust-тестов:

```bash
cargo test
```

Для запуска тестов на TypeScript:

```bash
npm test
```
