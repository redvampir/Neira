<!-- neira:meta
id: NEI-20250902-202115-rename-backend
intent: docs
summary: |
  Уточнено название каталога: backend/services -> spinal_cord/services.
-->
<!-- neira:meta
id: NEI-20250214-120000-lymph-filter
intent: docs
summary: Добавлено решение о подсистеме «Лимфатический фильтр».
-->
// Architectural Decision Records (ADR Index)

Format
- ADR-XXX Title — Status (Proposed/Accepted/Rejected/Superseded)
- Context
- Decision
- Consequences
- Links

Index
- ADR-001 Rust as primary language — Accepted
- ADR-002 Лимфатический фильтр — Accepted

Templates
- Use this skeleton when adding a new ADR under `docs/adrs/ADR-XXX-<slug>.md`.

---

ADR-001 Rust as primary language — Accepted
- Context: Safety (memory correctness), performance, strong tooling, suitability for long-running services with low overhead and predictable behavior. Owner prefers Rust for control and reliability.
- Decision: Implement spinal_cord/services in Rust; prefer Rust-first libs; keep interfaces simple for future polyglot modules.
- Consequences: Higher initial complexity vs scripting, but better long-term stability; need strict coding guidelines and documentation to mitigate onboarding cost.

ADR-002 Лимфатический фильтр — Accepted
- Date: 2025-02-14
- Responsible: С. Иванов
- Context: Требуется очистка входящих сигналов и защита от вредных артефактов.
- Decision: Принять подсистему «Лимфатический фильтр» для анализа и фильтрации данных.
- Consequences: Повышает устойчивость и безопасность, но требует мониторинга расхода ресурсов.
