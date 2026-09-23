#!/usr/bin/env bash
set -euo pipefail
shopt -s nullglob

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
EXAMPLES_DIR="${REPO_ROOT}/codesamples/examples"
SCREENSHOT_DIR="${REPO_ROOT}/src/screenshots"

mkdir -p "${SCREENSHOT_DIR}"
cd "${REPO_ROOT}/codesamples"

for example_file in "${EXAMPLES_DIR}"/*.rs; do
    example_name="$(basename "${example_file}" .rs)"

    if ! grep -q '#\[turtle_main' "${example_file}"; then
        continue
    fi

    if grep -q '^[[:space:]]*use dialog::' "${example_file}"; then
        echo "Skipping because of interactive dialog" >&2
        continue
    fi

    output_path="${SCREENSHOT_DIR}/${example_name}.svg"
    echo "Exporting ${example_name} -> ${output_path}"
    cargo run --features svg-export --example "${example_name}" -- --export-svg "${output_path}"
done
