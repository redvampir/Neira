<!-- neira:meta
id: NEI-20250905-000000-docs-git-github-guide
intent: docs
summary: Подробная инструкция по Git/GitHub, меткам авто-ребейза и авто-фикса конфликтов, Merge Queue и git rerere. Разъяснение "Allow edits by maintainers".
-->
<!-- neira:meta
id: NEI-20250101-120000-docs-manifest-merge
intent: docs
summary: Указано, что авто-фиксер объединяет зависимости в Cargo.toml и package.json.
-->

# Git и GitHub — инструкция, чтобы не спотыкаться

Цель — сделать работу через ветки и PR предсказуемой и почти без конфликтов.

Коротко: ставь на PR две метки `auto-rebase` и `auto-fix-conflicts`. Для «жёсткого» предпочтения своей ветки добавь `prefer-branch-hard`.

## Быстрый старт (рекомендованный поток)
- Обнови `main` локально: `git fetch origin && git checkout main && git pull`.
- Создай ветку: `git checkout -b feat/моя-ветка`.
- Коммить изменения: `git add -A && git commit -m "feat: короткое описание"`.
- Запушь ветку: `git push -u origin feat/моя-ветка`.
- Открой PR в `main` и включи чекбокс “Allow edits by maintainers”.
- Поставь метки на PR: `auto-rebase` и `auto-fix-conflicts`.
- При необходимости «всегда моя сторона» — добавь `prefer-branch-hard`.
- Дальше работай по делу; бот будет поддерживать PR свежим и гасить тривиальные конфликты.

## Метки автоматики и что они делают
- `auto-rebase`: бот периодически ребейзит PR на актуальную базовую ветку. Уменьшает шанс «протухших» конфликтов.
- `auto-fix-conflicts`: бот вливает базовую ветку в PR, объединяет зависимости в `Cargo.toml` и `package.json` и пытается погасить простые конфликты.
- `prefer-branch-soft` (необязательно): мягкий режим (по умолчанию). Предпочтение твоей ветки (ours), но lockfiles (Cargo.lock, yarn.lock и т.п.) берутся из базы (theirs). Для `*.list` — объединение (union).
- `prefer-branch-hard`: жёсткий режим. Всегда предпочитается твоя ветка (ours), без специальных исключений.

Примечание: `prefer-branch-soft` можно не ставить — это дефолт.

## «В сторону новой ветки» по умолчанию
Наш авто‑фиксер настроен в пользу ветки PR. Это значит, что при конфликтах он выберет твою версию файлов, за редкими исключениями (lockfiles в мягком режиме). Если нужно «всё моё», ставь `prefer-branch-hard`.

## Если конфликт остался
- Значит конфликт смысловой (по содержанию). Открой его локально, реши руками, закоммить и запушь. Метки оставь — бот продолжит работать на следующих шагах.
- Быстрый шаблон «в сторону моей ветки, а lockfile из базы» локально:
  ```bash
  git fetch origin
  git checkout feat/моя-ветка
  git merge origin/main || true
  git checkout --ours -- .
  git checkout --theirs -- Cargo.lock **/Cargo.lock yarn.lock pnpm-lock.yaml package-lock.json 2>/dev/null || true
  git add -A && git commit -m "merge main (prefer ours; lockfiles theirs)" && git push
  ```

## PR из форков и «Allow edits by maintainers»
- Что это: флаг в PR, позволяющий мейнтейнерам базового репо (и GitHub Actions с токеном репо) пушить изменения прямо в ветку автора PR в форке.
- Зачем: наш авто‑фиксер и авто‑ребейзер пушат коммиты в ветку PR. Без этого флага пуш в форк будет запрещён, и автоматизация не сработает.
- Где включить: в окне создания PR — чекбокс “Allow edits by maintainers”. Для существующего PR — нажми «Edit» справа от заголовка PR и проставь чекбокс (если владелец форка разрешает).
- Если чекбокса нет: автор форка отключил разрешения или репо приватное/ограничения организации не позволяют. В этом случае автоматике нужен персональный токен автора PR или ручное вмешательство.

## Merge Queue (опционально)
Включается в настройках репозитория (репо Settings → General → Merge queue):
- Enable merge queue.
- Require branches to be up to date before merging (в Branch protection rules).
- Разреши Auto-merge для безопасных PR.

Эффект: PR попадают в очередь, GitHub сам обновляет их до `main` и запускает проверки до фактического merge. Конфликты ловятся раньше.

## Git rerere — запоминание решений конфликтов
- Что это: git rerere (reuse recorded resolution) сохраняет шаблоны разрешённых конфликтов и переиспользует их при повторном возникновении.
- Как у нас работает: workflow «PR Conflict Fixer» импортирует общий кэш из ветки `rerere-cache` перед слиянием, а после успешного авто‑разрешения экспортирует обновления обратно в `rerere-cache`.
- Выгода: повторяющиеся конфликты (например, «шапки» в Markdown) гасятся автоматически всё чаще.

## Типовые команды (локально) 
- Обновить `main` и свою ветку:
  ```bash
  git fetch origin
  git checkout main && git pull
  git checkout feat/моя-ветка
  git rebase main   # или: git merge main
  ```
- Предпочесть свою версию всех конфликтов:
  ```bash
  git checkout --ours -- .
  git add -A && git rebase --continue   # или git commit при merge
  ```
- Предпочесть базу (редко нужно):
  ```bash
  git checkout --theirs -- .
  git add -A && git rebase --continue
  ```
- Свести списки без дублей (union) вручную:
  ```bash
  sort -u input.list > input.list
  ```

## Создание нужных меток
Мы добавили workflow `Setup Labels`, который создаст/обновит метки:
- Открой Actions → Setup Labels → Run workflow.
- Будут созданы: `auto-rebase`, `auto-fix-conflicts`, `prefer-branch-soft`, `prefer-branch-hard`.

## FAQ
- Авто‑фиксер ничего не запушил — почему?
  - Скорее всего, PR из форка и не включён “Allow edits by maintainers”. Включи и перезапусти событие (перепоставь метку или сделай пустой коммит).
- Боюсь, что lockfile «сломает» сборку.
  - Используй мягкий режим (по умолчанию) — lockfiles берутся из базы. Жёсткий режим использовать осознанно.
- У меня конфликт в одном и том же месте снова и снова.
  - Это как раз для rerere. Дай авто‑фиксеру один раз успешно решить (или реши локально), после чего повтор будет гаситься автоматически.

