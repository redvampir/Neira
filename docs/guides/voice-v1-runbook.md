<!-- neira:meta
id: NEI-20250831-voice-v1-runbook
intent: docs
summary: Пошаговый запуск Voice v1 через Factory Adapter: dry-run → create → approve, автоподхват шаблонов, проверки.
-->

# Voice v1 — Runbook (Adapter‑only)

Goal
- Зарегистрировать 3 шаблона узлов (NodeTemplate) для конвейера Voice v1:
  - `analysis.text_normalize.v1`
  - `analysis.text_to_phonemes.v1`
  - `analysis.speak_adapter.v1`
- Прогнать dry‑run, создать записи (draft), при необходимости продвинуть до canary.
- Убедиться в автоподхвате шаблонов при перезапуске (NodeRegistry watcher).

Prerequisites
- Backend собран и запущен с включённым адаптером фабрики:
  - PowerShell: `$env:FACTORY_ADAPTER_ENABLED='1'`
  - (опц.) каталог шаблонов: `$env:NODE_TEMPLATES_DIR='templates'`
- Токен (минимум write) для API: `$env:FACTORY_TOKEN='secret'` (см. backend init)
- Базовый URL: `$env:FACTORY_BASE_URL='http://localhost:8080'`

Files
- Примеры шаблонов: `examples/factory/voice-v1/*.json` (соответствуют `schemas/v1/node-template.schema.json`).

Steps (PowerShell)
1) Dry‑run каждого узла
```
node scripts/factory-shim/index.mjs dryrun-node --spec examples/factory/voice-v1/analysis.text_normalize.v1.json
node scripts/factory-shim/index.mjs dryrun-node --spec examples/factory/voice-v1/analysis.text_to_phonemes.v1.json
node scripts/factory-shim/index.mjs dryrun-node --spec examples/factory/voice-v1/analysis.speak_adapter.v1.json
```

2) Создать записи (draft) и сохранить в `templates/`
```
node scripts/factory-shim/index.mjs create-node --spec examples/factory/voice-v1/analysis.text_normalize.v1.json
node scripts/factory-shim/index.mjs create-node --spec examples/factory/voice-v1/analysis.text_to_phonemes.v1.json
node scripts/factory-shim/index.mjs create-node --spec examples/factory/voice-v1/analysis.speak_adapter.v1.json
```

3) (Опционально) Аппрув до canary
```
node scripts/factory-shim/index.mjs approve-node --id adapter:analysis.text_normalize.v1 --yes
node scripts/factory-shim/index.mjs approve-node --id adapter:analysis.text_to_phonemes.v1 --yes
node scripts/factory-shim/index.mjs approve-node --id adapter:analysis.speak_adapter.v1 --yes
```

4) Проверка автоподхвата
- Убедитесь, что в каталоге `templates/` появились файлы `analysis.*.json`.
- Перезапустите backend — NodeRegistry загрузит шаблоны из каталога (есть файловый watcher).
- Проверьте `/nodes/:id` (или Admin UI) и `logs/factory_audit.ndjson`.

Notes
- Organ Builder (маршруты `/organs/*`) пока не реализован — работаем на уровне узлов.
- Политики/гейты: ошибки приходят JSON `{ code, reason, capability }`; включение адаптера — `FACTORY_ADAPTER_ENABLED=1`.
- Метрики: `factory_*` доступны на `/metrics` при включённой NERVOUS_SYSTEM.

Handover (RU)
- Что: минимальный процесс Voice v1 через Factory Adapter, без исполняемого TTS.
- Почему: позволяет нормализовать цикл роста и проверить автоподхват.
- Проверка: команды выше должны вернуть `ok:true` и появление файлов в `templates/`.

