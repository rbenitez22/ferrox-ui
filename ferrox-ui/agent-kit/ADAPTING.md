# Adapting the kit to an agent

The kit's guidance is plain Markdown in `core/`, written for any agent. An
**adapter** wraps it in one agent's own mechanisms (its project
instructions, its reusable procedures, its sub-agents, its commands) so
that agent finds and uses it without being told. This page is the kit's
contract and how to map it; hand it to your agent with the name of the
agent to adapt to, and it can write the adapter.

If an agent has no such mechanisms, it needs no adapter: paste
`BOOTSTRAP.md` into its instructions.

## The contract

`kit.tsv` lists every piece: its kind, its id, its file in the kit, and its
purpose (one line, written to be shown to an agent deciding what to use).
Wrap every piece it lists; when the kit gains a piece, the adapter gains
one more wrapper and nothing else changes.

| Kind | What it is | Map it to |
|---|---|---|
| `entry` | `core/AGENTS.md`: read first, before any UI work | the agent's always-loaded project instructions: include it, or point to it from there |
| `task` | a procedure for one job (`setup`, `screen`, `component`, `check`) | the agent's reusable procedures (a skill, a prompt file, a rule), one per task, named `ferrox-<id>`, described by its purpose |
| `role` | a brief for a second pair of eyes (`reviewer`) | the agent's sub-agents or separate sessions, given the brief as their instructions |

Optionally, also give each task a command (a slash command, a menu entry)
that starts it.

## Rules for an adapter

1. **Point, never copy.** A wrapper holds the agent's required boilerplate
   (a name, a description, front matter) and one instruction: "follow
   `<kit>/<path>`". The description is the piece's purpose from
   `kit.tsv`, word for word. Nothing of the guidance itself goes in the
   wrapper, so it can't drift from the kit.
2. **Paths are the installed kit's.** In the kit's files, `{{KIT}}` stands
   for the folder the kit is installed in (`.ferrox` in the app); the
   installer replaces it. A wrapper uses the same path.
3. **The scripts stay scripts.** The drift check and the docs locator are
   run, not read. If the agent's environment can't run bash or PowerShell,
   port them from `check/*.spec.md` and test the port with
   `check/tests/fixtures/`.
4. **Generated, not hand-written, when you can.** Write one template per
   kind, and fill it from `kit.tsv`; the kit's own adapters
   (`adapters/`) work that way, through `install/targets.tsv`.

## Checklist

- [ ] The entry is always in the agent's context, or pointed to from what
      is.
- [ ] Every `task` row has a wrapper named `ferrox-<id>` whose description
      is its purpose and whose body points to its file.
- [ ] Every `role` row has a sub-agent or a documented way to start one.
- [ ] No wrapper contains guidance of its own.
- [ ] In a fresh session, asking the agent to "add a list page of things"
      makes it read `AGENTS.md`, follow the `screen` task, and run the
      check before finishing.
