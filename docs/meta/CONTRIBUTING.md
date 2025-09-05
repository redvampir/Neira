# Руководство по внесению вкладов

<!-- neira:meta
id: NEI-20250904-121100-contrib-cell-runtime
intent: docs
summary: Уточнено, что используется Cell runtime (Node.js 20 LTS).
-->

<!-- neira:meta
id: NEI-20270330-contrib-workspace-install
intent: docs
summary: Добавлена установка зависимостей через npm/pnpm workspace.
-->

## Требования

- Cell runtime: Node.js 20 LTS (см. `.nvmrc`)
- Rust 1.75+ (см. `rust-toolchain`)

## Установка сред разработки

Установите Cell runtime (Node.js) через [nvm](https://github.com/nvm-sh/nvm):

```
nvm install
```

Rust устанавливается через [rustup](https://rustup.rs/). Версия задается файлом `rust-toolchain`:

```
rustup toolchain install 1.75
```

Убедитесь, что ваш редактор поддерживает настройки из `.editorconfig`.

## Установка зависимостей и хуков

После установки runtime и Rust установите зависимости workspace:

```
npm install # или pnpm install
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
