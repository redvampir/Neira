---
neira:meta:
  id: "NEI-20251101-codex-pr-checklist"
  intent: "review_checklist"
  summary: "Чеклист для ревью PR от Codex: neira:meta, тесты, метрики, безопасность."
---

PR Review Checklist — кратко
- [ ] neira:meta: добавлен/обновлён для каждого заметного файла (id, intent, summary).
- [ ] Tests: unit/integration tests добавлены и локально проходят.
- [ ] Lint/format: проектные линтеры/форматтеры применены (eslint/prettier/black/rustfmt).
- [ ] No secrets: в коде/fixtures нет хардкодных секретов/паролей.
- [ ] DigestivePipeline: если изменяет входы — проверено соответствие схемы (spinal_cord/config/digestive.toml `schema_path`).
- [ ] store_parsed_input: при изменении парсинга — есть вызов/смок (MemoryCell) и тест на сохранение.
- [ ] Metrics: новые метрики задокументированы и отправляются в существующую подсистему (идеально — мок/тест).
- [ ] Feature gates: изменения влияющие на training/feature gates отмечены и по умолчанию locked/require owner activation.
- [ ] Escalation: операции schema change / data migration требуют request_confirmation (см. AGENTS.md).
- [ ] Resource limits: таймауты/лимиты берутся из конфига или заданы разумные дефолты (не хардкод).
- [ ] Filesize/scope: изменения небольшие и обратимы; большие изменения — разбить или получить согласование.
- [ ] README/docs: обновлены краткие инструкции по запуску/проверке изменений.
- [ ] CI: green (или объяснение временной перегрузки), тесты в CI покрывают новые кейсы.
- [ ] Changelog/Commit: понятные сообщения по Conventional Commits.
- [ ] Owner approval: если операция рискованная — получить owner confirmation / использовать owner activation phrase.

Короткий процесс:
1) Проверить чеклист. 2) Прогнать тесты и линтеры. 3) Отметить review-ready и дождаться owner для risky items.

