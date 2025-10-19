#!/usr/bin/env bash
: <<'neira:meta'
id: NEI-20250829-180333-check-duplicates
intent: docs
summary: |
  Скрипт проверяет наличие дублированных версий crate в зависимостях Cargo.
neira:meta

# neira:meta
# id: NEI-20250904-duplicates-allowlist
# intent: ci
# summary: Добавлен allowlist для Windows/WASI семейств в отчёте cargo tree -d, чтобы не блокировать CI на неизбежных дублях.

# neira:meta
# id: NEI-20261009-ignore-same-version
# intent: ci
# summary: Игнорируется дубль crate, если присутствует только одна версия.

# neira:meta
# id: NEI-20250221-parse-cargo-tree
# intent: ci
# summary: Исправлен парсинг вывода cargo tree -d, чтобы скрипт проверял все crates.

set -euo pipefail

# Detect duplicate crate versions in Cargo dependencies.
# Allow-list crate name prefixes that are known to be duplicated upstream
# and currently cannot be unified due to transitive constraints.
# We still fail CI on any other duplicates to prevent dependency bloat.
raw_output=$(cargo tree -d --target all 2>/dev/null)
if [ "$raw_output" = "nothing to print" ]; then
  raw_output=""
fi

filtered=$(RAW_OUTPUT="$raw_output" python3 - <<'PY'
import os
import re

allow = re.compile(r"^(wasi|windows(|-sys|-core|-targets)|windows_[A-Za-z0-9_]+)$")
blocks = {}
order = []

for line in os.environ.get("RAW_OUTPUT", "").splitlines():
    match = re.search(r"([A-Za-z0-9_-]+) v([0-9][^ ]*)", line)
    if not match:
        continue

    name, version = match.groups()
    if allow.match(name):
        continue

    block = blocks.get(name)
    if block is None:
        block = {"lines": [], "versions": []}
        blocks[name] = block
        order.append(name)

    block["lines"].append(line)
    if version not in block["versions"]:
        block["versions"].append(version)

out_lines = []
for name in order:
    block = blocks[name]
    if len(block["versions"]) > 1:
        if out_lines:
            out_lines.append("")
        out_lines.extend(block["lines"])

if out_lines:
    print("\n".join(out_lines))
PY
)

if [ -n "$filtered" ]; then
  echo "$filtered"
  echo "Duplicate crate versions detected (excluding known Windows/WASI families)." >&2
  exit 1
fi
