# Neira Sandbox Agent — README

## Назначение
Безопасный агент для применения улучшений, предложенных Neira через Consciousness API.

## Принцип работы
1. **Мониторинг** — периодически проверяет `/api/neira/consciousness/stats` на наличие `improvement_tasks`
2. **Песочница** — создаёт изолированную копию репозитория через `git worktree`
3. **Применение** — интерпретирует `reasoning_steps` и применяет изменения
4. **Тестирование** — запускает lint, unit tests, smoke tests
5. **Pull Request** — если тесты прошли → создаёт PR, иначе откатывает
6. **Очистка** — удаляет временную песочницу

## Безопасность
- ✅ Изменения не применяются напрямую в main branch
- ✅ Все правки проходят проверку тестами
- ✅ Человек может ревью PR перед merge
- ✅ Откат автоматический при провале тестов
- ⚠️ Требует настройки GitHub CLI (`gh`) для создания PR

## Установка зависимостей
```bash
pip install requests
```

## Использование
```bash
# Однократный запуск
python tools/sandbox_agent.py f:\Neyra\neira

# Мониторинг (каждые 2 минуты)
python tools/sandbox_agent.py f:\Neyra\neira --watch --interval 120
```

## Пример workflow
1. Neira записывает мысль с `dialogue_id="improvement_task_*"`
2. Sandbox Agent видит новую задачу
3. Создаёт worktree в `../neira_sandbox_<pid>/`
4. Применяет изменения (например, добавляет ARIA метки)
5. Запускает `cargo clippy`, `node --check dashboard.js`
6. Если OK → `git push origin neira/improvement-123` + `gh pr create`
7. Удаляет worktree

## TODO
- [ ] Реализовать парсинг `reasoning_steps` → действия
- [ ] Добавить support для TypeScript/ESLint
- [ ] Интеграция с CI (GitHub Actions)
- [ ] Dashboard для мониторинга агента
- [ ] Rollback mechanism через `git revert`
- [ ] Notifications (Slack/Discord) при создании PR

## Связанные файлы
- [sandbox_agent.py](sandbox_agent.py) — основной код
- [AUTONOMOUS_TASK_2_FINAL.md](../AUTONOMOUS_TASK_2_FINAL.md) — контекст про автономность
