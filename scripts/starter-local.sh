#!/usr/bin/env bash
# Builds or serves the starter against this working tree. starter/ depends on
# ferrox-ui by git (as a copied app does); this copies it into
# target/starter-local, points its ferrox dependencies at the crates here, and
# runs Trunk there. starter/ itself is never changed.
#
# Usage: scripts/starter-local.sh build
#        scripts/starter-local.sh serve [--port N]

set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
copy="$root/target/starter-local"

mkdir -p "$copy"
# Fresh sources each time; the copy's own build output stays, for speed.
rm -rf "$copy/src" "$copy/Cargo.toml" "$copy/Cargo.lock"
cp -r "$root/starter/src" "$root/starter/build.rs" "$root/starter/Trunk.toml" "$root/starter/index.html" "$root/starter/Cargo.toml" "$copy/"

# `ferrox-x = { git = "…ferrox-ui", rev = "…", …` → `ferrox-x = { path = "<root>/ferrox-x", …`
for crate in ferrox-ui ferrox-theme ferrox-icons ferrox-theme-build; do
    sed -i -E "s#^(${crate} +=) \{ git = \"[^\"]+\", rev = \"[^\"]+\"#\1 { path = \"${root}/${crate}\"#" "$copy/Cargo.toml"
done
# The copy sits inside this repo's workspace folder: make it its own root.
printf '\n[workspace]\n' >> "$copy/Cargo.toml"
if grep -q 'git = ' "$copy/Cargo.toml"; then
    echo "starter-local: a ferrox dependency in starter/Cargo.toml isn't in the expected form" >&2
    exit 1
fi

cd "$copy"
export CARGO_TARGET_DIR="$root/target"
case "${1:-build}" in
    build) trunk build ;;
    serve) shift; trunk serve --no-autoreload "$@" ;;
    *) echo "usage: $0 build|serve [--port N]" >&2; exit 2 ;;
esac
