# Руководство по внесению вкладов

## Требования

- Node.js 20 LTS (см. `.nvmrc`)
- Rust 1.75+ (см. `rust-toolchain`)

## Установка сред разработки

Установите Node.js через [nvm](https://github.com/nvm-sh/nvm):

```
nvm install
```

Rust устанавливается через [rustup](https://rustup.rs/). Версия задается файлом `rust-toolchain`:

```
rustup toolchain install 1.75
```

Убедитесь, что ваш редактор поддерживает настройки из `.editorconfig`.

## Установка зависимостей и хуков

После установки Node.js и Rust установите npm зависимости:

```
npm install
```

Команда настроит pre-commit хуки Husky и lint-staged.

## Тестирование

Перед отправкой изменений убедитесь, что проходят следующие проверки:

```
npm run lint
npm test
cargo clippy
cargo test
```

Файлы в коммите автоматически форматируются с помощью lint-staged.

