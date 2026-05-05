#!/usr/bin/env bash
set -euo pipefail

MIN_COVERAGE="${1:-60}"
LLVM_BIN="$(rustup run stable rustc --print target-libdir)/../bin"

LINES_PERCENT="$(LLVM_COV="$LLVM_BIN/llvm-cov" LLVM_PROFDATA="$LLVM_BIN/llvm-profdata" cargo +stable llvm-cov --workspace --json --summary-only | jq -r '.data[0].totals.lines.percent')"

if [[ -z "$LINES_PERCENT" || "$LINES_PERCENT" == "null" ]]; then
    echo "Failed to read line coverage from cargo llvm-cov output" >&2
    exit 2
fi

printf 'Line coverage: %.2f%% (required: %.2f%%)\n' "$LINES_PERCENT" "$MIN_COVERAGE"

awk -v pct="$LINES_PERCENT" -v min="$MIN_COVERAGE" 'BEGIN { exit ((pct + 0) >= (min + 0) ? 0 : 1) }'
