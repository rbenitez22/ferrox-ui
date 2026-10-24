#!/usr/bin/env bash
# install: puts ferrox-ui's agent kit into an app, where its agent finds it.
# install/targets.tsv says what goes where; install.spec.md is the full
# behaviour (install.ps1 is the same in PowerShell).
#
# Usage: install.sh --generic [--dry-run] <app folder>
# Exit:  0 installed (or, with --dry-run, shown), 2 usage error

set -euo pipefail

kit="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# Where the kit goes in the app; {{KIT}} in its Markdown files becomes this.
kit_dir=".ferrox"

usage() {
    echo "usage: install.sh --generic [--dry-run] <app folder>" >&2
    exit 2
}

adapter=""
dry=no
app=""
for arg in "$@"; do
    case "$arg" in
        --generic) adapter=generic ;;
        --dry-run) dry=yes ;;
        -*) usage ;;
        *) [[ -z "$app" ]] || usage; app="$arg" ;;
    esac
done
[[ -n "$adapter" && -n "$app" && -d "$app" ]] || usage
version="$(tr -d '\r\n' < "$kit/VERSION")"

# The plan: "<source file><US><destination>", in targets order, a folder's
# files in bytewise order of their path inside it.
plan=()
while IFS=$'\x1f' read -r row_adapter source dest; do
    [[ -z "$row_adapter" || "$row_adapter" == \#* || "$row_adapter" != "$adapter" ]] && continue
    if [[ -d "$kit/$source" ]]; then
        while IFS= read -r file; do
            plan+=("$kit/$source/$file"$'\x1f'"$dest/$file")
        done < <(cd "$kit/$source" && find . -type f -not -path './tests/*' | sed 's|^\./||' | LC_ALL=C sort)
    else
        plan+=("$kit/$source"$'\x1f'"$dest")
    fi
done < <(tr '\t' '\037' < "$kit/install/targets.tsv")

# A kit already there is kept, as the first free .ferrox.bak, .ferrox.bak.2, …
if [[ -e "$app/$kit_dir" ]]; then
    backup="$kit_dir.bak"
    n=2
    while [[ -e "$app/$backup" ]]; do
        backup="$kit_dir.bak.$n"
        n=$((n + 1))
    done
    if [[ "$dry" == yes ]]; then
        echo "would back up $kit_dir to $backup"
    else
        mv "$app/$kit_dir" "$app/$backup"
        echo "backed up $kit_dir to $backup"
    fi
fi

for entry in "${plan[@]}"; do
    source="${entry%%$'\x1f'*}"
    dest="${entry#*$'\x1f'}"
    if [[ "$dry" == yes ]]; then
        echo "would write $dest"
        continue
    fi
    mkdir -p "$(dirname "$app/$dest")"
    if [[ "$dest" == *.md ]]; then
        sed "s|{{KIT}}|$kit_dir|g" "$source" > "$app/$dest"
    else
        cp -p "$source" "$app/$dest"
    fi
    echo "wrote $dest"
done

if [[ "$dry" == yes ]]; then
    echo "ferrox-kit: dry run, nothing written"
else
    echo "ferrox-kit: installed $version ($adapter) into $kit_dir"
fi
