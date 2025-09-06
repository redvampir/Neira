#!/usr/bin/env bash
# neira:meta
# id: NEI-20250916-merge-comments-driver
# intent: ci
# summary: Авто-слияние комментариев: если различия только в комментариях, выбираем нашу версию.

set -euo pipefail

base="$1"
ours="$2"
theirs="$3"

# Получаем diff между нашей и их версией
if diff_output=$(diff -U0 --strip-trailing-cr "$ours" "$theirs" 2>&1); then
  # Файлы идентичны
  cat "$ours"
  exit 0
fi

# Если diff завершился с кодом, отличным от 1, это ошибка
status=$?
if [ "$status" -ne 1 ]; then
  echo "$diff_output" >&2
  exit "$status"
fi

comment_only=true
while IFS= read -r line; do
  case "$line" in
    ---*|+++*|@@*)
      continue
      ;;
    [+-]*)
      text="${line:1}"
      trimmed="$(printf '%s' "$text" | sed -e 's/^[[:space:]]*//')"
      if [[ "$trimmed" =~ ^(//|/\*|#|<!--) ]]; then
        continue
      else
        comment_only=false
        break
      fi
      ;;
  esac
done <<< "$diff_output"

if [ "$comment_only" = true ]; then
  cat "$ours"
else
  git merge-file --stdout "$ours" "$base" "$theirs"
fi
