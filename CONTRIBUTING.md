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

## Тестирование

Перед отправкой изменений выполните:

```
npm test
cargo test
```
