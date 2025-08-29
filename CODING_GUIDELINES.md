// Coding Guidelines

Scope & Style
- Keep changes focused on the task; avoid broad refactors.
- Follow existing module patterns and naming.
- Prefer explicit over clever; prioritize readability.

Sizing & Structure
- File size: target 200–400 lines; 400–800 is heavy, split where reasonable; avoid >800.
- Function size: handlers 20–60 lines; generally up to 50–80. Extract helpers/services.
- One responsibility per module/file; separate DTOs, services, infrastructure.
- Keep tests/fixtures out of prod code (separate files/dirs).

Safety
- No destructive data ops without approval.
- Gate risky features behind env flags.

Observability
- Add metrics/counters for new flows; prefer existing registries/names.
- Update ENV.md and docs for any new env vars, flags, or endpoints.

Testing & Docs
- Use cargo check/test where available; summarize results.
- Document endpoints, request/response changes, metric names, and flags.

Adaptivity & Capability Discovery
- Avoid hard-coded constants. Use config/env with sane defaults; allow runtime tuning.
- Add capability probes (CPU/Mem/IO/Net) to choose pool sizes, batch sizes, timeouts.
- Implement back-off and circuit-like controls based on error/latency metrics.
- Prefer feature flags/policies to toggle subsystems and aggressiveness levels.
- Expose introspection endpoints/metrics that reflect current capabilities and policies.
