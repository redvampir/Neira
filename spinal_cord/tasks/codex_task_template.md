---
neira:meta:
  id: "NEI-20251101-codex-task-template"
  intent: "task_template"
  summary: "Краткий шаблон задачи для Codex: цель, вход, выход, тесты."
---

Title: Короткое название задачи (что сделать)

Description:
- Кратко: цель задачи и почему она нужна (1–2 предложения).

Inputs (файлы/данные):
- Перечислить ожидаемые входы: пути к файлам, схемы, fixtures.

Outputs (ожидаемый результат):
- Что должен вернуть/создать Codex: файлы, конфиги, тесты.

Acceptance tests / Verification:
- Минимальный набор тест-кейсов (куда положить fixtures и как запустить).
- Команды для локальной проверки (например, npm test / pytest / запуск pipeline).

Constraints / Notes:
- Ограничения по форматам, безопасность, не включать секреты.
- Указать neira:meta если изменение заметное.

Owner / Reviewer:
- Кто принимает результат и критерии приёмки.

Estimate:
- Примерно: small/medium/large

Example (коротко):
- Input: spinal_cord/training/fixtures/phonetics_samples.json
- Output: script preprocess_audio.py + unit tests in tests/
- Verify: pytest tests/test_preprocess.py (должен проходить)
