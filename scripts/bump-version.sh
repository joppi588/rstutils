#!/usr/bin/env bash
set -euo pipefail

cargo set-version --workspace --bump $1
