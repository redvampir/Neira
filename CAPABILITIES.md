# Neira Capabilities & Feature Gates

Purpose
- Stage new abilities safely. Start minimal, unlock gradually.
- Prevent confusion and resource thrash by enabling only what’s ready.

States
- locked: implemented but disabled by default
- experimental: enabled with safeguards/limits, monitored
- stable: enabled by default in perform mode
- deprecated: slated for removal

Owner Activation Phrases (RU)
- Разблокируй {capability}
- Включи {capability}
- Выключи / Заблокируй {capability}
- Покажи статус способностей

Assistant Policy
- Treat these phrases as intent to flip the gate (after quick risk check).
- Always report back the new state, safeguards, and rollback.

Graduation Criteria (move experimental → stable)
- Error rate/latency within SLO for N runs/time window
- No safety/policy violations observed
- Resource impact within budget

Examples (YAML)
```yaml
capabilities:
  training_pipeline:
    state: locked
    notes: offline scripted training
  self_edit:
    state: locked
    notes: modify own modules under policies
  probes_autotune:
    state: experimental
    notes: tune pools/batches via capability probes
  quarantine_auto:
    state: stable
    notes: enforce safe-mode write=admin
```

