#!/usr/bin/env bash
set -euo pipefail

# Detect duplicate crate versions in Cargo dependencies.
output=$(cargo tree -d)
if [ -n "$output" ]; then
  echo "$output"
  echo "Duplicate crate versions detected. Ensure a single version per crate." >&2
  exit 1
fi
