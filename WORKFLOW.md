<!-- neira:meta
id: NEI-20250225-120100-workflow-doc-map
intent: docs
summary: |
  Добавлено напоминание обновлять docs/index.md через gen-doc-map.
-->
<!-- neira:meta
id: NEI-20260413-workflow-rename
intent: docs
summary: Пример scope обновлён под каталог spinal_cord.
-->

// Execution Workflow for the Assistant

Start-of-task

- Read AGENTS.md, README.md, and task prompt.
- Confirm scope and risks if unclear. Create/update a short plan.
- Run/consider capability probes (CPU/Mem/IO/Net) to choose sane defaults and avoid hard-coded limits.

Implementation

- Explore with rg/ls; patch code via apply_patch.
- Keep diffs minimal. Avoid unrelated edits.
- Add env/docs updates when behavior or endpoints change.
- При изменении файлов в docs/ запускай `npm run gen-doc-map` для обновления docs/index.md.
- Prefer adaptive parameters (config/env/probes) over constants.

Validation

- cargo check/build/test where applicable. Summarize results.
- Surface metrics and logs to validate runtime effects when possible.
- (Optional) Simulate high/low capability scenarios and observe auto‑tuning behavior.

Handover

- Summarize changes (what/why/where), any follow-ups, and how to verify.
- Propose 1–10 potential improvements (backlog) with short impact notes.

Staged Capability Rollout

- Gate every new capability behind a feature flag/state (see CAPABILITIES.md).
- Default state: locked; move to experimental with safeguards, then stable by criteria.
- On owner phrase to unlock/lock: apply the gate, report state, risks, rollback path.
- Include metrics/SLO to judge stabilization; avoid enabling multiple complex gates at once.

Comments & Metadata (for the Assistant)

- When to add: only for notable changes (new/changed endpoints, env vars, metrics, auth/safe-mode/permissions, data format/migration). Skip for trivial tweaks.
- Where to add: one block per touched file (top-of-file) or immediately above the primary changed function.
- Format: machine-readable YAML in a block comment with a stable marker `neira:meta`.

Template (Rust/JS/TS block comment)
/\* neira:meta
id: NEI-YYYYMMDD-HHMMSS-<slug>
intent: feature|fix|refactor|docs|perf|security|chore
scope: spinal_cord/<area>
summary: |
Коротко по-русски что и зачем изменено (1–2 строки).
links:

- docs/backend-api.md#section
- DECISIONS.md#ADR-XXX
  env:
- SOME_ENV_FLAG
  metrics:
- some_metric_name
  endpoints:
- POST /api/neira/chat/stream
  risks: low|medium|high
  safe_mode:
  affects_write: true|false
  requires_admin: true|false
  i18n:
  reviewer_note: |
  1-2 короткие фразы по-русски для владельца.
  \*/

Conventions

- Keys на английском (стабильные), текстовые значения по-русски.
- Одна запись на файл за изменение; при повторных правках — обновлять тот же блок.
- Длина строки ≤ 120 символов. Использовать относительные ссылки на файлы.

Owner-facing notes

- По умолчанию не добавлять отдельные «человеческие» комментарии в код.
- В хендовере давать 2–3 маркера по-русски: «что», «зачем», «как проверить».
