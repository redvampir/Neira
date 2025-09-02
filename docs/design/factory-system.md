<!-- neira:meta
id: NEI-20250923-factory-system-design
intent: design
summary: |
  Каркас системы Фабрикаторов: FabricatorCell/SelectorCell, жизненный цикл клеток (Draft→Canary→Experimental→Stable), интеграции с Nervous/Immune, HITL‑обучение и органы.
-->
<!-- neira:meta
id: NEI-20251115-organ-cancel-build-design
intent: design
summary: упомянут DELETE /organs/:id/build в API эскизе.
-->
<!-- neira:meta
id: NEI-20250316-stemcell-rename
intent: docs
summary: Термины обновлены на StemCellFactory/StemCellRecord/StemCellState.
-->

# Stem Cell Factory System (Фабрикаторы)

Purpose
- Создавать новые клетки (Analysis/Action/Memory) из дескрипторов с безопасными бэкендами исполнения и управлением рисками.
- Обеспечить повторное использование готовых клеток, автоподключение к ключевым системам, и «человек в петле» для обучения/стабилизации.
- Центральный сервис управления — **StemCellFactory**; записи о созданных клетках хранятся как **StemCellRecord** со статусами **StemCellState**.

Components
- FabricatorCell (Action): принимает FabricationRequest и создаёт клетки через бэкенды:
  - Adapter: адаптер над CellTemplate (без кода; маршрутизация/линки/пороги). [start: experimental]
  - Script: Rhai (sandbox, лимиты/политики). [locked]
  - WASM: WASI (CPU/Mem/IO лимиты, без сети). [locked]
- SelectorCell (Analysis): решает reuse vs create (подбор готового клетки по типу/сигнатурам/политикам); формирует FabricationRequest при необходимости.
- PolicyEngine: проверяет гейты/capabilities, права, ограничения safe‑mode, требуемые интеграции.
- Training Orchestrator (Action): мини‑циклы обучения и конвергенции новых клеток (HITL — approvals, отчёты, rollback).
- OrganBuilder (Action): сборка «органов» — связок клеток по OrganTemplate (граф/роли/каналы/политики) с dry‑run/canary.

Lifecycle (HITL)
- Draft → Canary → Experimental → Stable → Deprecated/Rollback.
- Draft: dry‑run, без сайд‑эффектов; только метрики/линки/совместимость.
- Canary: ограниченный трафик/теневой запуск; обязательны уведомления и approve.
- Graduation: по SLO/ошибкам/ресурсам и отсутствию нарушений политик.

Integrations
- Nervous System: публикация RED/USE метрик, watchdogs, экспорт `/metrics`.
- Immune System: quarantine hooks, integrity checks, audit trail; safe‑mode (write=admin).
- Introspection: статусы фабрикаторов/органов/клеток в `/api/neira/introspection/status`.

API (эскиз)
- POST `/factory/cells/dryrun` → {report: deps/compat/links, risks}
- POST `/factory/cells` → {id, state: draft} (гейт `factory_adapter`)
- POST `/factory/cells/:id/approve|disable|rollback`
- POST `/organs/build` (dryrun=true|false) → {organ_id, state}
- GET `/organs/:id/status`
- DELETE `/organs/:id/build`

Metrics (минимальный набор)
- factory_cells_created_total, factory_cells_active, factory_exec_errors_total
- factory_dryrun_requests_total, factory_approvals_total, factory_rollbacks_total
- organ_build_attempts_total, organ_build_failures_total
- training_iterations_total, training_converged_total

Feature Gates (CAPABILITIES)
- factory_adapter: experimental — включить адаптер (без кода) + dry‑run/HITL.
- factory_script: locked — Rhai‑бэкенд (sandbox, лимиты).
- factory_wasm: locked — WASI‑бэкенд (лимиты, без сети).
- organs_builder: experimental — сборка органов из шаблонов (dry‑run/canary/HITL).
- self_edit: locked — модификация собственных модулей под политиками.

Rollout (минимальная дорожная карта)
1) Включить factory_adapter + SelectorCell: только dry‑run + approve; интеграции NS/IS/Introspection.
2) OrganTemplate + OrganBuilder (dry‑run → canary) + отчёты в admin UI.
3) Training Orchestrator (HITL): мини‑циклы нормализации клеток до стабильности.
4) Точечное включение Script/WASM (внутренние сценарии, без внешней сети).

Notes
- Все опасные операции зафлажены; по умолчанию locked. Все переходы — с журналом и быстрым rollback.

## Natural Language Commands (эскиз)

- Примеры: “создай голосовой орган v1”, “обнови зрение до v2 (grayscale)”, “собери speak‑pipeline normalize→phonemes→adapter”.
- Парсинг команд (Command Cell) → генерация FactorySpec/OrganTemplate + Policy checks.
- В ответ — dry‑run отчёт/дифф, затем создание draft и проведение HITL‑циклов до canary/experimental.
