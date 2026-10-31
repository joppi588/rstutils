#!/bin/bash

# Check if a directory is provided as an argument
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <directory>"
    exit 1
fi

DIRECTORY=$1
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)

# Check if the provided argument is a directory
if [ ! -d "$DIRECTORY" ]; then
    echo "Error: $DIRECTORY is not a directory."
    exit 1
fi

# Iterate through all .rst files in the directory and its subdirectories
while IFS= read -r -d '' file; do
    # Check if there are no .rst files
    if [ ! -e "$file" ]; then
        echo "No .rst files found in $DIRECTORY."
        exit 0
    fi

    # Call rst2yaml.sh for each .rst file
    if bash "$SCRIPT_DIR/rst2yaml.sh" "$file" "--prefer-block-scalars=false"; then
        echo "Successfully converted: $file"
    else
        echo "Error converting: $file"
    fi
done < <(find "$DIRECTORY" -type f -name '*.rst' -print0)
