<!-- neira:meta
id: NEI-20250923-state-recovery-design
intent: design
summary: Узлы без потери памяти: постоянное состояние, автоподхват на старте, upgrade hooks, runtime‑параметры и snapshot++.
-->

# State & Recovery (состояние узлов и автоподхват)

Purpose
- Сохранить навыки и параметры узлов между перезапусками/перекомпиляциями.
- Позволить эволюцию (upgrade) без переобучения и с быстрым откатом.

Layout
- `templates/<id>-<version>.json` — спецификации (NodeTemplate).
- `state/<id>/<version>/state.json` — параметры/счётчики/флаги обучения (NodeState).
- `models/<id>/<version>/*` — артефакты (вектора, статистики); `models/<id>/current` → активная версия.

Discovery (автоподхват)
- На старте: скан `templates/` и `state/`; регистрация узлов через Adapter (validate→register→ns/is hooks).
- Восстановление параметров из `state.json`; проверка зависимостей органов; метки `pending/ready`.
- Интроспекция: список восстановленных узлов/версий и результат recovery (ok/warn/fail).

Upgrade Hooks (эволюция без потери)
- Жизненный цикл: Draft→Canary→Experimental→Stable (общий для узлов/органов).
- Хуки: `pre_upgrade` (чекпойнт) → `migrate_state(old→new)` → `post_upgrade` (warm‑up).
- Режим blue/green + shadow‑run: новая версия в “тени”, сравнение метрик, затем promote.

Runtime Config Overlay
- Узлы принимают `NodeConfig` (JSON) с overlay: `input_format`, `thresholds`, `features[]` и др.
- Источник параметров (template vs runtime) виден в интроспекции; изменение overlay не требует сборки.

Snapshot++ и Audit
- Snapshot включает `templates,state,models` (+ ссылки на артефакты) — переносимость среды и быстрый restore.
- Журнал событий: create/approve/upgrade/rollback; diff dry‑run отчётов/спецификаций.

Policies & Safety
- Все изменения — под Policy Engine (гейты/approvals/роли), безопасные дефолты (safe‑mode write=admin).
- Откат по ревизии (быстрый), quarantine на нарушения целостности.

See also
- design/factory-system.md, design/policy-engine.md, design/system-lifecycle.md
