# ferrox-ui's agent kit

Everything an agent needs to build consistent apps with ferrox-ui, with as
few tokens as possible: an entry point, a procedure for each job, a
reviewer's brief, and two scripts, the drift check and the docs locator.
The guidance is plain Markdown for any agent; installing the kit into an
app puts it where that app's agent finds it.

## Install it into an app

```sh
<ferrox-ui>/ferrox-ui/agent-kit/install/install.sh --generic path/to/app
```

(`install.ps1` on Windows, with the same arguments; `--dry-run` shows what
it would do.) It copies the kit into `path/to/app/.ferrox/`, with paths
filled in, and backs up a `.ferrox/` already there. Then paste
`.ferrox/BOOTSTRAP.md`'s paragraph into your agent's instructions for the
project. Adapters for particular agents (installed with their own flag)
are on their way; until then, and for any agent without one, the paragraph
is all it takes.

## What's in it

<!-- rendered: kit -->
| Kind | Piece | File | What it's for |
|---|---|---|---|
| entry | `agents` | [`core/AGENTS.md`](core/AGENTS.md) | Read first, before any UI work: what ferrox-ui is, how to work with it, its rules, and which component fits which need |
| task | `setup` | [`core/tasks/setup.md`](core/tasks/setup.md) | Add ferrox-ui to a Leptos CSR app, or start a new app from the starter |
| task | `screen` | [`core/tasks/screen.md`](core/tasks/screen.md) | Build a screen (a list, a form, a detail page, settings, the app shell) from a recipe |
| task | `component` | [`core/tasks/component.md`](core/tasks/component.md) | Find the right component for a need and use it correctly |
| task | `check` | [`core/tasks/check.md`](core/tasks/check.md) | Run the drift check on the app and fix what it finds |
| role | `reviewer` | [`core/roles/reviewer.md`](core/roles/reviewer.md) | Review UI changes against ferrox-ui's rules and report file:line findings, changing nothing |
<!-- /rendered -->

And beside them:

| Folder or file | What it's for |
|---|---|
| `check/` | `ferrox-check` (the drift check) and `ferrox-docs` (prints the docs of the ferrox-ui the app builds against), in bash and PowerShell 7, with `rules.tsv` (the check's rules, as data) |
| `BOOTSTRAP.md` | the paragraph that makes any agent use the kit |
| `ADAPTING.md` | the kit's contract, for writing an adapter for an agent the kit doesn't know |
| `install/` | the installer, in bash and PowerShell 7, and `targets.tsv` (what goes where) |
| `kit.tsv` | the list of pieces above: the one place an adapter or the installer reads them from |

## Scripts in other languages

Every script has a specification beside it (`*.spec.md`): give it to your
agent with the language you want, and test the port against
`check/tests/fixtures/`.

## Tested

`cargo test -p ferrox-ui --all-features`: the bash and PowerShell versions
of every script give the same, specified output on the same fixtures; the
rendered blocks here and in `core/AGENTS.md` match their sources; and
`core/` and `BOOTSTRAP.md` name no particular agent's tools or files.
