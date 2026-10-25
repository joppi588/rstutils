#!/usr/bin/env bash
set -euo pipefail

usage() {
    echo "Usage: $0 <input.rst> [output.yaml] [rstu options...]" >&2
    exit 2
}

[[ $# -ge 1 ]] || usage

input=$1
shift

if [[ $# -gt 0 && $1 != -* ]]; then
    output=$1
    shift
elif [[ $input == *.* ]]; then
    output="${input%.*}.yaml"
else
    output="${input}.yaml"
fi

rstu_options=("$@")

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
manifest_path="$script_dir/../Cargo.toml"
temporary_output=$(mktemp "${output}.tmp.XXXXXX")
trap 'rm -f -- "$temporary_output"' EXIT

cargo run --quiet --manifest-path "$manifest_path" --bin rstu -- \
    convert --debug "$input" --output yaml "${rstu_options[@]}" > "$temporary_output"
mv -- "$temporary_output" "$output"
