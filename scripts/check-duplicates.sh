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

set -euo pipefail

# Detect duplicate crate versions in Cargo dependencies.
# Allow-list crate name prefixes that are known to be duplicated upstream
# and currently cannot be unified due to transitive constraints.
# We still fail CI on any other duplicates to prevent dependency bloat.
raw_output=$(cargo tree -d --target all 2>/dev/null)
if [ "$raw_output" = "nothing to print" ]; then
  raw_output=""
fi

filtered=$(printf "%s" "$raw_output" \
  | awk 'BEGIN{RS=""; ORS="\n\n"} {
      # первым словом идёт имя crate, вторым — версия
      name=$1; ver=$2;
      if (name ~ /^(wasi|windows(|-sys|-core|-targets)|windows_[A-Za-z0-9_]+)$/) next;
      block[name]=block[name] (block[name]!=""?ORS:"") $0;
      if (vers[name] !~ "(^| )" ver "( |$)") vers[name]=vers[name] " " ver;
    }
    END {
      for (n in block) {
        split(vers[n], arr, " ");
        count=0; for (i in arr) if (arr[i]!="") count++;
        if (count>1) print block[n];
      }
    }' \
)

if [ -n "$filtered" ]; then
  echo "$filtered"
  echo "Duplicate crate versions detected (excluding known Windows/WASI families)." >&2
  exit 1
fi
