// Architectural Decision Records (ADR Index)

Format
- ADR-XXX Title — Status (Proposed/Accepted/Rejected/Superseded)
- Context
- Decision
- Consequences
- Links

Index
- ADR-001 Rust as primary language — Accepted

Templates
- Use this skeleton when adding a new ADR under `docs/adrs/ADR-XXX-<slug>.md`.

---

ADR-001 Rust as primary language — Accepted
- Context: Safety (memory correctness), performance, strong tooling, suitability for long-running services with low overhead and predictable behavior. Owner prefers Rust for control and reliability.
- Decision: Implement backend/services in Rust; prefer Rust-first libs; keep interfaces simple for future polyglot modules.
- Consequences: Higher initial complexity vs scripting, but better long-term stability; need strict coding guidelines and documentation to mitigate onboarding cost.
