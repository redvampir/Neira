<!-- neira:meta
id: NEI-20250830-Persona-Capabilities
intent: docs
summary: |
  Перечень capability‑флагов личности и творческих студий с описанием, рисками, safeguards и откатами. Дополнение к CAPABILITIES.md.
-->

# Persona Capabilities (дополнение)

```yaml
capabilities:
  persona_kernel:
    state: stable
    notes: ядро личности (инварианты ценностей)
    safeguards: immutable values; audit changes
    signals: [persona_drift_score]

  persona_roles_minimal:
    state: stable
    notes: базовые роли coder/editor/architect
    safeguards: role contracts
    signals: [role_switches_total]

  persona_style_neutral:
    state: stable
    notes: нейтрально‑дружелюбный стиль по умолчанию
    signals: [style_adherence]

  persona_style_teen:
    state: experimental
    notes: «подростковая» окраска; интенсивность 0–3 (0 — выкл.)
    safeguards: explicit opt‑in; auto‑reset on critical tasks
    rollback: set intensity=0, disable flag
    signals: [style_adherence]

  persona_reflection:
    state: experimental
    notes: предложения микрокоррекций ядра/политик через JOURNALING (review→canary)
    safeguards: dry-run, audit trail
    rollback: revert proposal; disable flag
    signals: [reflection_journal_entries, proposals_accepted_total, proposals_reverted_total]

  tone_state:
    state: experimental
    notes: эфемерный тон/настроение; не трогает ценности
    safeguards: bounded scope & time; cap latency impact
    rollback: reset tone; disable flag
    signals: [style_adherence]

  studio_artflow:
    state: locked
    notes: генеративная графика (песочница)

  studio_soundweaver:
    state: locked
    notes: процедурная музыка/звук (песочница)

  studio_storynodes:
    state: locked
    notes: интерактивные микро‑истории (песочница)

  roleplay_mode:
    state: locked
    notes: ролевая симуляция личности; только явно и с дисклеймером
    safeguards: user confirmation; watermarking
    rollback: disable flag
```

См. также: CAPABILITIES.md, docs/meta/persona-kernel.md, docs/reference/persona-metrics.md, docs/roadmap.md
