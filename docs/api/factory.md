<!-- neira:meta
id: NEI-20250923-factory-api-draft
intent: docs
summary: Черновой API Фабрикаторов (dry‑run/approve/rollback) и сборки органов.
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

- POST `/organs/build`
  - Gate: `organs_builder=experimental`
  - Body: { organ_template, dryrun?: true }
  - Resp: { organ_id, state: 'draft'|'canary'|'experimental'|'stable' }

- GET `/organs/:id/status`
  - Resp: { id, state, nodes, metrics }

Notes
- Exec backends (script/wasm) остаются locked. Сначала adapter‑only.
