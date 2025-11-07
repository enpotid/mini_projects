#!/usr/bin/env bash

set -euo pipefail

if [ $# -lt 1 ]; then
  exit 2
fi

proj="$1"
shift

cargo_args=()
if [ $# -gt 0 ]; then
  if [ "$1" = "--" ]; then
    shift
  fi
  cargo_args=("$@")
fi

if [ ! -d "$proj" ]; then
  exit 3
fi

if [ ! -f "$proj/Cargo.toml" ]; then
  exit 4
fi

proj="$(cd "$proj" 2>/dev/null && pwd)"

(
  cd "$proj"
  cargo run "${cargo_args[@]}"
)
