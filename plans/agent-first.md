# Agent first: ferrox-ui for a developer working through an agent

Status: approved (2026-10-24), open points as recommended (`pwsh` installed here for the tests). Revised the same day before approval: the kit as the deliverable (the user's `ai-toolkit` untouched); every script in bash and PowerShell with a spec for other languages; an agent-agnostic core with generated adapters and a path for unknown agents; one source per fact; the kit inside the crate; a front door.

## The direction (the user, 2026-10-24)

The audience is not a human developer reading docs: it's a developer
working through an agent. The library must let an agent build a consistent
app with minimal token use. With ferrox-ui in place, an app's whole UI
directive should be one line, "use ferrox-ui", instead of conventions,
look and feel restated in every prompt and every `CLAUDE.md`.

Two measures, then: **consistency** (an agent's screens look and behave
like every other app's without being told how) and **tokens** (what it has
to read, write and fix to get there).

**The deliverable is an agent kit** (the user, 2026-10-24): ferrox-ui ships
its own toolkit (entry point, skills, a subagent, commands, the drift
check, an installer) that any developer attaches to, or adapts for, their
own environment and agent. The user's `ai-toolkit` is not changed: it's an
example of the shape, and its owner adopts the kit like anyone else. The
library stays usable by a human; the kit is what makes it cheap for an
agent.

## Where an agent stands today

- **The reference is one 1,671-line file** (`ferrox-ui/README.md`, 73 KB,
  roughly 18,000 tokens). Using one component means loading all of it, or
  searching it blind.
- **Nothing says which component fits a need**, except the new menus table.
  An agent picks by name, or writes its own.
- **Page layout is the app's CSS.** Components look right; the space
  between and around them is whatever the agent writes, so screens drift
  even when every component is ferrox-ui's.
- **Examples aren't compiled.** A renamed prop leaves the README wrong,
  and the agent pays for that in a compile-fix loop.
- **Nothing ships for agents.** The only agent guidance that exists is in
  the user's private `ai-toolkit` (`leptos-list-view`, `leptos-page`,
  `leptos-ui-setup`, `webapp-lib-ref`: about 580 lines, about webapp-lib
  0.x). Another developer gets nothing.
- **Nothing checks drift.** "Is this consistent?" is a review, not a
  command.

## Design

### 0. One source per fact

The user (2026-10-24): no duplicated content. Every agent, and every
human, taps into the same core, as the user's `ai-toolkit` was meant to.
The rule for every piece below:

- **Each fact has exactly one home.** Everything else links to it.
- **Where an agent needs a fact inline** (to save a file read), it is
  *rendered* from its home by a tool, between markers, and a test fails
  when the render is stale. Never copied by hand.
- **Adapters hold no content**, not even a description: they are
  generated from `kit.tsv` at install time.

| Fact | Its one home | Reached from |
|---|---|---|
| A component's props, example, behaviour | `ferrox-ui/docs/<component>.md` | rustdoc (`include_str!`), the README (link), agents (via the index) |
| Every public item, one line | the source's doc comments → `docs/index.md` (generated) | `AGENTS.md` (link), the `component` task |
| Which component for which need | `docs/choosing.md` | `AGENTS.md` (rendered in, it's the most-used fact), the README (link) |
| The consistency rules | `ferrox-ui/agent-kit/check/rules.tsv` (id, rule, pattern, fix) | the drift check (reads it), `AGENTS.md` (rules rendered in), the reviewer role (link) |
| A recipe | its source (`ferrox-ui/docs/recipes/*.rs`, compiled by the demo, e2e-tested) | `docs/recipes.md`: one line each, what it shows, its path |
| A task's procedure | `ferrox-ui/agent-kit/core/tasks/<task>.md` | adapters (generated from `kit.tsv`), `BOOTSTRAP.md` |
| A task's name and purpose | `ferrox-ui/agent-kit/kit.tsv` | every generated adapter, `ADAPTING.md`, the kit README's table (rendered) |
| A script's behaviour | its `.spec.md` (the master) | the bash and PowerShell implementations, which follow it; the shared test suite holds them to it |
| Where an adapter's files go | `ferrox-ui/agent-kit/install/targets.tsv` | both installers |

### 1. One entry point: `ferrox-ui/agent-kit/core/AGENTS.md`

Small enough to read every time (target under 2,000 tokens), and complete
enough that most screens need nothing else:

- What ferrox-ui is, in three lines; setup in five lines that point to the
  `setup` task for the details.
- **The "need → component" table for the whole library**, rendered from
  `docs/choosing.md` (its one home), the menus table's shape: "a list of records → `DataView`", "a destructive confirm
  → `ConfirmDialog`", "pick from many → `Combobox`", "row actions → `Menu`
  with `MORE_VERTICAL`", …
- **The rules that make an app consistent**, rendered from
  `check/rules.tsv` (the same rules the drift check enforces), each one
  line: no raw
  `<button>` / `<select>` / `<dialog>` / `<input>` where a component
  exists; no colours, sizes or spacing outside the tokens; page structure
  from the layout primitives (below), not app CSS; a screen's actions in
  its `Panel`'s `actions`; every icon from ferrox-icons; …
- **Where to look next**: the recipe for the screen at hand, then the one
  component page it needs.

It lives in the ferrox-ui repo, in the kit's agent-neutral core, so it
versions with the code. Each adapter, or `BOOTSTRAP.md`, points the
developer's agent at it (sections 8 and 8a).

### 2. A reference that loads on demand

- `ferrox-ui/docs/<component>.md`, one page per component (or per small
  family: `menu-items.md`): props, one example, the behaviour that matters.
  Most are 50–150 lines.
- `ferrox-ui/docs/index.md`: every public item, one line each, its
  signature and what it's for. **Generated from the source** (the
  `#[component]` signatures and their first doc line) by a small tool in
  the repo, with a test that fails when the committed index is stale.
- `ferrox-ui/docs/choosing.md`: which component for which need, the one
  home of that table (the menus table in today's README moves there).
- The README shrinks to a human overview with links. The content moves; it
  isn't written twice. The same render tool and staleness test serve
  every rendered block in the repo (sections 0, 1, 8).

### 3. Docs that can't rot

The component pages are pulled into rustdoc with `#[doc =
include_str!(…)]`, so their examples compile as doctests on every `cargo
test`. A renamed prop breaks the build, not the agent.

### 4. Recipes: whole screens, tested

The shapes apps repeat, each a complete, working screen:

| Recipe | Built from |
|---|---|
| List page | `Panel` (title, `+`, `DataLayoutSwitch`) + `DataView` of `Card`s + a row `Menu` + empty / loading / error |
| Form in a dialog | `Dialog` + `Field`s + validation + saving |
| Detail page | `Panel`s, a `SplitButton`, a `ConfirmDialog` for delete |
| Settings page | collapsible `Panel`s, `Switch`es, `Select`s |
| App shell | header, `SideMenu`, a user `Menu` with a header |

Each recipe is a demo page (`demo/src/recipes/<name>.rs`, so it runs and
the e2e suite covers it), commented where a choice needs explaining. The
page *is* the recipe: `docs/recipes.md` only lists them, one line each
with its path. An agent reads the one file and changes the nouns: few
tokens, and the same screen shape in every app.

### 5. Page layout primitives (`layout`)

So apps need next to no CSS of their own. Three small components on the
spacing tokens:

- `Stack` — children in a column with a token gap (`Gap::Sm/Md/Lg`).
- `Cluster` — children in a wrapping row with a gap and alignment (a row
  of buttons, a label and its value).
- `PageGrid` — a page's main column plus an optional side column that
  stacks under it on a phone.

With these, a recipe has no `<style>`, and neither does the app that
copies it.

### 6. Names an agent can predict

An agent generalises from one component to the next, so the same idea must
have the same name everywhere. An audit, then renames while it's cheap
(nothing is released): `label` vs `title` vs `text` for the visible words;
`on_select` / `on_click` / `on_change`; `disabled` as `Signal<bool>`
everywhere; `*_label` for every word that can be translated; the same
`class` behaviour. Each rename is noted in a CHANGELOG entry.

### 7. A drift check: `ferrox-ui/agent-kit/check/ferrox-check`

Run in an app, it lists where the app drifted, with the fix: a raw
`<button>` (→ `Button`), `style=` (→ tokens or a layout primitive), a hex
colour or `px` in app CSS (→ a token), a hand-made menu or dialog, a
missing `init()`. File:line, one per line, short. AGENTS.md says to run it
before calling a screen done.

### 8. The agent kit (`ferrox-ui/agent-kit/`, the deliverable)

It lives **inside the `ferrox-ui` crate's folder**, with `docs/`, so the
same files serve both ways a developer can get ferrox-ui:

- **From GitHub** (the plan of record): one clone gives the kit, the docs,
  the recipes and the starter at one version. The developer points their
  agent at the clone; the installer writes the app's git dependency pinned
  to the clone's own tag or commit, so what the agent reads matches what
  the app compiles. No separate kit repo: that would version apart from
  the library.
- **From crates.io**, if it's ever published: Cargo packages only what's
  inside a crate's folder, so `cargo add ferrox-ui` would deliver the kit
  and docs with the crate, version-matched, found through `cargo
  metadata`. Publishing would also mean publishing `ferrox-icons`,
  `ferrox-theme` and `ferrox-theme-build` (crates.io refuses git
  dependencies). A `cargo install ferrox-kit` CLI with the kit built in is
  an option then, beside the scripts. Not planned now.

The shape of a good toolkit: agent-neutral masters, thin per-agent shells,
an installer. Everything an agent needs to build with ferrox-ui, and
nothing about the user's own stack ([[general-not-stack]]).

```
ferrox-ui/agent-kit/
├── README.md              what the kit is; attaching it to Claude Code,
│                          Copilot, any AGENTS.md agent, or any other
├── BOOTSTRAP.md           one paragraph to paste into ANY agent: where the
│                          kit is and what to read first
├── ADAPTING.md            the kit's contract, for writing an adapter for an
│                          agent the kit doesn't know (section 8a)
├── core/                  agent-neutral: plain Markdown, no frontmatter, no
│   │                      agent's tool or feature names
│   ├── AGENTS.md          the entry point (section 1)
│   ├── tasks/             one procedure per task:
│   │   ├── setup.md       add ferrox-ui to a Leptos CSR app
│   │   ├── screen.md      build a screen from a recipe
│   │   ├── component.md   find and use the right component
│   │   └── check.md       run the drift check, fix what it finds
│   └── roles/
│       └── reviewer.md    a reviewer's brief: the rules, the check,
│                          a file:line report
├── adapters/              templates only, rendered per kit.tsv row on
│   │                      install; they hold no content
│   ├── claude/            skill, subagent and command templates
│   ├── copilot/           prompt and instructions templates
│   └── agents-md/         an AGENTS.md section template, for agents that
│                          read one (Codex, Cursor, Copilot and others)
├── kit.tsv                every core piece: id, kind (entry, task, role,
│                          command), path, one-line purpose; the installer
│                          and ADAPTING.md both read it
├── install/               install.sh, install.ps1, install.spec.md,
│                          targets.tsv (adapter → where its files go)
└── check/                 ferrox-check and ferrox-docs (.sh, .ps1,
                           .spec.md), rules.tsv
```

- **Docs that match the version in use.** The reference pages and recipes
  stay in the crate (`ferrox-ui/docs/`). The kit finds them through the
  app's own `Cargo.lock`: `check/ferrox-docs.sh` asks `cargo metadata`
  where the ferrox-ui being built lives, and prints its `docs/` path. An
  agent never reads docs for a different version than it compiles.
- **Installed by copying**, stamped with the ferrox-ui version, so an
  app's repo carries its kit to every machine and teammate; `install.sh
  --update` refreshes it and shows what changed.
- The kit has its own tests: the installer into a scratch app (each mode,
  dry-run, backup), every skill's paths resolving, the drift check against
  fixtures.

### 8a. Any agent, including ones nobody knows yet

The user (2026-10-24): a developer may use any agent, so the kit is
agent-agnostic first, with known agents as conveniences.

- **The core is the product; adapters are wrappers.** Everything an agent
  needs to do well is in `core/`, as plain Markdown that names actions
  generically ("read the file", "run the command", "ask the user"), never
  an agent's tools, slash syntax or frontmatter. A test fails if a core
  file mentions one (a deny-list: `Skill tool`, `Bash tool`,
  `subagent_type`, `$ARGUMENTS`, `#file:`, …).
- **Adapters are generated, not written.** An adapter is a template (how
  that agent wraps one task, one role, the entry point) plus its line in
  `install/targets.tsv` (where the files go). The installer renders one
  shell per `kit.tsv` row: a Claude skill is frontmatter from the row plus
  "follow `core/tasks/screen.md`". Adding a task is a core file and a
  `kit.tsv` row; every adapter gets it. Nothing in `adapters/` repeats
  guidance or descriptions.
- **`BOOTSTRAP.md`, for any agent at all.** A short paragraph a developer
  pastes into an agent's instructions or first message: the kit's path,
  "read `core/AGENTS.md` first", "for a task, read its file in
  `core/tasks/`", "before saying a screen is done, run the check". Any
  agent that can read files and run commands can use the kit with nothing
  else.
- **`ADAPTING.md`, to make a native adapter for a new agent.** The
  contract, and how to map it: the entry point → the agent's
  project-instructions file; each task → its reusable prompt / skill /
  rule; the reviewer role → its subagent or a second session; commands →
  its command mechanism or a documented phrase. It lists `kit.tsv` as the
  source of what to wrap and ends with a checklist. A developer can hand
  `ADAPTING.md` to their own agent and have it write the adapter.
- **`install --generic <dir>`** copies just `core/`, `BOOTSTRAP.md`,
  `kit.tsv` and the scripts, for agents with no adapter.

### 8b. Scripts: bash, PowerShell, and a spec for anything else

Every script the kit ships (`install`, `ferrox-check`, `ferrox-docs`) comes
in three forms (the user, 2026-10-24):

- **bash** (`*.sh`), for Linux and macOS;
- **PowerShell 7** (`*.ps1`, `pwsh`), for Windows (and anywhere else
  `pwsh` runs);
- **a spec** (`*.spec.md`), agent-neutral: inputs, outputs, exit codes,
  what it reads and writes, each step, and the edge cases, written so a
  developer's agent can translate the script into any other language (a
  Makefile, Python, Nushell, …). The kit README says how: "give your agent
  `<script>.spec.md` and the language".

To keep the three from drifting apart:

- **The rules are data, not code.** The drift check's patterns, messages
  and fixes live in one file (`check/rules.tsv`); the installer's what-goes-
  where in another (`install/targets.tsv`). Both scripts read them, and
  the specs describe the format. A new rule is one line, in one place.
- **No tools beyond the shell.** bash uses `grep -E`, `find`, `sed`;
  PowerShell uses `Select-String` and `Get-ChildItem`. Nothing to install
  (no `rg`, no `jq`); `cargo` only where it's needed anyway
  (`ferrox-docs`).
- **The spec is the master; the scripts implement it.** Two
  implementations are the one unavoidable repeat (a script can't link to
  a spec); the shared test suite is what keeps them the same.
- **One test suite for both.** The same fixtures and expected output run
  against the bash and the PowerShell version, so they must agree line for
  line. PowerShell runs locally through `pwsh` on Linux (installed here);
  on a machine without it, that half skips with a clear message.

### 8c. The front door

What a developer who found the repo does first:

- **A starter app** (`starter/`): the smallest app with everything wired
  (dependencies, the theme's build script, the Trunk hook, the stylesheet
  links, the two `init()` calls), one page built from the list-page
  recipe, and an e2e smoke test. "Copy `starter/`, `trunk serve`." It is
  the one home of the setup facts: the `setup` task and the README point
  at its files.
- **"Start here" at the top of the README**, three paths: with an agent
  (install the kit into your app, or paste `BOOTSTRAP.md`, then "use
  ferrox-ui"); a new app by hand (copy the starter); an existing app (the
  setup steps, pointing at the starter's files).
- **A dependency line that works today:** pinned to a git `rev` until
  `v0.1.0` is tagged (the README's `tag = "v0.1.0"` points at a tag that
  doesn't exist yet), rendered from one place.
- The README's "Apps that use webapp-lib…" moves to webapp-lib's docs
  ([[general-not-stack]]).

### 9. Measure it

A reference task, the same before and after: "in a fresh Leptos app with
ferrox-ui, add a recipes list page with add, row actions (edit, delete
with confirm), and empty, loading and error states". A fresh agent (a
subagent, no other context, none of the user's own skills) gets only the
one-line directive: before, with today's README; after, with the kit
installed. Recorded:
tokens used, docs read, compile-fix rounds, and the drift check's findings
on the result. Run once now (baseline) and once at the end.

## Baseline (piece 1, 2026-10-24)

The reference task, as a developer would say it, to a fresh agent (Sonnet,
`general-purpose`, no skills, no memory, only the repo's files): a new
Leptos CSR app with a recipes list page (add in a dialog, row actions:
edit, delete with confirm; empty, loading and error states; in-memory data
with a 500 ms load), ferrox-ui by path, done when `trunk build` passes.
The "after" run (piece 10) uses the same model and wording.

| Measure | Baseline |
|---|---|
| Tokens | **68,009** |
| Tool calls | 13 |
| Time | 1 min 48 s |
| Build rounds | 2 (one compile error) |
| ferrox-ui docs read | the root README; about 650 of the crate README's 1,671 lines, in 7 slices, found by grepping its headings; the demo's `Cargo.toml`, `build.rs`, `Trunk.toml`, `index.html`, `main.rs`; three crates' sources searched for setup functions |
| App written | 187 lines (`app.rs` 163) |

What it built, against the rules this plan will write down:
- Components: `Panel` (H1, a `+` in `actions`), `DataView` (List, loading,
  error, retry, an empty view), `IconButton`s, `Dialog`, `Field`,
  `Input`, `ConfirmDialog`. Good choices, mostly.
- **Drift:** each row is a hand-made `<div class="recipe-row">` with **no
  CSS behind it**: the name and the Edit / Delete buttons sit unstyled side
  by side. No row component or recipe told it what a row looks like. It
  used two `IconButton`s, not a row `Menu`, so its rows differ from the
  demo's.
- **Unfinished:** the error state is wired, but nothing can trigger it.

Where the docs failed it (its own report, confirmed in its code):
1. The README says depend on a tag, "never a branch or a local path", and
   the tag doesn't exist (→ piece 2, front door).
2. The theme setup is documented only with app themes (`embed_themes`,
   `register`); whether the minimal setup (`write_css(…, None)`, `init()`)
   is supported isn't said. The pre-paint script in `index.html` isn't
   mentioned, so it left it out (a theme flash) (→ piece 2, the starter).
3. `DataView`'s `key_of` must return `String`; nothing says so. Its tuple
   key was the one compile error (→ piece 5, signatures in the index).
4. No row template or list-item pattern; the README only shows `Card`
   (→ piece 6, recipes; perhaps a row component, to decide then).
5. Prop types it had to guess: `Dialog`'s `title`, `DataView`'s
   `empty_view` (→ piece 5).
6. Timers: it added `gloo-timers` itself for the simulated load (→ the
   starter or a recipe shows the idiom: Leptos' `set_timeout`).
7. Finding things cost it a heading grep and seven slices of one big file
   (→ piece 5, the index and per-component pages).

## Notes from piece 2 (front door, 2026-10-24)

- **The pre-paint script has one home now:** `ferrox-theme-build`'s
  `assets/prepaint.js`, written by `write_css` beside the stylesheet and
  inlined by `index.html` (`<link data-trunk rel="inline">`). No app
  pastes it; `ferrox_theme::PREPAINT_SCRIPT` and the demo's copy-check
  test are gone (`88425d0`).
- **`starter/`** pins ferrox-ui to `e05d7d1` (the `.ferrox-app` commit) by git (works once that
  commit is pushed). It isn't a workspace member. `scripts/starter-local.sh`
  copies it to `target/starter-local`, rewrites the git dependencies to
  paths into this tree, and builds or serves it; a `[patch]` was tried
  first, but Cargo still fetches the original git source, which needs the
  network and a pushed commit. The release check builds it; Playwright
  serves it on 3399 (`tests/starter.spec.ts`: mount, add, rename, delete
  with confirm, empty view, pre-paint with the WASM blocked).
- **`.ferrox-app`** (`e05d7d1`): the screenshot of the first starter build
  showed the theme reaching every component but not the page (a white
  background, the browser's serif). The page's ground (background, text
  colour, body font) is now an opt-in class in the theme stylesheet, on
  `<body>`; the demo uses it too. Page width and padding stay piece 4's.
- The README's top is "Start here": a new app (copy the starter), an
  existing app (a table of the starter's files and what to take from
  each), an agent (interim wording until the kit, pieces 8–9). The setup
  snippets it used to repeat are gone; "Your own themes" stays (not in the
  starter).
- **For piece 3 (names and ergonomics), found building the starter:**
  - Closures in `Menu`'s (re-buildable) children must be able to run again:
    moving a record into `on_select` fails with an `FnOnce` error. The
    starter uses a `StoredValue` (`Copy`). An agent will hit this; either
    the docs and recipes teach the idiom, or the API avoids it.
  - `DataView` doesn't redraw an item whose key is unchanged, so a rename
    keyed by `id` alone shows the old name. The starter keys by id and
    name. A `version_of`, or redrawing on equality, may be the better API.
- **Moved out of the README, for webapp-lib's docs** (its piece 2), verbatim:

  > Apps that use [webapp-lib](https://github.com/rbenitez22/webapp-lib) get
  > ferrox-ui through its re-exports (`webapp_lib::ui`, `webapp_lib::theme`) and
  > must **not** list ferrox-ui or ferrox-theme themselves. Two references that
  > differ (tag, or `ssh://` vs `https://`) make Cargo build two copies of every
  > type. The only direct dependency such an app adds is `ferrox-theme-build`, in
  > `[build-dependencies]`, from the same tag webapp-lib uses; a mismatch fails
  > the build with a message saying so.

## Piece 3: the names audit (approved 2026-10-24, all twelve; done the same day)

Done in five commits: A, renames and types (1–3, 6–9, 12; `0b446e8`);
B, no DOM events in callbacks (4; `6d82a56`); C, overlays own their state
(5; `76c431d`); D, content built once takes run-once closures (10;
`b177217`), which also covers `Card`'s and `Panel`'s `actions` and the
menus' `header` (the starter's card menu sits in `Card`'s `actions`, so
the trap survived there otherwise); E, `DataView` redraws a changed item
(11), by a generation per identity bumped when the value differs. The
README's conventions state the rules; `CHANGELOG.md` lists every change.
The starter lost both workarounds (its `StoredValue` and its id+name key).

Every public component's props, read from the source (40 components). An
agent generalises from one component to the next, so where the same idea
has two names or two types, it guesses wrong on one of them. Proposed,
with the call sites each one touches in this repo (demo, starter, the
crate itself):

| # | Today | Proposed | Why | Call sites |
|---|---|---|---|---|
| 1 | Visible words typed three ways: `Signal<String>` (most), plain `String` (`IconButton` `title`, every `close_label`, `Alert` `dismiss_label`, `Select` `refresh_label` / `create_label`, `PasswordInput` `show_label`, `SideMenuButton` `label`, `SplitButton` `text` / `menu_label`), `Option<String>` (`Button` `title`) | Every human-readable word is `Signal<String>` (`into`), or `MaybeProp<String>` when optional | One rule: any word can be a literal or a signal (translation, state). Literals keep working | none (literals convert either way) |
| 2 | `IconButton` `title` is the button's **name** (and tooltip); everywhere else `title` is a **heading** (`Panel`, `Card`, `Dialog`, `Sheet`, `Toolbar`, `SideMenu`) | `IconButton` `label`. `Button` keeps `title` (a plain tooltip, as in HTML) | "`label` names a control, `title` is a heading" holds everywhere | 28 |
| 3 | `LoadingOverlay` `text`; `Spinner` `label` for the same thing | `LoadingOverlay` `label` | Same idea, same name | 2 |
| 4 | `on_click` gets the `MouseEvent` on `Button`, `IconButton`, `SplitButton` (`move \|_\|`), nothing on `Card` (`move \|\|`); `SplitButton`'s is a generic `impl Fn(MouseEvent)` | Every `on_*` callback is an `UnsyncCallback`, and none passes a DOM event: `on_click=move \|\| …` | Every callback in the library is written the same way; no example needs the event | 54, mechanical |
| 5 | `Dialog`, `Sheet`, `ConfirmDialog` take `open: Signal<bool>` plus a required `on_close` that must set it false; `SideMenu` takes `open: RwSignal<bool>` and closes itself | Every overlay takes `open: RwSignal<bool>` and closes itself (×, Esc, Cancel, the backdrop); `on_close` / `on_cancel` become optional notices | Removes `on_close=move \|\| open.set(false)` from every dialog; one way for all four | 20 uses; ~16 `on_close` lines go |
| 6 | `Alert`: `on_dismiss`, `dismiss_label` for its ×; every other × is `close_label` | `on_close`, `close_label` | Every × is "close" | 6 |
| 7 | `Dialog`: `dismiss_label`, `no_dismiss` for its Cancel button; `ConfirmDialog`: `cancel_label` for the same button | `Dialog`: `cancel_label`, `no_cancel` | Same button, same name | 2 |
| 8 | `DataView` `empty` | `empty_label` | Every other word prop ends in `_label` (`Combobox` already has `empty_label`) | 1 |
| 9 | `ContextMenu` `items` is a view; `items` is the data on `DataView` and `Combobox` | `ContextMenu` `menu` | One meaning per name | 3 |
| 10 | `Menu`, `SplitButton`, `ContextMenu`, `Panel` take re-buildable children (`ChildrenFn`), so a closure inside can't move a value: the starter needed a `StoredValue` (piece 2's notes) | They build their children **once** (on first opening) and keep them, so take `Children` (`ViewFnOnce` for `ContextMenu`'s `menu`): closures may move values, no `StoredValue` idiom | The trap goes away instead of being documented. `Dialog`, `Sheet`, `ConfirmDialog` and `SideMenu` keep re-buildable bodies (a fresh form on each opening), documented as the one place closures must be re-runnable | none for users; the starter's `StoredValue` goes |
| 11 | `DataView` never redraws an item whose key is unchanged, so a renamed record keyed by id shows its old name (piece 2's notes); the starter keys by id **and** name | `DataView` keys by `key_of` for identity and also redraws an item whose value changed (`T: PartialEq`, compared with its last value) | Key by id, as every agent will; edits show | the starter's key; `T: PartialEq` on items (the demo's already are) |
| 12 | `class` on almost every component; missing on `MenuItem`, `MenuLink`, `LoadingOverlay` | `class` on every component that renders an element | "Every component takes `class`" becomes true | none |

Kept as they are, deliberately:
- `label` for both a field's visible label (`Field`, `Checkbox`, `Switch`)
  and an accessible name (`Menu`, `DataView`, `Spinner`, `NavList`): both
  name the thing.
- One word → an `x_label` prop; a set of words for repeated parts → an
  `XLabels` struct with `Default` (`StrengthLabels`, `PaginationLabels`,
  `DataLayoutLabels`). Already the case; the docs will state it.
- `Combobox` `text` (the field's text) beside `on_select` (the choice);
  inputs' `value`. Different things.
- `on_select` for both a `MenuItem` (run an action) and `Combobox` (choose
  a value): both are "chose this".
- `Button` children as its text; `SplitButton` `text` (its children are
  the menu).

Order of work, once approved: 1–3, 6–9, 12 (renames and types, one
commit), 4 (the callback sweep), 5 (overlays own their state), 10 (built
once), 11 (`DataView` redraw); demo, starter, tests and README with each;
a CHANGELOG started with the renames.

## Notes from piece 4 (layout primitives, 2026-10-24)

- `PageGrid` (width cap, padding, `<main>` / `<aside>` landmarks, an
  optional side column under the main one on a phone), `Stack`, `Cluster`;
  `Gap` on the spacing tokens, `Align`, `Justify`, `PageWidth`,
  `SidePosition`. `landmark=false` for a shell with its own `<main>` (the
  demo).
- The columns space their children by `--space-lg`, so a page of panels
  needs no wrapper.
- `.ferrox-app` now resets `<body>`'s margin: the starter showed 32px at
  the edge (the browser's 8px plus the page's 24px).
- The starter's page is a `PageGrid`: padded, capped at 64rem, a `<main>`.
- The demo's own layout CSS (`.demo-row`, `.demo-stack`, `.demo-main`) is
  left as it is: the demo is a showcase, the recipes (piece 6) are what
  agents copy.

## Notes from piece 5 (reference split, 2026-10-24)

- **Changed from the design:** examples aren't doctests through
  `include_str!`. Rustdoc compiles an example only with hidden `# setup`
  lines, which GitHub's Markdown shows literally (an agent would copy
  them). Instead every example is a region of `tests/doc_examples.rs`,
  compiled with every feature, and `tests/docs_current.rs` copies it into
  its page between `<!-- example: <name> -->` markers. It fails on a stale
  copy, a marker naming no region, a region no page shows, and any Rust
  block in `docs/` outside the markers. The app's own functions are stubbed
  outside the regions; no crate-wide import, so a region's `use` lines are
  complete or it doesn't compile.
- The same test generates `docs/index.md` from the source with `syn`
  (every public component with its props, types, defaults; every other
  public item with its first doc line); `FERROX_BLESS=1` rewrites both.
- Compiling the old README's examples found one that never worked: a
  closure parameter typed `|a: Option<Account>|` inside `view!` breaks the
  macro's parser. The example now lets the type be inferred.
- Pages: one per component or small family (32), one per group (shared
  behaviour), `choosing.md` (the groups' tables, plus new core and forms
  ones), `conventions.md`, `css-classes.md`, `help.md`, `index.md`. The
  crate README is a 90-line overview; the webapp-lib line left it.
- Eight public enums had no doc comment; they do now (the index showed it).

## Notes from piece 6 (recipes, 2026-10-24)

- The recipes are one small app, "Cookbook", at `/recipes` in the demo,
  outside the demo's frame (the demo's pages became a `ParentRoute` with an
  `Outlet`; the recipes are another): a shell, a list page, a form in a
  dialog, a detail page, a settings page, over an in-memory store that
  behaves like a server (its own copy; every change goes through it; `?offline`
  fails the first load). `docs/recipes.md` lists the files; each file is the
  recipe (no copy in the docs).
- **Gaps the recipes found, approved and built:** `AppBar` (there was no
  app bar, so the header was unpadded and flush), plain-link styling in
  `.ferrox-app` (a bare `<a>` was the browser's blue), and `Facts` / `Fact`
  (a record's labels and values; it was a raw `<dl>`). An agent would have
  written CSS for each.
- **Bugs the tests found in the recipes:** a duplicate (id 0) opened as
  "Edit recipe" (the title only asked whether anything was being edited);
  the stand-in store re-seeded when empty, so "delete all" raced the first
  load. The store now keeps a server-side copy, and the settings page
  deletes through `remove_all`.
- The starter stays the smallest app (one list page, no shell).

## Notes from piece 7 (drift check, 2026-10-24)

- `ferrox-ui/agent-kit/check/`: `rules.tsv` (14 rules: `forbid` raw
  controls, inline styles, colours and pixel sizes written out in CSS;
  `require` the two `init()` calls, `.ferrox-app`, the pre-paint script),
  `ferrox-check.{sh,ps1,spec.md}`, `ferrox-docs.{sh,ps1,spec.md}`, and
  fixtures. `tests/agent_kit.rs` runs both implementations against them
  and requires the expected output, byte for byte.
- Found while building: bash's `read` merges the tabs around an empty
  field, which shifted every rule with an empty `except` (the starter
  "passed" for that reason); fields now split on a unit separator. The
  inline-style pattern needed `style=[^ ]` (text mentioning "style= "
  isn't an attribute). Colour and pixel rules let `--token` definitions
  (theme files) and `@media` lines through. `require` rules apply only at
  an app's root (a `Cargo.toml`).
- Targets that must pass: the starter and the recipes. The demo (a
  showcase) trips 19 real findings (raw controls on purpose, its own
  layout CSS); it isn't held to the check.

## Notes from piece 8 (the kit, agent-neutral, 2026-10-24)

- `agent-kit/`: `core/AGENTS.md` (about 2,000 tokens with its rendered
  blocks: the tasks from `kit.tsv`, the rules from `check/rules.tsv`, the
  component table from `docs/choosing.md`), `core/tasks/{setup,screen,
  component,check}.md`, `core/roles/reviewer.md`, `kit.tsv`, `VERSION`,
  `BOOTSTRAP.md`, `ADAPTING.md`, `README.md` (its table rendered from
  `kit.tsv`), `install/{install.sh,install.ps1,install.spec.md,
  targets.tsv}` (`--generic`, `--dry-run`; backs up an old `.ferrox` to the
  first free `.ferrox.bak[.N]`).
- Paths in the kit's Markdown are `{{KIT}}/…`; the installer fills them
  with `.ferrox` (hidden, so the drift check skips the kit itself). The
  setup task points at the starter (`DOCS/../../starter`) instead of
  restating it.
- Tests: the installers give the same output and the same files (executable
  bits kept, no placeholder left, `tests/` left out); a dry run writes
  nothing; usage errors; `core/` and `BOOTSTRAP.md` name no particular
  agent (a deny-list); `kit.tsv`'s paths exist; `VERSION` is the crate's.
- The root README's "With an agent" is now: install `--generic`, paste
  the paragraph.

## Notes from piece 9 (adapters, 2026-10-24)

- `targets.tsv` gained a fourth column, *how*: `copy`, `each:<kinds>`
  (a template per `kit.tsv` row, `{{ID}}` in the path) and `block:<kinds>`
  (the rows' rendered templates as one marked block in a file the app owns,
  replaced on reinstall, the rest untouched). Templates are only an agent's
  boilerplate and placeholders (`{{ID}}`, `{{KIND}}`, `{{PATH}}`,
  `{{PURPOSE}}`, `{{KIT}}`); a test caps their own words at six.
- Claude: a skill and a command per task, the reviewer as a sub-agent, a
  `CLAUDE.md` block. Copilot: a prompt file per task and role, a
  `copilot-instructions.md` block. AGENTS.md: a block listing every piece.
  Every adapter installs the generic kit; `.ferrox/INSTALLED` records them.
- `--update` (with the recorded adapters, or new flags) prints `added`,
  `changed`, `removed` only, and removes kit files no longer in the plan.
- Both installers agree on every path tried (all adapters, generic, dry
  run, update, a `CLAUDE.md` without a final line ending). The PowerShell
  one uses ordinal string comparisons (.NET's defaults are
  culture-sensitive). The bash one needs bash 4 (associative arrays).

## Piece 10: measure again (2026-10-25)

### The run

Same as the baseline: Sonnet, `general-purpose`, no skills, no memory, an
empty app folder, ferrox-ui by path, done when `trunk build` passes. The
kit was installed with `--generic` (a subagent doesn't load skills from the
app's `.claude/`, so `--claude` would have measured nothing extra), and the
prompt began with `BOOTSTRAP.md`'s paragraph plus the app's folder. The
baseline's exact prompt wasn't kept; this one was rebuilt from the
description above. From now on the prompt is kept in
`agent-kit/tests/measure-prompt.md`, so later runs use the same words.

| Measure | Baseline | With the kit | After 10b–c | After 10e |
|---|---|---|---|---|
| Tokens | 68,009 | 64,708 | 61,335 | 60,793 |
| Tool calls | 13 | 14 | 10 | 11 |
| Time | 1 min 48 s | 1 min 46 s | 1 min 42 s | 1 min 59 s |
| Build rounds | 2 (one compile error) | 1 (no errors, no warnings) | 1 (no errors; a second after a comment edit) | 1 (no errors; two dead-code warnings) |
| Drift check | (no check yet) unstyled hand-made rows; an error state nothing triggers | no drift | no drift | no drift |
| Read | the root README, 650 lines of the crate README in 7 slices, the demo's setup files, three crates' sources | `AGENTS.md`, the four tasks, the starter's files, `recipes.md` and six component pages, three recipe sources in full | `AGENTS.md`, the four tasks, `.ferrox/starter`, `recipes.md`, `conventions.md`, three recipe sources; no component page | the same, less `store.rs`'s middle (its own truncated output) |
| App | 187 lines | 336 lines (the list recipe, trimmed: row `Menu`, `ConfirmDialog`, `DataView` with Retry, `?offline` shows the error) | 351 lines (the same recipes, as a `recipes` module) | 390 lines (the same) |

The cost is the same; the result went from drifted to clean, first build.

### Where it stumbled

1. **A circle at the start.** `AGENTS.md` step 1 runs `ferrox-docs`,
   which needs a `Cargo.toml` depending on ferrox-ui; an empty folder has
   none, and `setup.md` finds the starter only through DOCS
   (`DOCS/../../starter`).
2. **Path or git.** The starter pins a git `rev`; switching to a local
   path is a comment split over lines, easy to miss.
3. **Recipes assume the demo's shell:** `store.settings`, a
   `/recipes/{id}` route on the card, `use super::…`. It had to find and
   strip these itself.
4. **Recipe sources live in `demo/`**, outside the crate: `recipes.md`'s
   links work only in a whole checkout (a git dependency has one, a
   crates.io one won't).
5. The starter's header comment ("ready to rename") survives the rename.

### Fixes (each its own commit)

- **10a. Record this run.** This section, and the prompt in
  `agent-kit/tests/measure-prompt.md`.
- **10b. The starter ships in the kit** (fixes 1, 2, 5). `targets.tsv`
  gains `generic ../../starter .ferrox/starter copy` (without `target/`,
  `dist/`); the kit is copies already, made from the same checkout, so the
  starter matches the kit's version and its pinned `rev`. `setup.md` (new
  app): copy `{{KIT}}/starter`, no DOCS needed; one plain step for a local
  clone (each of the four `git`/`rev` → `path`); remove the header comment
  on rename. `AGENTS.md` step 1: "no `Cargo.toml` yet? do `setup` first".
  The starter's comment becomes one line. Tests: the installers still
  agree; the installed starter equals `starter/`.
  *Done:* the starter is listed file by file in `targets.tsv` (a folder
  copy would take its `target/`, `dist/`, `style/ferrox/`), and a test
  compares the installed copy with `git ls-files starter`, so a new
  starter file can't be left out. The installer reaches it as
  `../../starter`, so it must run from a checkout of the repo (a local
  clone, or Cargo's git cache), as the README already says; a packaged
  crate alone has no starter.
- **10c. Recipe sources move into the crate** (fixes 3, 4):
  `ferrox-ui/docs/recipes/*.rs`, which ships with the crate and is found
  through DOCS like every page. The demo compiles them from there
  (`#[path]` in `demo/src/recipes/mod.rs`), so its e2e tests still prove
  them. `recipes.md` links `recipes/<file>.rs`, and each recipe's row
  says what to change when copying it alone (the module path, the
  settings it reads, the detail-page link). Tests: `docs_current`, the
  demo's e2e, `cargo package --list` includes them, `ferrox-check` on the
  demo.
  *Done:* `demo/src/recipes/mod.rs` keeps only the module tree, each
  module `#[path]`ed to its file in the crate's docs, so `super::` works
  the same in both places. `recipes.md` gained "Copying one recipe alone":
  per recipe, what it needs from the rest of Cookbook and what to do
  without it. The drift-check test now runs on `docs/recipes/`. Found on
  the way: 10b's one-line comment in `starter/Cargo.toml` tripped
  `starter-local.sh`'s `git = ` check, now limited to code lines.
- **10d. Measure once more** with the same prompt, in a fresh empty
  folder; record the third column here.
  *Done:* the third column above. No stumble at the start; it read
  less (no component page: the recipe compiled as copied). What it still
  had to guess:
  1. The path swap is prose; its first `sed` broke `Cargo.toml`. The
     starter's pinned-rev comment goes stale after the swap.
  2. `store.rs` reads `window().location()`: web-sys `Location`, which the
     starter's `Cargo.toml` doesn't enable; it added it before building.
  3. Wiring copied recipes in (`mod recipes;`, `Store::provide()`) is
     said nowhere; it inferred it from the shell recipe.
  4. Trunk warns the starter's `address` is deprecated (`addresses`).
  5. `screen.md` sends it to `DOCS/choosing.md`, whose table `AGENTS.md`
     already carries.
- **10e. Fix those five** (approved 2026-10-25), then measure a fourth
  time. *Done:* the starter carries the four path lines, commented, under
  the git ones (a test keeps them naming the same crates and features),
  and `setup.md` says to edit, not script, the swap; web-sys `Location` in
  the starter; "Wiring copied recipes in" in `recipes.md` (the module,
  re-exporting the screens and `Store`, `Store::provide()`, the
  `Location` feature); `addresses` in the starter's and the demo's
  `Trunk.toml`; `screen.md` points at `AGENTS.md`'s table.
  *Fourth run:* the cost has levelled off (61k), with no compile error
  and no drift in each run since the kit; it read no component page.
  What was left: `setup.md`'s "copy every file to the app's folder" let
  it copy into `.ferrox/` first (its own slip, undone; `--update
  --dry-run` shows the kit intact), and the unused parts of `store.rs`
  (`Settings.units`, `remove_all`, `find`) gave dead-code warnings the
  copying table doesn't mention. Both are one-line fixes (10f).

## Pieces

Each is small and reviewed on its own.

1. **Baseline.** The reference task, run against today's docs; numbers in
   this plan.
2. **Front door.** The starter app (with its e2e smoke test), "Start here"
   in the README, the dependency line that works today.
3. **Names.** The audit (a table: today → proposed), approved, then the
   renames, demo, tests, CHANGELOG.
4. **Layout primitives.** `Stack`, `Cluster`, `PageGrid`, `Gap`; demo,
   e2e.
5. **Reference split.** `docs/` pages from the README, `docs/choosing.md`,
   the render tool (index from source, rendered blocks between markers)
   and its staleness test, doctests through `include_str!`; the README
   becomes the overview, linking.
6. **Recipes.** The five screens as demo pages (e2e), and
   `docs/recipes.md` listing them; the starter's page becomes the list
   recipe's copy.
7. **Drift check.** `check/rules.tsv`; `ferrox-check` and `ferrox-docs`
   in bash and PowerShell, with their specs; one test suite for both, run
   against the demo (which must pass) and fixtures (which must fail the
   right way).
8. **The kit, part 1 (agent-neutral):** `core/` (AGENTS.md, the four
   tasks, the reviewer role), `kit.tsv`, `BOOTSTRAP.md`, `ADAPTING.md`,
   the installer's `--generic` in bash and PowerShell with its spec, the
   kit README, the core deny-list test.
9. **The kit, part 2 (adapters):** the templates in `adapters/claude`
   (skill, subagent, command), `adapters/copilot`, `adapters/agents-md`;
   the installer's `--claude`, `--copilot`, `--agents-md` and `--update`
   rendering them (both shells, `install/targets.tsv`); a test that
   renders every adapter for every `kit.tsv` row and checks no rendered
   shell carries text that isn't from `kit.tsv`.
10. **Measure again** with the kit installed; adjust where the agent
    stumbled.

## Open points

1. **Agent-neutral masters plus thin per-agent shells** (the toolkit
   shape), recommended: one copy of the guidance, any agent. The
   alternative is writing each agent's format separately.
2. **Split the README into `docs/` pages, the README an overview**,
   recommended. The alternative keeps one README and adds only the index,
   still 18,000 tokens when an agent opens it.
3. **Layout primitives** (`Stack`, `Cluster`, `PageGrid`), recommended:
   page spacing is where screens drift once components are shared. The
   alternative is a page of CSS rules in AGENTS.md, which costs tokens
   every time and is followed loosely.
4. **Renames now**, before 0.1.0 and before any app depends on ferrox-ui,
   recommended. The alternative is living with the inconsistencies, which
   agents will copy.
5. **Measuring with a subagent**, twice, recommended: it costs one agent
   run each time, and it's the only number that says whether this worked.
6. **The drift check as scripts over a rules file** (`grep -E` /
   `Select-String` patterns in `check/rules.tsv`) first, recommended; a
   Rust tool only if patterns aren't enough.
7. **Install by copying, version-stamped** (`--update` to refresh),
   recommended: the app's repo carries the kit to every machine. The
   alternative, symlinks into a ferrox-ui checkout, breaks anywhere that
   checkout isn't.
8. **Adapters shipped:** Claude Code (skills, subagent, commands),
   `AGENTS.md` (read by Codex, Cursor, Copilot and others), Copilot
   prompts, recommended; every other agent through `BOOTSTRAP.md` and
   `ADAPTING.md`. More native adapters only when someone asks.
9. **Docs found through `cargo metadata`** (version-matched, nothing
   copied), recommended. The alternative copies `docs/` into the app on
   install, which goes stale when ferrox-ui is upgraded.
10. **PowerShell 7 (`pwsh`)** for the Windows scripts, recommended: it's
   current, cross-platform (testable here), and installs alongside the old
   one. The alternative also targets Windows PowerShell 5.1, which is on
   every Windows machine but older and quirkier; worth it only if a user
   can't install 7.
11. **`pwsh` on this machine** for the tests: done (2026-10-24, the
   user's go-ahead). PowerShell 7.6.8 from the official release tarball
   (SHA-256 checked) in `~/.local/share/powershell/7.6.8`, linked as
   `~/.local/bin/pwsh`; no root. The PowerShell half of the suite still
   skips loudly on a machine without `pwsh`.
