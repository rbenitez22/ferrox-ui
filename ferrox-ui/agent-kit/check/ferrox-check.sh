#!/usr/bin/env bash
# ferrox-check: lists where an app drifts from ferrox-ui, with the fix for
# each. The rules are in rules.tsv; ferrox-check.spec.md is the full
# behaviour (ferrox-check.ps1 is the same check in PowerShell).
#
# Usage: ferrox-check.sh [folder]     (default: the current folder)
# Exit:  0 no drift, 1 drift found, 2 usage error

set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
rules="$here/rules.tsv"
root="${1:-.}"

if [[ $# -gt 1 || ! -d "$root" ]]; then
    echo "usage: ferrox-check.sh [folder]" >&2
    exit 2
fi

# The files checked: *.rs, *.css and *.html, outside build output, generated
# styles, dependencies and hidden folders. Paths relative to the folder,
# sorted bytewise.
mapfile -t files < <(
    cd "$root" && find . \
        \( -type d \( -name target -o -name dist -o -name node_modules -o -name '.?*' -o -path '*/style/ferrox' \) -prune \) \
        -o -type f \( -name '*.rs' -o -name '*.css' -o -name '*.html' \) -print \
        | sed 's|^\./||' | LC_ALL=C sort
)

# Whether a file is one a rule applies to: "*.rs" and the like.
applies() { [[ "$1" == *"${2#\*}" ]]; }

findings=()   # "file<TAB>line<TAB>rule-index<TAB>text", sorted at the end
missing=()    # "files: id: rule (fix: …)", in rules order
index=0
# Fields are split on a unit separator, not tabs: `read` would merge the
# tabs around an empty field (an empty `except`) and shift the columns.
while IFS=$'\x1f' read -r id kind glob pattern except rule fix; do
    [[ -z "$id" || "$id" == \#* ]] && continue
    index=$((index + 1))
    text="$id: $rule (fix: $fix)"
    if [[ "$kind" == "forbid" ]]; then
        for file in "${files[@]}"; do
            applies "$file" "$glob" || continue
            while IFS=: read -r line _; do
                [[ -z "$line" ]] && continue
                current="$(sed -n "${line}p" "$root/$file")"
                previous=""
                (( line > 1 )) && previous="$(sed -n "$((line - 1))p" "$root/$file")"
                [[ -n "$except" ]] && grep -qE -- "$except" <<<"$current" && continue
                [[ "$current$previous" == *"ferrox-check: allow $id"* ]] && continue
                findings+=("$file"$'\t'"$line"$'\t'"$index"$'\t'"$text")
            done < <(grep -nE -- "$pattern" "$root/$file" || true)
        done
    elif [[ -f "$root/Cargo.toml" ]]; then
        # Require rules check a whole app: only at its root.
        found=no
        some=no
        for file in "${files[@]}"; do
            applies "$file" "$glob" || continue
            some=yes
            if grep -qE -- "$pattern" "$root/$file"; then found=yes; break; fi
        done
        [[ "$some" == yes && "$found" == no ]] && missing+=("$glob: $text")
    fi
done < <(tr '\t' '\037' < "$rules")

count=$(( ${#findings[@]} + ${#missing[@]} ))
if (( ${#findings[@]} > 0 )); then
    printf '%s\n' "${findings[@]}" | LC_ALL=C sort -t $'\t' -k1,1 -k2,2n -k3,3n | while IFS=$'\t' read -r file line _ text; do
        printf '%s:%s: %s\n' "$file" "$line" "$text"
    done
fi
for m in "${missing[@]+"${missing[@]}"}"; do
    printf '%s\n' "$m"
done
if (( count == 0 )); then
    echo "ferrox-check: no drift"
    exit 0
fi
echo "ferrox-check: $count problem(s)"
exit 1
