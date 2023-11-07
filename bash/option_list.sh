#!/usr/bin/env bash
set -euo pipefail

echo "$1/${2/:/\/}" >> /tmp/aaa
cat "$1/${2/:/\/}" | \
    sed -n '/^SYNOPSIS$/,/^OPTIONS$/p' | \
    grep -v '^SYNOPSIS$' | \
    grep -v '^OPTIONS$' | \
    grep -v '^\s*$' | \
    sed 's/^\s\+//' | \
    bat --language css --color always --plain | \
    cat
