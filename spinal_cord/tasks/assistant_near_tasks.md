---
neira:meta:
  id: "NEI-20251103-assistant-near-tasks"
  intent: "assistant_tasks"
  summary: "Короткий список ближайших стратегических и ревью задач для ассистента."
---

Задачи для ассистента (near-term)

1) Принятие/ревью запроса на разблокировку phonetics
- Что: проверить unlock_phonetics_request.v1.json, оценить риск, запросить owner confirmation.
- Verify: owner activation phrase или явное подтверждение в issue.

2) Определить acceptance thresholds
- Что: задать пороги (например, phoneme_error_rate <= X) для перехода из экспериментального в стабильный.
- Verify: документ с порогами в spinal_cord/docs/training_acceptance.md

3) Ревью PRs от Codex
- Что: проверять neira:meta, тесты, метрики, отсутствие секретов, feature gates.
- Verify: заполненный чеклист codex_pr_review.md в PR.

4) Подготовка eval suite и мониторинга
- Что: составить базовую eval_suite_ru (пара примеров human-in-loop) и метрики (turn_success_rate).
- Verify: smoke-run eval, report generation.

5) План следующего этапа обучения
- Что: собрать требования для next stage (mapping/vocab) и список необходимых ресурсов/данных.
- Verify: короткий план в spinal_cord/training/next_stage_plan.md

Notes:
- Для рискованных операций — требовать request_confirmation (AGENTS.md).
- Все решения документировать коротко и помещать neira:meta когда меняются файлы.
