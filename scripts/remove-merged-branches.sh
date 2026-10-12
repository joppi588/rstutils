#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
#
# SPDX-License-Identifier: MIT

set -euo pipefail

usage() {
    cat << 'EOF'
Usage: remove-merged-branches.sh [options]

Remove local branches that are merged into main and older than 1 month (30 days).
Supports both standard git merges (ancestor / cherry-pick) and GitHub Squash & Merge commits.

Options:
  --dry-run       Show branches that would be deleted without deleting them (default)
  --force         Actually delete matching branches
  --days <N>      Specify age threshold in days (default: 30)
  --target <ref>  Specify target branch to check merge status against (default: main)
  -h, --help      Show this help message
EOF
    exit "${1:-0}"
}

ACTION="dry-run"
DAYS=30
TARGET="main"

while [[ $# -gt 0 ]]; do
    case "$1" in
        --dry-run)
            ACTION="dry-run"
            shift
            ;;
        --force)
            ACTION="force"
            shift
            ;;
        --days)
            if [[ $# -lt 2 || ! "$2" =~ ^[0-9]+$ ]]; then
                echo "Error: --days requires a positive integer argument." >&2
                exit 2
            fi
            DAYS="$2"
            shift 2
            ;;
        --target)
            if [[ $# -lt 2 ]]; then
                echo "Error: --target requires a branch name argument." >&2
                exit 2
            fi
            TARGET="$2"
            shift 2
            ;;
        -h|--help)
            usage 0
            ;;
        *)
            echo "Error: Unknown option '$1'" >&2
            usage 2
            ;;
    esac
done

if ! git rev-parse --verify "$TARGET" >/dev/null 2>&1; then
    echo "Error: Target branch '$TARGET' does not exist." >&2
    exit 1
fi

now_ts=$(date +%s)
cutoff_ts=$((now_ts - DAYS * 86400))
cutoff_date=$(date -d "@$cutoff_ts" "+%Y-%m-%d %H:%M:%S" 2>/dev/null || date -r "$cutoff_ts" "+%Y-%m-%d %H:%M:%S" 2>/dev/null || echo "$DAYS days ago")

current_branch=$(git branch --show-current 2>/dev/null || true)

# Collect all tree hashes in target branch commit history (for squash merge detection)
mapfile -t target_trees < <(git log "$TARGET" --pretty=format:'%T')
declare -A target_tree_set
for tree in "${target_trees[@]}"; do
    target_tree_set["$tree"]=1
done

is_branch_merged() {
    local branch="$1"
    local base="$2"

    # 1. Direct ancestor check (standard merge or fast-forward)
    if git merge-base --is-ancestor "$branch" "$base" 2>/dev/null; then
        return 0
    fi

    # 2. Tip tree identity check (GitHub Squash & Merge commits)
    local branch_tree
    branch_tree=$(git rev-parse "${branch}^{tree}" 2>/dev/null || true)
    if [[ -n "$branch_tree" && -n "${target_tree_set[$branch_tree]:-}" ]]; then
        return 0
    fi

    # 3. Cherry-pick check (all commits already in upstream with equivalent patch-ids)
    local cherry_out
    cherry_out=$(git cherry "$base" "$branch" 2>/dev/null || true)
    if [[ -n "$cherry_out" ]] && ! grep -q '^+ ' <<< "$cherry_out"; then
        return 0
    fi

    # 4. Merge-tree check (clean squash merge that produces no tree change vs target)
    local merge_tree target_tree
    if merge_tree=$(git merge-tree --write-tree "$base" "$branch" 2>/dev/null); then
        target_tree=$(git rev-parse "${base}^{tree}")
        if [[ "$merge_tree" == "$target_tree" ]]; then
            return 0
        fi
    fi

    return 1
}

echo "Scanning local branches merged into '$TARGET' older than $DAYS days (before $cutoff_date)..."
echo "Mode: $([[ "$ACTION" == "force" ]] && echo "DELETION (--force)" || echo "DRY-RUN (--dry-run)")"
echo ""

candidates=()
while read -r ts branch; do
    [[ -z "$branch" ]] && continue
    # Protect default and active branches
    [[ "$branch" == "$TARGET" || "$branch" == "main" || "$branch" == "master" ]] && continue
    [[ -n "$current_branch" && "$branch" == "$current_branch" ]] && continue

    if [[ "$ts" -lt "$cutoff_ts" ]]; then
        if is_branch_merged "$branch" "$TARGET"; then
            candidates+=("$branch")
            commit_date=$(date -d "@$ts" "+%Y-%m-%d" 2>/dev/null || date -r "$ts" "+%Y-%m-%d" 2>/dev/null || echo "timestamp $ts")
            echo "  $branch (last commit: $commit_date)"
        fi
    fi
done < <(git for-each-ref --format='%(committerdate:unix) %(refname:short)' refs/heads/)

echo ""
total=${#candidates[@]}

if [[ $total -eq 0 ]]; then
    echo "No matching branches found."
    exit 0
fi

if [[ "$ACTION" == "dry-run" ]]; then
    echo "Found $total branch(es) eligible for deletion."
    echo "Run with --force to remove these branches."
else
    echo "Deleting $total branch(es)..."
    for branch in "${candidates[@]}"; do
        # Use -D because squash-merged branches are not standard ancestors
        git branch -D "$branch"
    done
    echo "Successfully removed $total branch(es)."
fi
