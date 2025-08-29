#!/usr/bin/env bash
: <<'neira:meta'
id: NEI-20250829-180333-check-duplicates
intent: docs
summary: |
  Скрипт проверяет наличие дублированных версий crate в зависимостях Cargo.
neira:meta

set -euo pipefail

# Detect duplicate crate versions in Cargo dependencies.
output=$(cargo tree -d)
if [ -n "$output" ]; then
  echo "$output"
  echo "Duplicate crate versions detected. Ensure a single version per crate." >&2
  exit 1
fi
