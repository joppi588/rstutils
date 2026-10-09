#!/usr/bin/env bash
set -euo pipefail

usage() {
    echo "Usage: $0 <input.rst> [output.yaml]" >&2
    exit 2
}

[[ $# -ge 1 && $# -le 2 ]] || usage

input=$1
if [[ $# -eq 2 ]]; then
    output=$2
elif [[ $input == *.* ]]; then
    output="${input%.*}.yaml"
else
    output="${input}.yaml"
fi

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
manifest_path="$script_dir/../Cargo.toml"
temporary_output=$(mktemp "${output}.tmp.XXXXXX")
trap 'rm -f -- "$temporary_output"' EXIT

cargo run --quiet --manifest-path "$manifest_path" --bin rstu -- \
    convert "$input" --output yaml > "$temporary_output"
mv -- "$temporary_output" "$output"
