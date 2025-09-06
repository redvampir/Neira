#!/usr/bin/env bash
# neira:meta
# id: NEI-20270311-120010-merge-cargo-script
# intent: chore
# summary: |
#   Объединяет варианты Cargo.toml через toml-merge с проверкой целостности.

set -euo pipefail

base="$1"
ours="$2"
theirs="$3"

if ! command -v toml-merge >/dev/null 2>&1; then
  echo "Ошибка: toml-merge не найден. Установите: cargo install toml-merge" >&2
  exit 1
fi

tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT

toml-merge "$base" "$ours" "$theirs" >"$tmp"

if command -v tomlq >/dev/null 2>&1; then
  tomlq -r '.' "$tmp" >/dev/null || {
    echo "Ошибка: некорректный TOML после объединения" >&2
    exit 1
  }
fi

mv "$tmp" "$ours"
