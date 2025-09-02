# CI/CD и деплой клеток

Документ описывает типичный конвейер непрерывной интеграции и доставки для Neira.

## Этапы конвейера

1. **Сборка**
   - `npm run build` — компиляция TypeScript-клеток.
   - `cargo build --release` — сборка Rust-компонентов.
2. **Тесты**
   - `npm test` — запуск тестов на TypeScript.
   - `cargo test` — запуск тестов на Rust.
3. **Упаковка**
   - `npm pack` — формирование npm-пакета.
   - `cargo package` — подготовка crate к публикации.
4. **Выкладка**
   - `scp target/release/neira user@server:/opt/neira` — копирование бинарника на сервер.
   - `npm publish` — публикация JS-пакета при необходимости.

## Пример GitHub Actions

```yaml
name: CI
on:
  push:
    branches: [main]

jobs:
  build-test-deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-cell@v4
        with:
          cell-version: 20
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          override: true
      - run: npm ci
      - run: npm run build
      - run: npm test
      - run: cargo test
      - run: cargo build --release
      - name: Package
        run: |
          npm pack
          cargo package
      - name: Deploy
        env:
          SSH_PRIVATE_KEY: ${{ secrets.SSH_PRIVATE_KEY }}
        run: scp target/release/neira user@server:/opt/neira
```
