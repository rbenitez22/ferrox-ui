#!/usr/bin/env bash
# ferrox-docs: prints the docs folder of the ferrox-ui an app builds against,
# so an agent reads the docs for the version it compiles. Asks Cargo
# (`cargo metadata`); ferrox-docs.spec.md is the full behaviour
# (ferrox-docs.ps1 is the same in PowerShell).
#
# Usage: ferrox-docs.sh [folder]      (the app's folder; default: the current)
# Exit:  0 printed, 1 ferrox-ui isn't a dependency there, 2 usage or Cargo error

set -euo pipefail

root="${1:-.}"
if [[ $# -gt 1 || ! -d "$root" ]]; then
    echo "usage: ferrox-docs.sh [folder]" >&2
    exit 2
fi

if ! meta="$(cd "$root" && cargo metadata --format-version 1 2>/dev/null)"; then
    echo "ferrox-docs: cargo metadata failed in $root" >&2
    exit 2
fi

# The ferrox-ui package's manifest: its folder is ferrox-ui (a clone or a
# git checkout) or ferrox-ui-<version> (from crates.io).
manifest="$(printf '%s' "$meta" | grep -oE '"manifest_path":"[^"]*/ferrox-ui(-[0-9][^/"]*)?/Cargo\.toml"' | head -n 1 || true)"
dir="${manifest#\"manifest_path\":\"}"
dir="${dir%/Cargo.toml\"}"

if [[ -z "$manifest" || ! -f "$dir/docs/index.md" ]]; then
    echo "ferrox-docs: ferrox-ui isn't a dependency in $root" >&2
    exit 1
fi
echo "$dir/docs"
