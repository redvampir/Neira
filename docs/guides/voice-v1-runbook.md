<!-- neira:meta
id: NEI-20250317-120400-voice-v1-runbook-cell-template
intent: docs
summary: Пошаговый запуск Voice v1 через Factory Adapter, обновлена ссылка на схему cell-template.
-->

<!-- neira:meta
id: NEI-20250310-cell-templates-env-doc
intent: docs
summary: Обновлена переменная окружения на CELL_TEMPLATES_DIR с поддержкой NODE_TEMPLATES_DIR.
-->
<!-- neira:meta
id: NEI-20260413-voice-runbook-rename
intent: docs
summary: Заменены упоминания backend на spinal_cord.
-->

# Voice v1 — Runbook (Adapter-only)

Goal

- Зарегистрировать 3 шаблона клеток (CellTemplate) для конвейера Voice v1:
  - `analysis.text_normalize.v1`
- `analysis.text_to_phonemes.v1`
  - `analysis.speak_adapter.v1`
  - `action.speak_adapter.v1` (ActionCellTemplate)
- Прогнать dry-run, создать записи (draft), при необходимости продвинуть до canary.
- Убедиться в автоподхвате шаблонов при перезапуске (CellRegistry watcher).

Prerequisites

- Backend собран и запущен с включённым адаптером фабрики:
  - PowerShell: `$env:FACTORY_ADAPTER_ENABLED='1'`
  - (опц.) каталог шаблонов: `$env:CELL_TEMPLATES_DIR='templates'` (подкаталоги поддерживаются; fallback `NODE_TEMPLATES_DIR`)
  - Токен (минимум write) для API: `$env:FACTORY_TOKEN='secret'` (см. spinal_cord init)
  - Базовый URL: `$env:FACTORY_BASE_URL='http://localhost:3000'` (порт можно сменить через `NEIRA_BIND_ADDR`)
- Пример смены порта: `$env:NEIRA_BIND_ADDR='0.0.0.0:4000'`
- Проверьте свободность порта: `lsof -i :3000` (иначе задайте другой через `NEIRA_BIND_ADDR`)

Files

- Примеры шаблонов: `examples/factory/voice-v1/*.json` (соответствуют `schemas/v1/cell-template.schema.json`).

Steps (PowerShell)

1. Dry‑run каждого клетки
   cell scripts/factory-shim/index.mjs dryrun-cell --spec examples/factory/voice-v1/analysis.text_normalize.v1.json
   cell scripts/factory-shim/index.mjs dryrun-cell --spec examples/factory/voice-v1/analysis.text_to_phonemes.v1.json
   cell scripts/factory-shim/index.mjs dryrun-cell --spec examples/factory/voice-v1/analysis.speak_adapter.v1.json
   cell scripts/factory-shim/index.mjs dryrun-cell --spec examples/factory/voice-v1/action.speak_adapter.v1.json
2. Создать записи (draft) и сохранить в `templates/`
   cell scripts/factory-shim/index.mjs create-cell --spec examples/factory/voice-v1/analysis.text_normalize.v1.json
   cell scripts/factory-shim/index.mjs create-cell --spec examples/factory/voice-v1/analysis.text_to_phonemes.v1.json
   cell scripts/factory-shim/index.mjs create-cell --spec examples/factory/voice-v1/analysis.speak_adapter.v1.json
   cell scripts/factory-shim/index.mjs create-cell --spec examples/factory/voice-v1/action.speak_adapter.v1.json
3. (Опционально) Аппрув до canary
   cell scripts/factory-shim/index.mjs approve-cell --id adapter:analysis.text_normalize.v1 --yes
   cell scripts/factory-shim/index.mjs approve-cell --id adapter:analysis.text_to_phonemes.v1 --yes
   cell scripts/factory-shim/index.mjs approve-cell --id adapter:analysis.speak_adapter.v1 --yes
   cell scripts/factory-shim/index.mjs approve-cell --id adapter:action.speak_adapter.v1 --yes
4. Проверка автоподхвата

- Убедитесь, что в каталоге `templates/` появились файлы `analysis.*.json`.
- Перезапустите spinal_cord — CellRegistry загрузит шаблоны из каталога (есть файловый watcher).
- Проверьте `/cells/:id` (или Admin UI) и `logs/factory_audit.ndjson`.

HTTP API голосового органа
- `POST /voice/speak` — вход: `{ "text": "Привет" }` или `{ "normalized": "привет" }`. Ответ содержит `request_id`, путь к WAV и base64 аудио.
- `POST /voice/transcribe` — вход: `{ "audio_base64": "..." }` или `{ "file_path": "voice_output/voice-1.wav" }`. Ответ: `request_id`, текст.
- Переменные окружения: `VOICE_BACKEND`, `VOICE_TTS_CMD`, `VOICE_STT_CMD`, `VOICE_PLAY_CMD`, `VOICE_OUTPUT_DIR`.
  Notes
- Organ Builder (маршруты `/organs/*`) пока не реализован — работаем на уровне клеток.
- Политики/гейты: ошибки приходят JSON `{ code, reason, capability }`; включение адаптера — `FACTORY_ADAPTER_ENABLED=1`.
- Метрики: `factory_*` доступны на `/metrics` при включённой NERVOUS_SYSTEM.
  Handover (RU)
- Что: минимальный процесс Voice v1 через Factory Adapter, без исполняемого TTS.
- Почему: позволяет нормализовать цикл роста и проверить автоподхват.
- Проверка: команды выше должны вернуть `ok:true` и появление файлов в `templates/`.
