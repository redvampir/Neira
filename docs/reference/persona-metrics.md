<!-- neira:meta
id: NEI-20250904-120940-persona-metrics
intent: docs
summary: |
  Метрики личности: дрейф, соответствие стилю, переключения ролей и артефакты рефлексии. Привязка к Stage 0/1 и CAPABILITIES.
-->

# Метрики личности (Persona)

## Содержание
- [Таблица метрик](#метрики-личности-persona)
- [Stage и связи](#stage-и-связи)
- [См. также](#см-также)

| имя | тип | единица | источник | описание |
|---|---|---|---|---|
| persona_drift_score | gauge | 0..1 | Analysis/Memory | Отклонение ответа от сводки ядра личности (0 — полное соответствие). |
| style_adherence | gauge | 0..1 | Dialogue/Voice | Доля соблюдения выбранного стиля/интенсивности. |
| role_switches_total | counter | count | Analysis | Количество переключений ролей (coder/editor/architect). |
| reflection_journal_entries | counter | entries | Memory | Количество записей в журнале рефлексии. |
| proposals_accepted_total | counter | count | Memory/Review | Принятые предложения по коррекции ядра/политик. |
| proposals_reverted_total | counter | count | Memory/Review | Откаты предложений после canary. |
| safety_breaches_total | counter | count | Immune | Нарушения политик безопасности. |
| safety_mitigations_total | counter | count | Immune | Сработавшие меры защиты/маскирования. |
| persona_tone_intensity | gauge | 0..1 | Dialogue/Tone Controller | Текущая интенсивность выбранного тона. |
| persona_tone_confidence | gauge | 0..1 | Dialogue/Tone Controller | Уверенность в выбранном тоне (агрегированная обратная связь). |
| persona_tone_transitions_total | counter | count | EventBus/Tone Controller | Количество переходов тона (лейблы from/to/reason). |
| persona_tone_feedback_total | counter | count | Tone Controller | Обратные связи (лейбл reason: observation/direct/reset/decay). |

Stage и связи
- Stage 0: рекомендованы к сбору `role_switches_total`, при наличии — `style_adherence` (neutral). Остальные — по готовности.
- Stage 1: включаются полностью; мониторинг SLO и условия отката для `persona_reflection`, `persona_style_teen`. `tone_state` переведён в Stage 0 (см. CAPABILITIES.md).

См. также
- Дорожная карта: docs/roadmap.md
- Ядро личности: docs/meta/persona-kernel.md
- Способности/флаги: CAPABILITIES.md

