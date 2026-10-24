# install: the specification

`install.sh` (bash) and `install.ps1` (PowerShell 7) implement this; a
test runs both and requires the same output and the same files. To install
from another language, give this file, `targets.tsv` and `../kit.tsv` to
your agent and ask for the same program.

## What it does

Installs ferrox-ui's agent kit into an app: the kit itself (its guidance,
scripts and list of pieces) into `<app>/.ferrox/`, and, for each agent
asked for, small wrappers in that agent's own places (its skills, prompt
files, project instructions) that point into `.ferrox/`.

## Invocation

```
install <adapter flags> [--dry-run] <app folder>
install --update [<adapter flags>] [--dry-run] <app folder>
```

- Adapter flags: `--generic` (the kit alone), `--claude`, `--copilot`,
  `--agents-md`; any number of them. The generic kit is always installed:
  every other adapter points into it.
- `--update`: refresh a kit installed before. Without adapter flags, the
  adapters it was installed with (`.ferrox/INSTALLED`) are used again.
- `--dry-run`: say what would happen; change nothing.
- `<app folder>`: an existing folder (the app's root).
- Print `usage: install.<ext> <--generic|--claude|--copilot|--agents-md>...
  [--update] [--dry-run] <app folder>` to standard error and exit 2 when:
  an argument starting with `-` isn't one of these; there's no folder or
  more than one; the folder doesn't exist; there's no adapter flag and no
  `--update`; or `--update` has no flags and there's no
  `<app>/.ferrox/INSTALLED`.

The kit is the folder above the script's own (`install/..`). The **kit
folder** in the app is `.ferrox`. The kit's version is its `VERSION` file,
without its line ending.

## The adapters chosen

`generic`, then each other adapter asked for (or read from `INSTALLED`, one
name per line), in the order they first appear in `targets.tsv`, each once.

## The kit's pieces (`kit.tsv`)

Lines that are empty or start with `#` are skipped; each other line has
four tab-separated fields: kind, id, path, purpose. Rows keep their file
order.

## The targets (`targets.tsv`)

Lines that are empty or start with `#` are skipped; each other line has
four tab-separated fields: adapter, source (a path in the kit), destination
(a path in the app), how. Only the rows of the chosen adapters count, in
file order.

**Rendering** a template for a `kit.tsv` row replaces, everywhere in the
text, `{{ID}}` with its id, `{{KIND}}` its kind, `{{PATH}}` its path,
`{{PURPOSE}}` its purpose, and `{{KIT}}` with `.ferrox`, as plain text (no
pattern syntax).

Each target row adds to the **plan**, in order:

- `copy`: if the source is a folder, each file under it at any depth
  except under its own `tests/`, in bytewise order of its relative path
  `p` (parts joined with `/`), to destination/`p`; if a file, to the
  destination. Content: a destination ending in `.md` gets every `{{KIT}}`
  replaced with `.ferrox`; any other file is copied as it is, keeping its
  permissions.
- `each:<kinds>` (kinds comma-separated): for each `kit.tsv` row whose kind
  is one of them, in `kit.tsv` order, a file at the destination with its
  `{{ID}}` replaced by the row's id; content: the source template,
  rendered for the row.
- `block:<kinds>`: one **block** in the destination file: the line
  `<!-- ferrox-kit: written by ferrox-ui's agent kit; install --update rewrites it -->`,
  then the source template rendered for each `kit.tsv` row of those kinds
  (in order, each keeping the template's line ending), then the line
  `<!-- /ferrox-kit -->`.

Then the plan's last file: `.ferrox/INSTALLED`, the chosen adapters, one
per line, each line ending with `\n`.

Files are written as UTF-8 without a byte-order mark; text is written as
read (line endings included).

## Installing (no `--update`)

1. If `<app>/.ferrox` exists, keep it as a backup: the first of
   `.ferrox.bak`, `.ferrox.bak.2`, `.ferrox.bak.3`, … that doesn't exist in
   `<app>`. Rename it there and print `backed up .ferrox to <name>`
   (`--dry-run`: print `would back up .ferrox to <name>`).
2. For each entry of the plan, in order:
   - a file: create its folders, write it (replacing any file there), print
     `wrote <destination>` (`--dry-run`: `would write <destination>`);
   - a block: the destination's new text is its old text (empty if it
     doesn't exist) with the block put in: if the old text holds a line
     starting `<!-- ferrox-kit` and, later, the line `<!-- /ferrox-kit -->`,
     everything from the start of the first to the end of the second is
     replaced by the block; otherwise the block is added at the end, after
     a blank line if the old text isn't empty (and after a line ending if
     the old text doesn't end with one), and followed by a line ending.
     Write it; print `wrote block in <destination>` (`--dry-run`:
     `would write block in <destination>`).
3. Print `ferrox-kit: installed <version> (<adapters, joined with ", ">) into .ferrox`
   (`--dry-run`: `ferrox-kit: dry run, nothing written`).

## Updating (`--update`)

No backup. For each entry of the plan, in order, compute its new content
(for a block, the destination's whole new text) and compare it with what's
there:

- missing: print `added <destination>` and write it;
- different: print `changed <destination>` and write it;
- the same: print nothing, write nothing.

Then every file under `<app>/.ferrox` that isn't a destination in the plan,
in bytewise order of its path: print `removed <path>` and delete it.

Then print `ferrox-kit: updated to <version> (<adapters>); <n> change(s)`,
where `<n>` counts the lines printed before it. With `--dry-run`, print
`would add`, `would change`, `would remove` instead, write and delete
nothing, and end with `ferrox-kit: dry run, <n> change(s), nothing written`.

## Output and exit

Everything goes to standard output, one line each, as above. Exit 0
(or 2 for a usage error).
