#!/usr/bin/env bash
# neira:meta
# id: NEI-20270311-120020-setup-git-attrs
# intent: chore
# summary: |
#   Регистрирует merge driver cargo-toml в локальной конфигурации Git.

set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

git config merge.cargo-toml.driver 'bash scripts/merge-cargo-toml.sh %O %A %B'
