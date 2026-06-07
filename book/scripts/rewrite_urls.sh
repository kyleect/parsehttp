#!/usr/bin/env bash
set -euo pipefail

DOC_DIR="target/doc"

find "$DOC_DIR" -name "*.html" -type f | while read -r file; do
  sed -Ei \
    -e 's|(href|src)="/([^"]+)"|\1="/api/\2"|g' \
    "$file"
done
