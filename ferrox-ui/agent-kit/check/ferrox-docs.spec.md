# ferrox-docs: the specification

`ferrox-docs.sh` (bash) and `ferrox-docs.ps1` (PowerShell 7) implement
this; a test runs both. To use it from another language, give this file to
your agent and ask for the same program in that language.

## What it does

Prints the folder holding the docs (`docs/`) of the ferrox-ui an app
actually builds against: a local clone, a git checkout in Cargo's cache, or
a crates.io download. So an agent reads the docs for the version it
compiles, never another.

## Invocation

```
ferrox-docs [folder]
```

- `folder`: the app's folder (where `cargo` finds its `Cargo.toml`).
  Default: the current folder.
- More than one argument, or a folder that doesn't exist: print
  `usage: ferrox-docs.<ext> [folder]` to standard error, exit 2.

## Steps

1. In `folder`, run `cargo metadata --format-version 1` (it may fetch
   dependencies the first time). If it fails: print
   `ferrox-docs: cargo metadata failed in <folder>` to standard error,
   exit 2.
2. In its JSON output, find the package named `ferrox-ui` in `packages`
   and take the folder of its `manifest_path` (that folder's name is
   `ferrox-ui`, or `ferrox-ui-<version>` from crates.io). A program
   without a JSON parser may search the text for the first
   `"manifest_path":"…/ferrox-ui/Cargo.toml"` (or
   `…/ferrox-ui-<version>/Cargo.toml`) instead.
3. If there's no such package, or `<that folder>/docs/index.md` doesn't
   exist: print `ferrox-docs: ferrox-ui isn't a dependency in <folder>` to
   standard error, exit 1.
4. Print `<that folder>/docs` to standard output, exit 0.
