<!-- neira:meta
id: NEI-20250330-immune-metrics-doc
intent: docs
summary: добавлен раздел с метриками иммунной системы.
-->
# Иммунная система

## Навигация
- [Обзор Нейры](README.md)
- [Клетки действий](cells/action-cells.md)
- [Клетки анализа](cells/analysis-cells.md)
- [Клетки памяти](cells/memory-cells.md)
- [Архитектура анализа](system/analysis-architecture.md)
- [Поддерживающие системы](system/support-systems.md)
- [Личность Нейры](meta/personality.md)
- [Шаблон клетки](cells/cell-template.md)
- [Политика источников](system/source-policy.md)
- [Система самообновления](system/self-updating-system.md)

## Оглавление
- [IntegrityCheckerCell](#integritycheckercell)
- [QuarantineCell](#quarantinecell)
- [SafeModeController](#safemodecontroller)

### IntegrityCheckerCell
`IntegrityCheckerCell` вычисляет и сравнивает контрольные суммы файлов с эталонными значениями из `config/integrity.json`. Клетка запускается при старте и по таймеру, регистрируя найденные расхождения. При обнаружении подозрительных изменений он публикует событие, которое перехватывают другие защитные клетки.

### QuarantineCell
`QuarantineCell` реагирует на сообщения о нарушении целостности. Клетка помечает затронутые файлы или пакеты как небезопасные и переносит их в отдельный каталог карантина. Пока объект находится в карантине, обращения к нему блокируются, а пользователю отправляется уведомление с рекомендациями.

### SafeModeController
`SafeModeController` переводит систему в ограниченный режим, если количество критических инцидентов превышает порог. В безопасном режиме загружается минимальный набор клеток, отключаются неосновные функции и активируется строгая проверка входящих данных. Выход из режима происходит после ручного подтверждения или успешного пересканирования.

## Метрики
- `immune_alerts_total{severity}` (counter, alerts): счётчик алертов иммунной системы с лейблом `severity`; помогает отслеживать частоту предупреждений разных уровней.
- `immune_actions_total{action}` (counter, ops): успешные действия защиты, лейбл `action` показывает тип реакции (`quarantine`, `safe_mode`, `integrity_check`, `init_config`).
- `immune_action_failures_total{action}` (counter, ops): неудачные действия защиты по типам, сигнализируют о проблемах исполнения.
