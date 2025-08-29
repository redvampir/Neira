# Anti‑Idle System (Система активного времени)

Цель — продуктивно использовать время простоя для саморазвития и (позже) монетизации при строгой безопасности и контроле.

## 1) Компоненты (узлы)
- IdleDetectionNode: определяет состояние простоя (ActiveWithUser, ShortIdle, LongIdle, DeepIdle) по активности/времени.
- GoalManagementNode: глобальные цели, активная цель, прогресс, зависимость целей.
- MicroTaskGeneratorNode: шаблоны микрозадач и подбор активных задач под текущую цель/состояние.
- SafetyControllerNode: правила безопасности (Block/RequestApproval/Log/Pause), квоты времени, аварийная остановка.
- ReflectionNode: внутренний «дневник размышлений», вопросы к владельцу, инсайты и сомнения.
- IncomeGenerationNode (позже): безопасные оффлайн‑задачи с оценкой ценности — ЛОКИРОВАНО на старте.
- ActivityReportNode: краткий отчёт «что сделано в простое» при возвращении пользователя.

## 2) Конфигурация и данные
- Конфиг `config/idle_system.toml`:
  - idle_detection: пороги простоя, длительность deep idle
  - safety: лимиты автономии, слова аварийной остановки, требование одобрения
  - learning/income/reflection: отдельные лимиты сессий
  - priorities: приоритеты классов задач (user_response > system_maintenance > …)
  - reporting: частота отчётов и детализация
- Состояние/каталоги:
  - `data/idle/goals.toml` — цели и зависимости
  - `data/idle/progress.json` — прогресс/активная цель/чекпоинты
  - `data/idle/journal.ndjson` — краткие записи (можно лентой)

## 3) Политики и лимиты
- Приоритет 1: всегда немедленный ответ пользователю (автозадачи прерываются).
- Квоты: `micro_task_max_duration`, `session_max_duration`, `daily_autonomous_limit`, отдельные лимиты на learning/money/reflection.
- Без сети/внешних ресурсов на старте; только локальные данные.
- Правила SafetyController: белые/чёрные списки доменов задач; запрос одобрения на рискованные действия.

## 4) Метрики (добавить в docs/reference/metrics.md)
- idle_state (gauge: 0=active,1=short,2=long,3=deep)
- idle_minutes_today (counter)
- auto_tasks_started/auto_tasks_completed/auto_tasks_blocked (counters)
- approvals_pending (gauge)
- autonomous_time_spent_seconds (counter)
- microtask_queue_depth (gauge)

## 5) ENV (добавить в docs/reference/env.md)
- idle_system.* секция: пороги простоя, лимиты времени, приоритеты, флаги одобрения и отчётности.

## 6) Гейты способностей (CAPABILITIES.md)
- anti_idle_core — experimental (включает IdleDetection + Safety + Reporting «скелет»)
- learning_microtasks — experimental (без внешней сети)
- reflection_journal — experimental
- income_generation — locked (только документ/план)
- gaming_playground — locked

## 7) Зачаточный режим (Embryo)
- По умолчанию активен только каркас `anti_idle_core` в режиме safe defaults:
  - IdleDetectionNode задаёт idle_state, но не инициирует выполнение задач.
  - ReflectionNode пишет «мысли» и вопросы в журнал (без внешнего доступа).
  - API отчёта — только чтение (выдача краткого отчёта/журнала), выполнение микрозадач отключено.
  - Любые включения модулей требуют admin и открытого гейта; safe‑mode блокирует модификации.
- Разблокировка по шагам простыми фразами владельца (см. AGENTS.md/Feature Gates):
  - `learning_microtasks` → разрешить безопасные локальные учебные микрозадачи с квотами.
  - `reflection_journal` → расширить отчёты/вопросы/инсайты.
  - `income_generation`/`gaming_playground` остаются locked.

## 8) Отчётность
- ActivityReportNode формирует резюме при возврате пользователя: обучение/размышления/прогресс/вопросы; ссылки на артефакты.
- Записи journaling: что/зачем/как проверили, в связке с neira:meta id.

## 9) Этапы
- Stage 0: документация + конфиги + гейты; метрики idle_state и счётчики; dry‑run без автозадач.
- Stage 1: learning_microtasks + reflection_journal (experimental); без сети.
- Stage 2+: income_generation (ограниченные оффлайн‑задачи), далее — по согласованию.

