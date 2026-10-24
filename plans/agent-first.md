# Agent first: ferrox-ui for a developer working through an agent

Status: noted (2026-10-24), to plan and discuss after the menus group. Not
approved; nothing here is decided.

## The user's direction (2026-10-24)

The audience is not a human developer reading docs: it's a developer
working through an agent. The library must let an agent build a consistent
app with minimal token use. With ferrox-ui in place from the start, the
directive would have been just "use ferrox-ui", instead of repeating
instructions to keep this or that convention, look and feel.

So the question for the plan: what lets an agent understand the intent of
the library quickly, and arrive at consistent UIs, while spending as few
tokens as possible?

## First ideas, to discuss

- **A short entry point for agents** (an `AGENTS.md`, or a Claude Code
  skill, `ferrox-ui`): what the library is for, a "which component for
  which need" table, and the few rules that make an app consistent (no raw
  `<button>`, no colours or spacing outside the tokens, actions in a
  Panel's `actions`, …). Read first; small enough to read every time.
- **Split the reference so it loads on demand.** `ferrox-ui/README.md` is
  over 1,300 lines: too much to load to use one component. An index (one
  line per component: what it's for, its signature) plus a page per
  component the agent opens only when it needs it.
- **Recipes for whole screens**, the shapes apps repeat: a list page
  (Panel + DataView + Menu row actions + DataLayoutSwitch), a form in a
  Dialog, a detail page, a settings page. An agent copies a recipe instead
  of composing ten components from scratch. Perhaps some become
  components or webapp-lib adapters (`ListPage`).
- **Docs that can't rot:** examples compiled as doctests, and the index
  generated from (or checked against) the source, so the agent never
  trusts a stale signature.
- **Make the obvious call the right one:** defaults that are correct,
  types that refuse wrong combinations, compile errors that point to the
  fix. Every guess an agent doesn't have to make is tokens saved.
- **A consistency check an agent can run:** a script or lint over an app
  that flags drift (raw elements where a component exists, inline styles,
  hex colours, hand-made menus), so "is this consistent?" is one command,
  not a review.
- **Point the existing skills here:** `leptos-list-view`, `leptos-page`,
  `leptos-ui-setup` and `webapp-lib-ref` should produce ferrox-ui code and
  say so, once webapp-lib is on it.
- Measure: the tokens an agent spends building a reference screen (say, a
  list page) before and after, so changes are judged by the number.
