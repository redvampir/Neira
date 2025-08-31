<!-- neira:meta
id: NEI-20250923-taxonomy
intent: docs
summary: Консистентная таксономия терминов и соглашения по именованию/версиям.
-->

# Таксономия и соглашения

Термины
- Node (узел): минимальный исполняемый элемент (Analysis/Action/Chat/Memory‑service).
- Organ (орган): связка узлов с ролями/каналами/политиками.
- System/SubSystem: объединения органов/узлов по функции (Vision/Hearing/FS/Net и т. п.).

Идентификаторы
- `kind.namespace.name[:version]` — напр. `analysis.summarize.v1`, `action.factory.adapter.v1`.
- Органы: `organ.vision.v1`.

Статусы
- Draft / Canary / Experimental / Stable / Disabled / Deprecated / RolledBack — единообразно во всех документах и API.

Прочее
- Все шаблоны и спецификации валидируются по JSON‑схемам в каталоге `schemas/`.
- Политики и гейты отражаются в CAPABILITIES.md и Policy Engine docs.
