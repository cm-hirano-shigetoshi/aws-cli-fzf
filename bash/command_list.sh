#!/usr/bin/env bash
set -euo pipefail

(cd "$1" && grep '^\s*o [-a-z]\+$' *) | sed 's/^\(.*\?\):.*o \([-a-z]\+\)$/\1:\2/' | bat --language perl --plain --color always
