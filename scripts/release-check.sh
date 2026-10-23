#!/usr/bin/env bash
# The release gate: run before tagging a release. Stops at the first failing step.
#
#   1. every crate builds with each of its features alone (and with none)
#   2. clippy, warnings are errors
#   3. cargo test --workspace
#   4. the demo builds with Trunk (from scratch: generated CSS removed first)
#   5. Playwright e2e tests, Chromium + Firefox, against a fresh build of the
#      working tree on its own port (never a running `trunk serve`)
#
# Usage: scripts/release-check.sh

set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"

step() { printf '\n\033[1m== %s\033[0m\n' "$1"; }

# Library crates and their features (the demo is not a library).
crates=(ferrox-theme-build ferrox-theme ferrox-ui ferrox-icons)

step "1/5 features build alone"
if ! cargo hack --version >/dev/null 2>&1; then
    echo "cargo-hack is required: cargo install cargo-hack" >&2
    exit 1
fi
# Switch to --feature-powerset once ferrox-ui has two or more groups.
for crate in "${crates[@]}"; do
    cargo hack check -p "$crate" --each-feature --no-dev-deps
done

step "2/5 clippy"
cargo clippy -q --workspace --all-targets --all-features -- -D warnings

step "3/5 cargo test"
cargo test -q --workspace --all-features

step "4/5 demo build"
# From scratch: the generated stylesheet must appear on a clean checkout.
# Own dist folder: demo/dist belongs to a running `trunk serve`.
rm -rf demo/style/ferrox
(cd demo && trunk build --dist ../target/release-check-dist)

step "5/5 e2e tests"
(
    cd demo/e2e
    [[ -d node_modules ]] || npm ci
    E2E_FRESH=1 npx playwright test
)

printf '\n\033[1;32mrelease check passed\033[0m\n'
