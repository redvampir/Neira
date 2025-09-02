<!-- neira:meta
id: NEI-20250831-factory-shim-guide
intent: docs
summary: Внешний оркестратор (Shim) для фабрики: CLI, LLM-агент, безопасные команды dry-run/create/approve без прямой связи с ядром Нейры.
-->
<!-- neira:meta
id: NEI-20251115-organ-cancel-build-guide
intent: docs
summary: добавлена ссылка на DELETE /organs/:id/build как зарезервированный маршрут.
-->
<!-- neira:meta
id: NEI-20250305-factory-shim-runtime-term
intent: docs
summary: Термин Node.js заменён на Cell runtime.
-->
<!-- neira:meta
id: NEI-20260413-factory-shim-rename
intent: docs
summary: Обновлены упоминания backend на spinal_cord.
-->

# Factory Shim (External Orchestrator)

Purpose

- Внешнее приложение-«заглушка» для управления фабрикой через HTTP API (draft).
- Не связано напрямую с ядром Нейры; запускается отдельно, использует токен и базовый URL.
- Поддерживает два режима: прямые CLI-команды и LLM-агент (локальная модель) с ограниченными «инструментами».

Endpoints (реализовано сейчас)

- POST `/factory/cells/dryrun` → отчёт совместимости/рисков.
- POST `/factory/cells` → создать клетка (state=draft). Gate: `factory_adapter`.
- POST `/factory/cells/:id/approve|disable/rollback` → продвижение/отключение/откат.

Черновик (в доках, но ещё не реализовано в spinal_cord)

- POST `/organs/build`, GET `/organs/:id/status`, DELETE `/organs/:id/build` — зарезервированы для Organ Builder v0.

Safety & Policies

- Любые write-операции требуют токена `write`/`admin` (см. docs/api/backend.md). В Safe Mode — только `admin`.
- Ошибки политики возвращаются в JSON: `{ code, reason, capability? }`.
- В Shim по умолчанию запрещены `approve/disable/rollback` без флага `--yes` или интерактивного подтверждения.

Installation

- Ничего ставить не нужно: используется Cell runtime (Node.js >=18) с встроенным `fetch`.
- Файлы CLI: `scripts/factory-shim/index.mjs`.

Environment

- `FACTORY_BASE_URL` — базовый URL API (по умолчанию spinal_cord слушает `http://127.0.0.1:3000`; можно сменить через `NEIRA_BIND_ADDR`).
- `FACTORY_TOKEN` — админ/райт токен: `Authorization: Bearer <token>`.
- Пример смены порта spinal_cord: `$env:NEIRA_BIND_ADDR='0.0.0.0:4000'`
- LLM (опционально, для `agent`):
  - `LLM_PROVIDER=ollama|openai` — провайдер (локальный Ollama или OpenAI-совместимый).
  - `LLM_BASE_URL` — базовый URL чата (`http://localhost:11434` для Ollama).
  - `LLM_MODEL` — модель (например, `llama3`, `qwen2:7b` и т. п.).

Usage (CLI)

- Dry-run клетки:
  - `cell scripts/factory-shim/index.mjs dryrun-cell --spec examples/factory/voice-v1/analysis.text_normalize.v1.json`
- Создать клетка (draft):
  - `cell scripts/factory-shim/index.mjs create-cell --spec examples/factory/voice-v1/analysis.text_normalize.v1.json`
- Аппрув (draft→canary или canary→experimental):
  - `cell scripts/factory-shim/index.mjs approve-cell --id <cell_id> --yes`
- Орган (draft): не поддерживается бэкендом на текущем этапе; используйте клетки.

Agent Mode (LLM-in-the-loop)

- Позволяет давать задания естественным языком, LLM выбирает инструменты (`dryrun_cell`, `create_cell`, `approve_cell`, `organ_build`, `organ_status`).
- Старт:
  - `cell scripts/factory-shim/index.mjs agent --goal "Создай голосовой орган v1 (normalize→phonemes→speak_adapter) и сделай dry-run"`
- Безопасность: для действий `approve|disable|rollback` Shim потребует `--yes` или интерактивное подтверждение.

Examples

- Шаблоны для "Голос v1" (CellTemplate, схема v1): `examples/factory/voice-v1/*`.
- Клетки: `analysis.text_normalize.v1`, `analysis.text_to_phonemes.v1`, `analysis.speak_adapter.v1`.
- Орган: `organ.voice.v1` — концепт для будущего Organ Builder (не вызывается CLI).

Notes

- Exec spinal_cord'ы (Script/WASM) остаются закрытыми: Shim работает в режиме Adapter-only.
- Для стабильности развития используйте CAPABILITIES и Policy Engine; включение/выключение под флагами и с журналом.
- Тело запросов использует «плоский» формат CellTemplate (без обёртки `tpl`), как в `FactoryBody` (flatten).

Handover (RU)

- Что: добавлен внешний CLI-оркестратор фабрики с LLM-режимом и примерами Voice v1.
- Зачем: безопасно управлять ростом без прямой интеграции ассистента в ядро.
- Проверка: установить `FACTORY_BASE_URL`/`FACTORY_TOKEN`, затем выполнить примеры выше; видеть ответы JSON и изменения в интроспекции/Admin UI.
