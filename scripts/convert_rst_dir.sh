#!/bin/bash

if [ "$#" -eq 0 ]; then
    echo "Usage: $0 <directory|file|pattern>..."
    exit 1
fi

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
shopt -s nullglob
files=()

for path in "$@"; do
    if [ -d "$path" ]; then
        directory_files=("$path"/*.rst)
        if [ ${#directory_files[@]} -eq 0 ]; then
            echo "No .rst files found in $path."
        fi
        files+=("${directory_files[@]}")
    elif [ -f "$path" ]; then
        files+=("$path")
    else
        saved_ifs=$IFS
        IFS=
        matches=( $path )
        IFS=$saved_ifs
        if [ ${#matches[@]} -eq 0 ]; then
            echo "Error: no files match $path." >&2
            exit 1
        fi
        files+=("${matches[@]}")
    fi
done

for file in "${files[@]}"; do
    # Call rst2yaml.sh for each .rst file
    if bash "$SCRIPT_DIR/rst2yaml.sh" "$file" "--prefer-block-scalars=true"; then
        echo "Successfully converted: $file"
    else
        echo "Error converting: $file"
    fi
done
