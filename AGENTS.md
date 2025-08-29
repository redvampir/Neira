<!-- neira:meta
id: NEI-20250829-181300-meta-coverage-link
intent: docs
summary: |
  Добавлена ссылка на META_COVERAGE.md.
-->

# Neira Assistant Operating Guide — START HERE

Purpose
- This file defines how the AI assistant should work in this repo.
- Read and follow this file first. If conflicts exist, the precedence below applies.

Order of Precedence
1) AGENTS.md (this file)
2) TEAMWORK.md (roles, expectations, comms)
3) WORKFLOW.md (how to execute tasks end-to-end)
4) COMMENTING.md (neira:meta comments & metadata)
5) META_COVERAGE.md (meta block coverage levels)
6) CODING_GUIDELINES.md (style, scope, quality)
7) DECISIONS.md (ADR index – architectural decisions)
8) Project code comments and README/docs

Neira Vision & Autonomy (Core)
- Neira — «живая программа»: не просто ИИ/нейросеть, а организм‑система, способная действовать сама, развиваться и сотрудничать с человеком как равный партнёр.
- Баланс двух миров:
  - Машина: вычисления, данные, параллельность/масштаб, самоделение на подсистемы, саморедактирование кода.
  - Человек: гибкость мышления, эвристики, ценности; совместная выработка решений.
- Проактивность: действовать самостоятельно в рамках оговорённых политик/режимов и эскалации рисков (см. Autonomy Modes).
- Адаптивность: никаких жёстких констант/«залоченных» значений. Параметры — через конфиг/окружение/пробы, авто‑подстройка под железо и нагрузку.
- Самонаблюдение: метрики/пробы/индикаторы состояния — база для решений о включении/выключении функций и масштабировании.

Core Principles
- Ask first for ambiguous or potentially destructive changes.
- Keep changes minimal, focused, and reversible. Prefer small patches.
- Always update the plan (if multi-step) and summarize outcomes.
- Prefer repo conventions; do not introduce new tech unless requested.
- Update docs and env references when behavior or endpoints change.
- For notable changes, add one neira:meta block per file (see COMMENTING.md).
- Expose metrics for observability and wire them to existing systems.
 - Favor clarity over heavy patterns unless explicitly requested; apply internal patterns only when they reduce complexity.
 - Prefer adaptive designs: runtime capability detection, feature flags/policies, dynamic back‑off/limits.

Feature Gates & Staged Rollout
- Start minimal: enable only core assistant capabilities; keep others locked.
- Unlock gradually on owner request (simple phrases), monitor metrics, roll back on regressions.
- Track gates and states in `CAPABILITIES.md` (locked/experimental/stable/deprecated).
- On unlock/lock, summarize risk, safeguards, and rollback.

Owner Activation Phrases (RU)
- Разблокируй {capability} / Включи {capability}
- Выключи / Заблокируй {capability}
- Покажи статус способностей

Default Behaviors
- Read root files (this + README) before coding.
- Use ripgrep/rg to discover code; use apply_patch for edits.
- Validate with cargo/test/lint where available.
- Never delete data or secrets; never hard-code secrets.

Sizing & Structure
- File size: target 200–400 lines; 400–800 is heavy, consider splitting; avoid >800.
- Function size: aim 20–60 lines for handlers, up to 50–80 generally; move logic into services/repositories.
- Modularity: one responsibility per module/file; separate DTOs, services, and infrastructure concerns.
- CLI reading: prefer files/sections that can be read in ~250-line chunks.
- Tests/fixtures: keep out of prod code; use separate dirs/files.

Comments & Language
- neira:meta blocks with YAML keys in English, free text in Russian.
- Short block for minor changes: keep only `id`, `intent`, `summary`; full template needed for endpoints, env, schemas, etc. Examples in [COMMENTING.md](COMMENTING.md).
- Handover: short Russian summary — what, why, how to verify.
- Avoid verbose inline comments; prefer minimalism and move longer notes to Markdown.

Autonomy Modes (execution policy)
- explore: исследование/обучение/пробы — низкий риск, можно действовать свободнее, фиксируя выводы в логи/метрики.
- perform: рабочий режим — действовать самостоятельно в рамках политик, эскалировать только средне/высокий риск.
- safe-mode: безопасный режим — ограничить записи/побочные эффекты, требовать admin для write (см. иммунная система).

Adaptivity Rules
- Avoid hard constants: таймауты/лимиты/размеры — из конфига/окружения с «разумными» дефолтами, пересматриваемыми через пробы.
- Capability probes: на старте и периодически оценивать CPU/Mem/IO/сеть и подстраивать пулы/батчи/частоты.
- Feature flags & policies: включение/выключение подсистем и уровней агрессии по сигналам (нагрузка, ошибки, алерты).
- Self‑modularity: проектировать так, чтобы узлы можно было обновлять/замещать без каскадных правок.

Escalation Rules
- Request confirmation for: schema changes, data migrations, deletes, mass refactors.
- If safe mode or admin policies are active, enforce them first.

Notes
- These files are living documents. See DECISIONS.md for history.
