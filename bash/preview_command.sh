#!/usr/bin/env bash
set -euo pipefail

target_file="$1/${2/:/\/}"
if [[ ! -s "${target_file}" ]]; then
    echo ""
    mkdir -p $(dirname "${target_file}")
    aws ${2/:/ } help | fzf --ansi -f ^ > "${target_file}"
fi
cat "${target_file}" | \
    bat --language css --color always --plain
