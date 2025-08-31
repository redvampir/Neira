<!-- neira:meta
id: NEI-20250831-voice-v1-runbook
intent: docs
summary: Пошаговый запуск Voice v1 через Factory Adapter: dry-run → create → approve, автоподхват шаблонов, проверки.
-->

# Voice v1 — Runbook (Adapter-only)

Goal

- Зарегистрировать 3 шаблона узлов (NodeTemplate) для конвейера Voice v1:
  - `analysis.text_normalize.v1`
  - `analysis.text_to_phonemes.v1`
  - `analysis.speak_adapter.v1`
- Прогнать dry-run, создать записи (draft), при необходимости продвинуть до canary.
- Убедиться в автоподхвате шаблонов при перезапуске (NodeRegistry watcher).

Prerequisites

- Backend собран и запущен с включённым адаптером фабрики:
  - PowerShell: `$env:FACTORY_ADAPTER_ENABLED='1'`
  - (опц.) каталог шаблонов: `$env:NODE_TEMPLATES_DIR='templates'`
- Токен (минимум write) для API: `$env:FACTORY_TOKEN='secret'` (см. backend init)
- Базовый URL: `$env:FACTORY_BASE_URL='http://localhost:3000'` (порт можно сменить через `NEIRA_BIND_ADDR`)
- Пример смены порта: `$env:NEIRA_BIND_ADDR='0.0.0.0:4000'`
- Проверьте свободность порта: `lsof -i :3000` (иначе задайте другой через `NEIRA_BIND_ADDR`)

Files

- Примеры шаблонов: `examples/factory/voice-v1/*.json` (соответствуют `schemas/v1/node-template.schema.json`).

Steps (PowerShell)

1. Dry-run каждого узла


