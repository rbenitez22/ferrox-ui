# install: the specification

`install.sh` (bash) and `install.ps1` (PowerShell 7) implement this; a
test runs both and requires the same output and the same files. To install
from another language, give this file and `targets.tsv` to your agent and
ask for the same program.

## What it does

Copies ferrox-ui's agent kit into an app, where the app's agent will find
it: the kit's guidance, its scripts and its list of pieces go into
`<app>/.ferrox/`, with the kit's placeholder paths filled in.

## Invocation

```
install --generic [--dry-run] <app folder>
```

- `--generic`: install the agent-neutral kit (the only adapter so far).
- `--dry-run`: say what would happen; change nothing.
- `<app folder>`: an existing folder (the app's root).
- Anything else, a missing adapter flag, no folder, more than one folder,
  or a folder that doesn't exist: print
  `usage: install.<ext> --generic [--dry-run] <app folder>` to standard
  error, exit 2.

The kit is the folder above the script's own (`install/..`). The **kit
folder** in the app is `.ferrox`.

## What goes where (`targets.tsv`)

UTF-8 text, one row per line; lines that are empty or start with `#` are
skipped. Each row has three tab-separated fields: the adapter, a source (a
path in the kit), a destination (a path in the app). Only the rows of the
chosen adapter count, in file order.

The **plan** is a list of (source file, destination) pairs, built row by
row:

- A source that is a folder contributes each file under it, at any depth,
  except under its own `tests/` folder: for a file at relative path `p`
  (parts joined with `/`), the pair is (source/`p`, destination/`p`). A
  folder's files are taken in bytewise (ordinal) order of `p`.
- A source that is a file contributes (source, destination).

## Steps

1. If `<app>/.ferrox` exists, it is kept as a backup: the first of
   `.ferrox.bak`, `.ferrox.bak.2`, `.ferrox.bak.3`, … that doesn't exist
   in `<app>`. Rename it there and print `backed up .ferrox to <name>`
   (with `--dry-run`: print `would back up .ferrox to <name>` and don't
   rename).
2. For each pair in the plan, in order: with `--dry-run`, print
   `would write <destination>`. Otherwise create the destination's folders,
   then:
   - a destination ending in `.md`: write the source's text with every
     `{{KIT}}` replaced by `.ferrox` (UTF-8, no byte-order mark);
   - any other file: copy it as it is, keeping its permissions (the
     scripts stay executable);

   and print `wrote <destination>`.
3. Print `ferrox-kit: dry run, nothing written` (with `--dry-run`), or
   `ferrox-kit: installed <version> (<adapter>) into .ferrox`, where
   `<version>` is the kit's `VERSION` file without its line ending.

Everything is printed to standard output, one line each. Exit 0.
