<!-- neira:meta
id: NEI-20250831-factory-shim-guide
intent: docs
summary: Внешний оркестратор (Shim) для фабрики: CLI, LLM-агент, безопасные команды dry-run/create/approve без прямой связи с ядром Нейры.
-->

# Factory Shim (External Orchestrator)

Purpose

- Внешнее приложение-«заглушка» для управления фабрикой через HTTP API (draft).
- Не связано напрямую с ядром Нейры; запускается отдельно, использует токен и базовый URL.
- Поддерживает два режима: прямые CLI-команды и LLM-агент (локальная модель) с ограниченными «инструментами».

Endpoints (реализовано сейчас)

- POST `/factory/nodes/dryrun` → отчёт совместимости/рисков.
- POST `/factory/nodes` → создать узел (state=draft). Gate: `factory_adapter`.
- POST `/factory/nodes/:id/approve|disable/rollback` → продвижение/отключение/откат.

Черновик (в доках, но ещё не реализовано в backend)

- POST `/organs/build`, GET `/organs/:id/status` — зарезервированы для Organ Builder v0.

Safety & Policies

- Любые write-операции требуют токена `write`/`admin` (см. docs/api/backend.md). В Safe Mode — только `admin`.
- Ошибки политики возвращаются в JSON: `{ code, reason, capability? }`.
- В Shim по умолчанию запрещены `approve/disable/rollback` без флага `--yes` или интерактивного подтверждения.

Installation

- Ничего ставить не нужно: используется Node.js (>=18) с встроенным `fetch`.
- Файлы CLI: `scripts/factory-shim/index.mjs`.

Environment

- `FACTORY_BASE_URL` — базовый URL API (по умолчанию backend слушает `http://127.0.0.1:3000`; можно сменить через `NEIRA_BIND_ADDR`).
- `FACTORY_TOKEN` — админ/райт токен: `Authorization: Bearer <token>`.
- Пример смены порта backend: `$env:NEIRA_BIND_ADDR='0.0.0.0:4000'`
- LLM (опционально, для `agent`):
  - `LLM_PROVIDER=ollama|openai` — провайдер (локальный Ollama или OpenAI-совместимый).
  - `LLM_BASE_URL` — базовый URL чата (`http://localhost:11434` для Ollama).
  - `LLM_MODEL` — модель (например, `llama3`, `qwen2:7b` и т. п.).

Usage (CLI)

- Dry-run узла:
  - `node scripts/factory-shim/index.mjs dryrun-node --spec examples/factory/voice-v1/analysis.text_normalize.v1.json`
- Создать узел (draft):
  - `node scripts/factory-shim/index.mjs create-node --spec examples/factory/voice-v1/analysis.text_normalize.v1.json`
- Аппрув (draft→canary или canary→experimental):
  - `node scripts/factory-shim/index.mjs approve-node --id <node_id> --yes`
- Орган (draft): не поддерживается бэкендом на текущем этапе; используйте узлы.

Agent Mode (LLM-in-the-loop)

- Позволяет давать задания естественным языком, LLM выбирает инструменты (`dryrun_node`, `create_node`, `approve_node`, `organ_build`, `organ_status`).
- Старт:
  - `node scripts/factory-shim/index.mjs agent --goal "Создай голосовой орган v1 (normalize→phonemes→speak_adapter) и сделай dry-run"`
- Безопасность: для действий `approve|disable|rollback` Shim потребует `--yes` или интерактивное подтверждение.

Examples

- Шаблоны для "Голос v1" (NodeTemplate, схема v1): `examples/factory/voice-v1/*`.
- Узлы: `analysis.text_normalize.v1`, `analysis.text_to_phonemes.v1`, `analysis.speak_adapter.v1`.
- Орган: `organ.voice.v1` — концепт для будущего Organ Builder (не вызывается CLI).

Notes

- Exec backend'ы (Script/WASM) остаются закрытыми: Shim работает в режиме Adapter-only.
- Для стабильности развития используйте CAPABILITIES и Policy Engine; включение/выключение под флагами и с журналом.
- Тело запросов использует «плоский» формат NodeTemplate (без обёртки `tpl`), как в `FactoryBody` (flatten).

Handover (RU)

- Что: добавлен внешний CLI-оркестратор фабрики с LLM-режимом и примерами Voice v1.
- Зачем: безопасно управлять ростом без прямой интеграции ассистента в ядро.
- Проверка: установить `FACTORY_BASE_URL`/`FACTORY_TOKEN`, затем выполнить примеры выше; видеть ответы JSON и изменения в интроспекции/Admin UI.
