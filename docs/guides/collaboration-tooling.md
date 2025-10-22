<!-- neira:meta
id: NEI-20250904-120400-collab-tooling
intent: docs
summary: Описаны новые инструменты: проверка neira:meta и Conventional Commits, интеграция в pre-commit и CI.
-->

# Инструменты совместной работы

Что добавлено
- Проверка покрытия `neira:meta`: `scripts/check-meta.mjs`
  - Staged: `npm run meta:check:staged` (используется в pre-commit)
  - Сравнение с базой: `npm run meta:check` (используется в CI)
  - Валидация ключей: `id` (формат `NEI-YYYYMMDD-HHMMSS-<slug>`), `intent`, `summary`
- Проверка Conventional Commits: `scripts/check-commit-msg.mjs`
  - Подключено через `.husky/commit-msg`
- Шаблон PR с чек‑листом: `.github/PULL_REQUEST_TEMPLATE.md`
- Генератор id: `scripts/gen-neira-id.mjs` / `npm run meta:id`
 - Нормализация meta: `scripts/normalize-meta.mjs` / `npm run meta:normalize`

Как работает
- Pre-commit (`.husky/pre-commit`) теперь дополнительно валидирует наличие блока `neira:meta` в затронутых файлах
  (docs/, src/, spinal_cord/, sensory_organs/, scripts/). См. COMMENTING.md и META_COVERAGE.md.
- Commit-msg хук проверяет заголовок коммита на соответствие Conventional Commits.
- CI (`.github/workflows/ci.yml`) устанавливает зависимости и запускает проверку `neira:meta` относительно `origin/main`
  в строгом режиме (`--strict`), добавляет отчёт в Step Summary и комментарий в PR (если PR).

Подсказки
- Быстро сгенерируйте id: `NEI-YYYYMMDD-HHMMSS-<slug>` (UTC). Держите slug коротким и осмысленным.
- Для мелких правок достаточно упрощённого блока: id, intent, summary.

Генерация id
- `npm run meta:id` → печатает новый id в консоль
- `npm run meta:id -- <slug>` → с вашим slug: `NEI-YYYYMMDD-HHMMSS-<slug>`

Проверка локально
- Проверка только затронутых файлов: `node scripts/check-meta.mjs --staged`
- Проверка с базовой веткой: `node scripts/check-meta.mjs --since origin/main`
- Подробный отчёт: добавьте `--report summary` или `--out meta-report.json`

Нормализация meta
- Просмотр изменений (dry‑run): `npm run meta:normalize`
- Применение изменений: `npm run meta:normalize:write` (добавляет блоки и правит id/intent)

Строгая проверка YAML
- Локально (при установленных зависимостях): `npm run meta:check:strict`
- В pre‑commit используется быстрый режим без требуемого YAML: ошибки структуры ловятся в CI
