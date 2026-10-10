#!/usr/bin/env bash
set -euo pipefail

if [[ $# -gt 1 || ( $# -eq 1 && "$1" != "--dry-run" ) ]]; then
    printf 'Usage: %s [--dry-run]\n' "$0" >&2
    exit 2
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

cargo test --workspace

if [[ ${1:-} == "--dry-run" ]]; then
    for crate in rstu_ast rstu_parser rstu; do
        cargo package --list --allow-dirty --package "$crate" > /dev/null
    done
    printf 'Local package checks passed. No crates were published.\n'
    exit 0
fi

for crate in rstu_ast rstu_parser rstu; do
    cargo publish --package "$crate"
done
