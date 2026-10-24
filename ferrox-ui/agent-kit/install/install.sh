#!/usr/bin/env bash
# install: puts ferrox-ui's agent kit into an app, and wrappers for the
# agents asked for. install/targets.tsv says what goes where;
# install.spec.md is the full behaviour (install.ps1 is the same in
# PowerShell). Needs bash 4 or later.
#
# Usage: install.sh <--generic|--claude|--copilot|--agents-md>... [--dry-run] <app folder>
#        install.sh --update [adapter flags] [--dry-run] <app folder>
# Exit:  0 done (or, with --dry-run, shown), 2 usage error

set -euo pipefail
# In bash 5.2, `&` in a ${var//pattern/replacement} means the match; here it
# must be a plain `&`.
shopt -u patsub_replacement 2>/dev/null || true

kit="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# Where the kit goes in the app; {{KIT}} becomes this.
kit_dir=".ferrox"
us=$'\x1f'

usage() {
    echo "usage: install.sh <--generic|--claude|--copilot|--agents-md>... [--update] [--dry-run] <app folder>" >&2
    exit 2
}

flags=()
update=no
dry=no
app=""
for arg in "$@"; do
    case "$arg" in
        --generic | --claude | --copilot | --agents-md) flags+=("${arg#--}") ;;
        --update) update=yes ;;
        --dry-run) dry=yes ;;
        -*) usage ;;
        *) [[ -z "$app" ]] || usage; app="$arg" ;;
    esac
done
[[ -n "$app" && -d "$app" ]] || usage
if (( ${#flags[@]} == 0 )); then
    [[ "$update" == yes && -f "$app/$kit_dir/INSTALLED" ]] || usage
    mapfile -t flags < "$app/$kit_dir/INSTALLED"
fi
version="$(tr -d '\r\n' < "$kit/VERSION")"

# A file's exact text, trailing line endings included, in REPLY.
read_exact() {
    REPLY="$(cat -- "$1"; printf x)"
    REPLY="${REPLY%x}"
}

# Rows of a tab-separated file (comments and blank lines skipped), with the
# fields split on a unit separator so empty fields survive.
rows() { grep -v -e '^#' -e '^$' "$1" | tr '\t' '\037'; }

# The kit's pieces.
k_kind=() k_id=() k_path=() k_purpose=()
while IFS="$us" read -r kind id path purpose; do
    k_kind+=("$kind") k_id+=("$id") k_path+=("$path") k_purpose+=("$purpose")
done < <(rows "$kit/kit.tsv")

# The targets, and the adapters chosen: generic, then the others asked for,
# in the order they first appear in targets.tsv.
t_adapter=() t_source=() t_dest=() t_how=()
chosen=(generic)
while IFS="$us" read -r adapter source dest how; do
    t_adapter+=("$adapter") t_source+=("$source") t_dest+=("$dest") t_how+=("$how")
    if [[ " ${flags[*]} " == *" $adapter "* && " ${chosen[*]} " != *" $adapter "* ]]; then
        chosen+=("$adapter")
    fi
done < <(rows "$kit/install/targets.tsv")

# A template rendered for kit row $2, in REPLY.
render() {
    local text="$1" i="$2"
    text="${text//\{\{ID\}\}/${k_id[i]}}"
    text="${text//\{\{KIND\}\}/${k_kind[i]}}"
    text="${text//\{\{PATH\}\}/${k_path[i]}}"
    text="${text//\{\{PURPOSE\}\}/${k_purpose[i]}}"
    REPLY="${text//\{\{KIT\}\}/$kit_dir}"
}

# The plan: what (raw: copy the source; text: write p_text; block: put
# p_text into the file as a block), where, and from what.
p_what=() p_dest=() p_source=() p_text=()
for t in "${!t_adapter[@]}"; do
    [[ " ${chosen[*]} " == *" ${t_adapter[t]} "* ]] || continue
    source="$kit/${t_source[t]}" dest="${t_dest[t]}" how="${t_how[t]}"
    case "$how" in
        copy)
            if [[ -d "$source" ]]; then
                list="$(cd "$source" && find . -type f -not -path './tests/*' | sed 's|^\./||' | LC_ALL=C sort)"
            else
                list=""
            fi
            while IFS= read -r file; do
                from="$source${file:+/$file}" to="$dest${file:+/$file}"
                if [[ "$to" == *.md ]]; then
                    read_exact "$from"
                    p_what+=(text) p_dest+=("$to") p_source+=("$from") p_text+=("${REPLY//\{\{KIT\}\}/$kit_dir}")
                else
                    p_what+=(raw) p_dest+=("$to") p_source+=("$from") p_text+=("")
                fi
            done <<< "$list"
            ;;
        each:*)
            read_exact "$source"
            template="$REPLY"
            for i in "${!k_kind[@]}"; do
                [[ ",${how#each:}," == *",${k_kind[i]},"* ]] || continue
                render "$template" "$i"
                p_what+=(text) p_dest+=("${dest//\{\{ID\}\}/${k_id[i]}}") p_source+=("") p_text+=("$REPLY")
            done
            ;;
        block:*)
            read_exact "$source"
            template="$REPLY"
            block="<!-- ferrox-kit: written by ferrox-ui's agent kit; install --update rewrites it -->"$'\n'
            for i in "${!k_kind[@]}"; do
                [[ ",${how#block:}," == *",${k_kind[i]},"* ]] || continue
                render "$template" "$i"
                block+="$REPLY"
            done
            block+="<!-- /ferrox-kit -->"
            p_what+=(block) p_dest+=("$dest") p_source+=("") p_text+=("$block")
            ;;
    esac
done
p_what+=(text) p_dest+=("$kit_dir/INSTALLED") p_source+=("") p_text+=("$(printf '%s\n' "${chosen[@]}")"$'\n')

# A block put into a file's text: replacing the old block, or added at the end.
with_block() {
    local old="$1" block="$2" start="<!-- ferrox-kit" end="<!-- /ferrox-kit -->" before after
    if [[ "$old" == "$start"* ]]; then
        before=""
        after="${old#"$start"}"
    elif [[ "$old" == *$'\n'"$start"* ]]; then
        before="${old%%$'\n'"$start"*}"$'\n'
        after="${old#*$'\n'"$start"}"
    else
        after=""
    fi
    if [[ -n "${before+set}" && "$after" == *"$end"* ]]; then
        REPLY="$before$block${after#*"$end"}"
    elif [[ -z "$old" ]]; then
        REPLY="$block"$'\n'
    else
        [[ "$old" == *$'\n' ]] || old+=$'\n'
        REPLY="$old"$'\n'"$block"$'\n'
    fi
}

# Entry k's new content, in REPLY (a raw copy's is its source's).
new_content() {
    local k="$1"
    case "${p_what[k]}" in
        raw) read_exact "${p_source[k]}" ;;
        text) REPLY="${p_text[k]}" ;;
        block)
            local old=""
            if [[ -f "$app/${p_dest[k]}" ]]; then read_exact "$app/${p_dest[k]}"; old="$REPLY"; fi
            with_block "$old" "${p_text[k]}"
            ;;
    esac
}

# Writes entry k (a raw copy keeps its source's permissions).
write_entry() {
    local k="$1" target="$app/${p_dest[k]}"
    mkdir -p "$(dirname "$target")"
    if [[ "${p_what[k]}" == raw ]]; then
        cp -p "${p_source[k]}" "$target"
    else
        new_content "$k"
        printf '%s' "$REPLY" > "$target"
    fi
}

adapters="$(IFS=,; echo "${chosen[*]}")"
adapters="${adapters//,/, }"

if [[ "$update" == no ]]; then
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
    for k in "${!p_what[@]}"; do
        what="wrote"
        [[ "$dry" == yes ]] && what="would write"
        [[ "${p_what[k]}" == block ]] && what="$what block in"
        [[ "$dry" == yes ]] || write_entry "$k"
        echo "$what ${p_dest[k]}"
    done
    if [[ "$dry" == yes ]]; then
        echo "ferrox-kit: dry run, nothing written"
    else
        echo "ferrox-kit: installed $version ($adapters) into $kit_dir"
    fi
    exit 0
fi

# --update: only what changes.
prefix=""
[[ "$dry" == yes ]] && prefix="would "
changes=0
declare -A planned=()
for k in "${!p_what[@]}"; do
    planned["${p_dest[k]}"]=1
    target="$app/${p_dest[k]}"
    new_content "$k"
    if [[ ! -f "$target" ]]; then
        echo "${prefix}add ${p_dest[k]}" | sed 's/^add/added/'
    else
        new="$REPLY"
        read_exact "$target"
        [[ "$REPLY" == "$new" ]] && continue
        echo "${prefix}change ${p_dest[k]}" | sed 's/^change/changed/'
    fi
    changes=$((changes + 1))
    [[ "$dry" == yes ]] || write_entry "$k"
done
if [[ -d "$app/$kit_dir" ]]; then
    while IFS= read -r file; do
        [[ -z "$file" || -n "${planned[$kit_dir/$file]+set}" ]] && continue
        echo "${prefix}remove $kit_dir/$file" | sed 's/^remove/removed/'
        changes=$((changes + 1))
        [[ "$dry" == yes ]] || rm -- "$app/$kit_dir/$file"
    done < <(cd "$app/$kit_dir" && find . -type f | sed 's|^\./||' | LC_ALL=C sort)
fi
if [[ "$dry" == yes ]]; then
    echo "ferrox-kit: dry run, $changes change(s), nothing written"
else
    echo "ferrox-kit: updated to $version ($adapters); $changes change(s)"
fi
