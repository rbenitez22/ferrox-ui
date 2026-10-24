# ferrox-ui's agent kit

Everything an agent needs to build consistent apps with ferrox-ui, with as
few tokens as possible: an entry point, a procedure for each job, a
reviewer's brief, and two scripts, the drift check and the docs locator.
The guidance is plain Markdown for any agent; installing the kit into an
app puts it where that app's agent finds it.

## Install it into an app

```sh
<ferrox-ui>/ferrox-ui/agent-kit/install/install.sh --claude path/to/app
```

(`install.ps1` on Windows, with the same arguments; bash 4 or later
elsewhere.) It copies the kit into `path/to/app/.ferrox/`, with paths
filled in (a `.ferrox/` already there is kept as `.ferrox.bak`), and
writes small wrappers where the agent looks:

| Flag | For | Writes |
|---|---|---|
| `--claude` | Claude Code | a skill per task (`.claude/skills/ferrox-<task>/`), a command per task (`/ferrox-<task>`), the reviewer as a sub-agent (`.claude/agents/`), and a pointer block in `CLAUDE.md` |
| `--copilot` | GitHub Copilot | a prompt file per task and role (`.github/prompts/`), and a pointer block in `.github/copilot-instructions.md` |
| `--agents-md` | agents that read `AGENTS.md` (Codex, Cursor, Copilot and others) | a block in `AGENTS.md` listing every piece |
| `--generic` | any other agent | nothing more: paste `.ferrox/BOOTSTRAP.md`'s paragraph into its instructions |

Flags combine (`--claude --agents-md`). A block in a file of yours
(`CLAUDE.md`, `AGENTS.md`) sits between `<!-- ferrox-kit … -->` markers;
the rest of the file is never touched, and a reinstall replaces the block.
`--update` refreshes an installed kit with the adapters it was installed
with (`.ferrox/INSTALLED`) and lists only what changed; `--dry-run` shows
what any of it would do. For an agent with no adapter here, `ADAPTING.md`
says how to write one, or just use `--generic`.

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
| `adapters/` | each agent's templates: its boilerplate and placeholders, filled from `kit.tsv` on install (no guidance of their own) |
| `install/` | the installer, in bash and PowerShell 7, and `targets.tsv` (what goes where, and how) |
| `kit.tsv` | the list of pieces above: the one place an adapter or the installer reads them from |

## Scripts in other languages

Every script has a specification beside it (`*.spec.md`): give it to your
agent with the language you want, and test the port against
`check/tests/fixtures/`.

## Tested

`cargo test -p ferrox-ui --all-features`: the bash and PowerShell versions
of every script give the same, specified output on the same fixtures; the
rendered blocks here and in `core/AGENTS.md` match their sources;
`core/` and `BOOTSTRAP.md` name no particular agent's tools or files; every
adapter wraps every piece; and the templates carry no guidance.
