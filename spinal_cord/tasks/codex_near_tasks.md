---
neira:meta:
  id: "NEI-20251103-codex-near-tasks"
  intent: "codex_tasks"
  summary: "Короткий список ближайших рутинных задач для Codex — скрипты, fixtures, тесты, CI."
---

Приоритетные задачи для Codex (near-term)

1) preprocess_audio.py
- Что: скрипт конвертации/нормализации .wav → 16kHz mono, rms нормализация.
- Входы: spinal_cord/training/fixtures/audio/*.wav
- Выходы: spinal_cord/training/processed/*.wav
- Verify: unit test tests/test_preprocess.py проверяет sample длину/частоту.

2) Генерация fixtures для phonetics
- Что: расширить phonetics_samples.json до 10 примеров с метаданными.
- Входы: existing phonetics_samples.json
- Выходы: updated spinal_cord/training/fixtures/phonetics_samples.json
- Verify: валидатор JSON schema (если есть) и локальный запуск pipeline с --dry-run.

3) pipeline_runner CLI
- Что: маленькая CLI обёртка run_pipeline.py для локального запуска DigestivePipeline с указанием манифеста.
- Входы: path к fixtures
- Выходы: лог + exit code 0 при успешной валидации
- Verify: тест запуска в контейнере/CI (mocked).

4) Unit / Integration tests
- Что: tests для preprocess_audio, pipeline_runner и проверка store_parsed_input mock.
- Verify: pytest проходит локально.

5) CI step
- Что: добавить job в существующий CI: запуск линтеров и pytest для новых тестов.
- Verify: CI green для PR.

Notes:
- Не хардкодить секреты. Конфиги/пороги брать из spinal_cord/config или env.
- Любые изменения, влияющие на feature gates — пометить locked и добавить neira:meta.
