#!/usr/bin/env bash
# neira:meta
# id: NEI-20250101-120000-merge-manifest
# intent: ci
# summary: |
#   Объединяет зависимости в Cargo.toml и package.json из конфликтующих веток.

set -euo pipefail

manifest="${1:-}"
if [[ -z "$manifest" ]]; then
  echo "Usage: $0 <manifest>" >&2
  exit 1
fi

file_name="$(basename "$manifest")"

merge_cargo() {
  local file="$1"
  if command -v tomlq >/dev/null 2>&1; then
    local ours_tmp theirs_tmp
    ours_tmp="$(mktemp)"
    theirs_tmp="$(mktemp)"
    git show ":2:$file" >"$ours_tmp"
    git show ":3:$file" >"$theirs_tmp"
    for section in dependencies dev-dependencies; do
      local ours_json theirs_json merged
      ours_json="$(tomlq -j ".${section} // {}" "$ours_tmp" 2>/dev/null || echo '{}')"
      theirs_json="$(tomlq -j ".${section} // {}" "$theirs_tmp" 2>/dev/null || echo '{}')"
      merged="$(jq -s '.[0] * .[1]' <(echo "$ours_json") <(echo "$theirs_json"))"
      tomlq -i ".${section} = ${merged}" "$ours_tmp"
    done
    mv "$ours_tmp" "$file"
    rm -f "$theirs_tmp"
  else
    if ! command -v cargo >/dev/null 2>&1; then
      echo "cargo executable not found; install Rust toolchain or provide tomlq for manifest merging" >&2
      return 1
    fi
    if ! cargo add --version >/dev/null 2>&1; then
      echo "cargo add (cargo-edit) is required for manifest merge fallback; install via 'cargo install cargo-edit --locked' or make tomlq available" >&2
      return 1
    fi
    local ours_dir theirs_dir
    ours_dir="$(mktemp -d)"
    theirs_dir="$(mktemp -d)"
    git show ":2:$file" >"$ours_dir/Cargo.toml"
    git show ":3:$file" >"$theirs_dir/Cargo.toml"
    CARGO_NET_OFFLINE=true cargo metadata --no-deps --format-version=1 --manifest-path "$ours_dir/Cargo.toml" >/dev/null 2>&1 || true
    CARGO_NET_OFFLINE=true cargo metadata --no-deps --format-version=1 --manifest-path "$theirs_dir/Cargo.toml" >/dev/null 2>&1 || true
    local dep kind name req ours_list theirs_list
    for kind in normal dev; do
      ours_list=$(CARGO_NET_OFFLINE=true cargo metadata --no-deps --format-version=1 --manifest-path "$ours_dir/Cargo.toml" 2>/dev/null \
        | jq -r ".packages[0].dependencies[] | select(.kind == \"$kind\" or (.kind == null and \"$kind\" == \"normal\")) | \"\(.name) \(.req)\"")
      theirs_list=$(CARGO_NET_OFFLINE=true cargo metadata --no-deps --format-version=1 --manifest-path "$theirs_dir/Cargo.toml" 2>/dev/null \
        | jq -r ".packages[0].dependencies[] | select(.kind == \"$kind\" or (.kind == null and \"$kind\" == \"normal\")) | \"\(.name) \(.req)\"")
      for dep in $theirs_list; do
        name="${dep%% *}"
        req="${dep#* }"
        if ! grep -q "^$name " <<<"$ours_list"; then
          if [[ "$kind" == "dev" ]]; then
            cargo add --offline --dev --manifest-path "$ours_dir/Cargo.toml" "$name@$req" >/dev/null
          else
            cargo add --offline --manifest-path "$ours_dir/Cargo.toml" "$name@$req" >/dev/null
          fi
        fi
      done
    done
    mv "$ours_dir/Cargo.toml" "$file"
    rm -rf "$ours_dir" "$theirs_dir"
  fi
}

merge_package() {
  local file="$1"
  local ours_tmp theirs_tmp
  ours_tmp="$(mktemp)"
  theirs_tmp="$(mktemp)"
  git show ":2:$file" >"$ours_tmp"
  git show ":3:$file" >"$theirs_tmp"
  local section merged
  for section in dependencies devDependencies; do
    merged=$(jq -s '.[0] * .[1]' <(jq ".${section} // {}" "$ours_tmp") <(jq ".${section} // {}" "$theirs_tmp"))
    jq ".${section} = \$m" --argjson m "$merged" "$ours_tmp" >"${ours_tmp}.tmp" && mv "${ours_tmp}.tmp" "$ours_tmp"
  done
  mv "$ours_tmp" "$file"
  rm -f "$theirs_tmp"
}

case "$file_name" in
  Cargo.toml)
    merge_cargo "$manifest"
    ;;
  package.json)
    merge_package "$manifest"
    ;;
  *)
    echo "Unsupported manifest: $manifest" >&2
    exit 1
    ;;
esac

