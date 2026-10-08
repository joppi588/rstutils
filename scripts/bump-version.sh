#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
    printf 'Usage: %s <new-version>\n' "$0" >&2
    exit 2
fi

new_version="$1"
if [[ ! "$new_version" =~ ^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(-[0-9A-Za-z.-]+)?(\+[0-9A-Za-z.-]+)?$ ]]; then
    printf 'Invalid semantic version: %s\n' "$new_version" >&2
    exit 2
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/Cargo.toml"
temporary_manifest="$(mktemp)"
trap 'rm -f "$temporary_manifest"' EXIT

awk -v version="$new_version" '
    /^\[workspace\.package\]$/ {
        in_workspace_package = 1
        in_workspace_dependencies = 0
        print
        next
    }
    /^\[workspace\.dependencies\]$/ {
        in_workspace_package = 0
        in_workspace_dependencies = 1
        print
        next
    }
    /^\[/ {
        in_workspace_package = 0
        in_workspace_dependencies = 0
    }
    in_workspace_package && /^version = "/ {
        print "version = \"" version "\""
        replaced_package_version = 1
        next
    }
    in_workspace_dependencies && /^rstu_(ast|parser) = \{ version = "/ {
        sub(/version = "[^"]+"/, "version = \"" version "\"")
        replaced_dependencies++
        print
        next
    }
    { print }
    END {
        if (!replaced_package_version || replaced_dependencies != 2) exit 1
    }
' "$manifest" > "$temporary_manifest"

mv "$temporary_manifest" "$manifest"
trap - EXIT

cd "$repo_root"
cargo metadata --format-version 1 --no-deps > /dev/null
printf 'Workspace version set to %s.\n' "$new_version"
printf 'Review and commit Cargo.lock after running cargo check or cargo test.\n'
