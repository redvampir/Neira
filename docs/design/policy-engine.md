<!-- neira:meta
id: NEI-20250923-policy-engine-docs
intent: design
summary: Единый слой политик/гейтинга: фич-флаги, safe-mode, approvals, матрица прав и форматы отказов.
-->

# Policy Engine (Политики и гейтинг)

Цели
- Централизовать управление возможностями: locked/experimental/stable/disabled.
- Обеспечить «человек в петле» (approvals) и безопасные дефолты (safe‑mode write=admin).

Состав
- Capability Gates: источник истинны — CAPABILITIES.md; состояния и условия включения.
- Safe‑Mode: write=admin, карантин, проверки целостности.
- Approvals: шаги, требующие подтверждения (schema changes, фабрикация, сборка органов, включение exec backendов).
- Матрица прав: роли (read/write/admin) × действия (см. таблицы API); соответствие заголовкам токенов/скоупам.

Форматы отказов
- Стандартизованный JSON: { code, reason, capability?, required_role?, suggestions? }.

Интеграции
- Интроспекция: блок `capabilities` + статусы policy (read‑only).
- Admin UI: быстрые переключатели флагов (только там, где это безопасно), список pending approvals.

Схемы
- policy.schema.json — базовая форма правила/гейта (черновик) в `schemas/`.

См. также
- CAPABILITIES.md, docs/api/*, docs/design/factory-system.md.
