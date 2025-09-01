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

# Factory API (Draft)

All routes require admin token unless noted. Exec routes are gated via CAPABILITIES.

- POST `/factory/nodes/dryrun`
  - Body: { spec | template, mode?: 'adapter' }
  - Resp: { report: { deps, links, risks }, ok: bool }

- POST `/factory/nodes`
  - Gate: `factory_adapter=experimental`
  - Body: { spec | template, hitl?: true }
  - Resp: { id, state: 'draft' }

- POST `/factory/nodes/:id/approve`
  - Moves: draft→canary or canary→experimental
  - Resp: { id, state }

- POST `/factory/nodes/:id/disable|rollback`
  - Resp: { id, state: 'disabled'|'rolled_back' }

Adapter Contracts (обязательные хуки)

- Registry: регистрация NodeTemplate в файловом каталоге (`/nodes` API) + индексация в реестре.
- Hub/NS/IS: автопубликация метрик (`factory_*`), статусы в интроспекции, проверки safe‑mode/политик.
- Состояния: Draft → Canary → Experimental → Stable → (Disabled/RolledBack) — коды выдаются в API.
- Ошибки: унифицированный формат { code, reason, capability? } (Policy Engine); при выключенном гейте — `capability_disabled`.

- POST `/organs/build`
  - Gate: `organs_builder=experimental`
  - Body: { organ_template, dryrun?: true }
  - Resp: { organ_id, state: 'draft'|'canary'|'experimental'|'stable' }
  - Logs `organ build started` и метрики `organ_build_attempts_total`, `organ_build_duration_ms`
  - Задержки стадий берутся из `ORGANS_BUILDER_STAGE_DELAYS` (пример: `50,100,200` → canary/experimental/stable)

- GET `/organs/:id/status`
  - Resp: { id, state, nodes, metrics }
  - Метрика: `organ_build_status_queries_total`

- POST `/organs/:id/status`
  - Body: { state: 'draft'|'canary'|'experimental'|'stable'|'failed' }
  - Resp: { id, state }
  - Позволяет вручную продвигать орган по стадиям

- DELETE `/organs/:id/build`
  - Останавливает сборку органа и переводит его в `failed`

## Examples

Request (dry‑run, adapter):

```
POST /factory/nodes/dryrun
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
ORGANS_BUILDER_STAGE_DELAYS=50,100,200
```

## Sample Templates

- [examples/organs/organ.echo.v1.json](../../examples/organs/organ.echo.v1.json) — минимальный эхо‑орган.
- [examples/organs/organ.reverse.v1.json](../../examples/organs/organ.reverse.v1.json) — орган, разворачивающий текст.

## Organs Panel

1. `npm run frontend:dev`
2. Открыть `http://localhost:5173/src/organs.html`.

Notes

- Exec backends (script/wasm) остаются locked. Сначала adapter‑only.
