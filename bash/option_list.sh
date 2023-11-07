#!/usr/bin/env bash
set -euo pipefail

cat "$1/${2/:/\/}" | \
    sed -n '/^SYNOPSIS$/,/^OPTIONS$/p' | \
    grep -v '^SYNOPSIS$' | \
    grep -v '^OPTIONS$' | \
    grep -v '^\s*$' | \
    tail -n +2 | \
    sed 's/^\s\+//' | \
    bat --language css --color always --plain | \
    cat
