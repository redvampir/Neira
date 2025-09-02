<!-- neira:meta
id: NEI-20250923-factory-api-draft
intent: docs
summary: Черновой API Фабрикаторов (dry‑run/approve/rollback) и сборки органов.
-->
<!-- neira:meta
id: NEI-20251010-organ-builder-status-route
intent: docs
summary: описан ручной апдейт статуса органа и метрика длительности сборки.
-->
<!-- neira:meta
id: NEI-20260413-factory-rename
intent: docs
summary: Обновлён пример запуска sensory_organs вместо frontend.
-->
<!-- neira:meta
id: NEI-20251101-organ-builder-stage-delays-doc
intent: docs
summary: добавлен пример настройки ORGANS_BUILDER_STAGE_DELAYS.
-->
<!-- neira:meta
id: NEI-20250620-organ-builder-stage-delays-doc-rename
intent: docs
summary: пример обновлён под ORGANS_BUILDER_STAGE_DELAYS.
-->
<!-- neira:meta
id: NEI-20260501-organ-stream-doc
intent: docs
summary: описан WS /organs/:id/stream с примером подключения.
-->
<!-- neira:meta
id: NEI-20251115-organ-cancel-build-doc
intent: docs
summary: описан DELETE /organs/:id/build для отмены сборки.
-->
<!-- neira:meta
id: NEI-20250207-factory-sample-templates-doc
intent: docs
summary: добавлен раздел Sample Templates с примерами органных шаблонов.
-->
<!-- neira:meta
id: NEI-20250219-organs-panel-doc
intent: docs
summary: добавлен раздел о запуске панели органов.
-->
<!-- neira:meta
id: NEI-20250210-factory-template-schema-doc
intent: docs
summary: описана структура шаблона клетки.
-->
<!-- neira:meta
id: NEI-20240517-120003-factory-api-autoresponse
intent: docs
summary: |
  Добавлены события и API auto_heal/auto_rollback.
-->

# Factory API (Draft)

All routes require admin token unless noted. Exec routes are gated via CAPABILITIES.

- POST `/factory/cells/dryrun`
  - Body: { spec | template, mode?: 'adapter' }
  - Resp: { report: { deps, links, risks }, ok: bool }

- POST `/factory/cells`
  - Gate: `factory_adapter=experimental`
  - Body: { spec | template, hitl?: true }
  - Resp: { id, state: 'draft' }

- POST `/factory/cells/:id/approve`
  - Moves: draft→canary or canary→experimental
  - Resp: { id, state }

- POST `/factory/cells/:id/disable|rollback`
  - Resp: { id, state: 'disabled'|'rolled_back' }

- POST `/factory/cells/:id/auto_heal`
  - Trigger: Immune System при сбое
  - Resp: { id, state: 'healing'|'failed' }

- POST `/factory/cells/:id/auto_rollback`
  - Trigger: Immune System при критическом сбое
  - Resp: { id, state: 'rolled_back' }

## Events

- `factory.auto_heal` — запуск автоматического восстановления клетки.
- `factory.auto_rollback` — автооткат к последнему стабильному состоянию.

Adapter Contracts (обязательные хуки)

- Registry: регистрация CellTemplate в файловом каталоге (`/cells` API) + индексация в реестре.
- Hub/NS/IS: автопубликация метрик (`factory_*`), статусы в интроспекции, проверки safe‑mode/политик.
- Состояния: Draft → Canary → Experimental → Stable → (Disabled/RolledBack) — коды выдаются в API.
- Ошибки: унифицированный формат { code, reason, capability? } (Policy Engine); при выключенном гейте — `capability_disabled`.

- POST `/organs/build`
  - Gate: `organs_builder=experimental`
  - Body: { organ_template, dryrun?: true }
  - Resp: { organ_id, state: 'draft'|'canary'|'experimental'|'stable' }
  - Logs `organ build started` и метрики `organ_build_attempts_total`, `organ_build_duration_ms`
  - Задержки стадий берутся из `ORGANS_BUILDER_STAGE_DELAYS` (пример: `50,100,200` → canary/experimental/stable; устаревший алиас: `ORGANS_BUILDER_STAGE_DELAYS_MS`)

- GET `/organs/:id/status`
  - Resp: { id, state, cells, metrics }
  - Метрика: `organ_build_status_queries_total`

- POST `/organs/:id/status`
  - Body: { state: 'draft'|'canary'|'experimental'|'stable'|'failed' }
  - Resp: { id, state }
  - Позволяет вручную продвигать орган по стадиям

- WS `/organs/:id/stream`
  - При каждом изменении стадии отправляет `{ id, state }`
  - Пример:
    ```js
    const ws = new WebSocket('ws://localhost:3000/organs/organ-1/stream');
    ws.onmessage = ev => console.log(ev.data);
    ```

- DELETE `/organs/:id/build`
  - Останавливает сборку органа и переводит его в `failed`

## Template Schema

```json
{
  "id": "string",
  "version": "semver",
  "analysis_type": "string",
  "links": ["string"],
  "metadata": { "schema": "v1" }
}
```

- `id` — уникальный идентификатор клетки
- `version` — версия шаблона в формате SemVer
- `analysis_type` — тип анализа/действия клетки
- `links` — список зависимостей
- `metadata.schema` — версия используемой JSON‑схемы

## Examples

Request (dry‑run, adapter):

```
POST /factory/cells/dryrun
{
  "backend": "adapter",
  "tpl": {
    "id": "analysis.summarize.v1",
    "version": "0.1.0",
    "analysis_type": "summary",
    "links": [],
    "metadata": { "schema": "v1" }
  }
}
```

Response:

```
{
  "ok": true,
  "report": { "analysis_type": "summary", "links": [], "risks": [] }
}
```

Stage delay config:

```
# устаревший алиас: ORGANS_BUILDER_STAGE_DELAYS_MS
ORGANS_BUILDER_STAGE_DELAYS=50,100,200
```

## Sample Templates

- [examples/organs/organ.echo.v1.json](../../examples/organs/organ.echo.v1.json) — минимальный эхо‑орган.
- [examples/organs/organ.reverse.v1.json](../../examples/organs/organ.reverse.v1.json) — орган, разворачивающий текст.

## Organs Panel

1. `npm run sensory_organs:dev`
2. Открыть `http://localhost:5173/src/organs.html`.

Notes

- Exec backends (script/wasm) остаются locked. Сначала adapter‑only.
