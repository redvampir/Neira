<!-- neira:meta
id: NEI-20250923-system-lifecycle
intent: design
summary: Согласованная схема слоёв и жизненный цикл «отращивания органа» (от запроса до эксплуатации/отката).
-->

# Слоистая архитектура и жизненный цикл

Слои
- Core: Hub/Registry/Queues
- Nervous System: метрики/пробы/вотчдоги/интроспекция
- Immune System: безопасность/интегритет/карантин/safe‑mode
- Factory/Organs: рост способностей (узлы/органы)
- HITL Training: стабилизация/обучение
- Control: админ‑плоскость и снапшоты

Конвенции
- Идентификаторы: `kind.namespace.name[:version]` (напр. `analysis.summarize.v1`), органы: `organ.vision.v1`.
- Статусы: Draft/Canary/Experimental/Stable/Disabled/Deprecated/RolledBack.
- Интеграции: каждый узел/орган обязан публиковать метрики и присутствовать в интроспекции.

Жизненный цикл «отращивания органа»
1) Идея/Запрос: формирование OrganTemplate (роли/каналы/зависимости, use‑cases).
2) Dry‑Run: проверка совместимости/политик/ресурсов; отчёт рисков; NS/IS pre‑checks.
3) Фабрикация узлов (Adapter‑only): недостающие узлы создаются в Draft→Canary (теневой трафик, отчёты) → Experimental.
4) Сборка органа (Canary): ограниченный трафик, SLO/ошибки/ресурсы; готовность к откату.
5) Обучение/Стабилизация: HITL микроциклы до конвергенции; approve → повышение статуса.
6) Выпуск: Experimental→Stable; фиксация версий, запись в журнал/индекс органов.
7) Эксплуатация: мониторинг, алерты, бюджеты Homeostasis.
8) Изменения: миграции через тот же цикл (dry‑run→canary→…).
9) Деактивация/Откат: быстрый rollback/quarantine, миграция трафика.

См. также
- docs/design/nervous_system.md, docs/design/policy-engine.md, docs/design/factory-system.md, docs/design/organ-systems.md, docs/design/homeostasis.md.
