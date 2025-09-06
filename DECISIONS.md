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
<!-- neira:meta
id: NEI-20270615-lymphatic-dup-adr
intent: docs
summary: ADR о включении анализа дубликатов через лимфатический фильтр.
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
- ADR-003 Включение лимфатического фильтра при событиях — Accepted

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

ADR-003 Включение лимфатического фильтра при событиях — Accepted
- Date: 2027-06-15
- Responsible: С. Иванов
- Context: Требуется автоматически проверять новые гены на дубликаты при создании клеток и органов.
- Decision: Запускать `lymphatic_filter::scan_workspace` на событиях `CellCreated` и `OrganBuilt`, публикуя `lymphatic.duplicate_found`.
- Consequences: Небольшая нагрузка на CPU, но раннее выявление дубликатов и очистка кода.
