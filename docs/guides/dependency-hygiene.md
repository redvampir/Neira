<!-- neira:meta
id: NEI-20250904-dependency-hygiene-guide
intent: docs
summary: Руководство по гигиене зависимостей: cargo tree -d, cargo-deny, скрипты дублей и CI job Dependency Hygiene.
-->

# Гигиена зависимостей

Цели: не допускать множественных версий (дублей) crate'ов и контролировать рост дерева зависимостей.

Инструменты
- cargo tree: `cargo tree -d --target all` — показывает дубли версий.
- cargo-deny: `cargo deny check bans` — запрещает дубли по политике из `deny.toml`.
- Скрипты дублей:
  - Linux/macOS: `scripts/check-duplicates.sh`
  - Windows/PowerShell: `scripts/check-duplicates.ps1`

CI
- Шаг “Cargo deny (bans)” в `.github/workflows/ci.yml` запускается на каждом PR.
- Отдельный job “Dependency Hygiene” на Linux и Windows:
  - Печатает `cargo tree -d` (для наглядности в логах).
  - Запускает соответствующий скрипт дублей.
  - Запускает `cargo deny check bans`.

Особые исключения
- Семейства Windows/WASI временно в allowlist (см. `deny.toml` и скрипты), так как экосистема ещё не унифицирована; цель — снять исключения при обновлениях апстрима.

Ручная чистка (лёгкая)
1) Обновить верхнеуровневые зависимости в пределах minor/patch (см. Dependabot/PR).
2) Выполнить `cargo update`.
3) Проверить:
   - `cargo tree -d --target all`
   - `scripts/check-duplicates.sh` или `scripts/check-duplicates.ps1`
   - `cargo deny check bans`
4) Если появились новые дубли (не Windows/WASI): поднять версии верхних пакетов или откатить PR.

