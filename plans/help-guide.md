# Help from Markdown, related topics, and the full guide

Status: approved 2026-11-05 (open questions as recommended). Piece 1 done: `ferrox-help-build` parses, validates and writes Rust; `HelpTopic::related` / `related_ids` added; 44 rusty-skillet topics generate and compile. Awaiting review before piece 2 (JSON).

This is Track A of the help roadmap in rusty-skillet's `plans/help-docs.md`. It builds the help work deferred in `plans/help.md` (open point 6: topic registry, checked `related`, a `/help` guide page) as reusable ferrox pieces. Rusty Skillet is the first consumer: its 44 topics are already written as Markdown in `rusty-skillet/docs/help/`.

## Goal

An app writes its help once, as Markdown files, and gets the following:

- **Generated `HelpTopic` statics** for its `help=` props, one constant per topic. A wrong topic id is a compile error.
- **`related` links** between topics, checked at build time. In a `?` popover, a related link swaps to that topic in place.
- **A full guide page** (`HelpGuide`) rendered from the same topics: a contents list, one section per topic, and a section for each `#id` in the URL.
- **A JSON copy** of the same topics, so the app's API can serve them to an assistant. Garçon's `search_help` / `get_help_topic` tools come later, in Track B4.

ferrox-ui itself still parses nothing at runtime. All Markdown handling happens in a build script, keeping the rule from `plans/help.md`.

## Out of scope

The following belong to other tracks or have been ruled out:

- **Chat UI and assistant wiring.** Track B covers these, in garcon-client. ferrox keeps `HelpAssistant { on_ask }` as it is.
- **Search inside the guide.** This could come later, and the `HelpTopics` API leaves room for it.
- **Translation.** There is one language per app. Topics remain data, so a locale can later be added to the generator.
- **Inline styling** (bold, links) inside blocks. `HelpBlock` stays as it is, `#[non_exhaustive]`.

## Design

### A1. `ferrox-help-build`: a new crate that uses only the standard library

It follows the `ferrox-theme-build` pattern: no dependencies, a `links` key so an app gets one copy, and it is used from `build.rs`. The Markdown subset is small enough for a hand-written line parser, so there's no pulldown-cmark.

```rust
// build.rs
fn main() {
    ferrox_help_build::Help::load("../docs/help")   // reads ORDER + area files, validates
        .guide_prefix("/help#")                     // each topic gets .guide("/help#<id>")
        .write_rust("help_topics.rs");              // $OUT_DIR/help_topics.rs
}
```

```rust
// src/help.rs
include!(concat!(env!("OUT_DIR"), "/help_topics.rs"));

// in a view
<Page title="Recipes" help=help::RECIPE_LIST.clone()>
```

An API's `build.rs` uses the same loader to write JSON: `.write_json("help.json")`.

**The format.** It is defined in rusty-skillet's `docs/help/README.md`. That definition moves to `ferrox-ui/docs/help-markdown.md` and becomes the contract; rusty-skillet's README then links to it.

- An `ORDER` file lists the area files in order.
- Each file has one `# Area` heading and an optional intro paragraph.
- Topics are `## Title {#id}` headings.
- A topic body can hold:
  - paragraphs (consecutive lines joined with a space)
  - `- ` lists, with two-space continuation lines
  - pipe tables
  - a final `Related: #id, #id` line
- `<!-- -->` comments are dropped.

**Validation.** Each failure panics with `file:line: message`, so `cargo build` names the problem. The build fails on:

- a file missing from `ORDER`, or listed but absent;
- an id that is not `^[a-z][a-z0-9-]*$`, or that appears twice across all files;
- a `### ` heading;
- a topic that is empty, or whose first block is not a paragraph;
- ragged table rows, or a bad separator line;
- a `Related:` id that names no topic, or the topic itself;
- inline markup: `**`, a backtick, `](`, or anything shaped like an HTML tag.

**Generated Rust.** For each topic, the generator writes `pub static RECIPE_SEARCH: LazyLock<HelpTopic>`, the id upper-snake-cased; ids are unique, so the names are too. Each static is built with the existing builder: `.paragraph(..)`, `.list(..)`, `.table(..)`, `.related(..)` and `.guide(..)`.

It also writes `pub static TOPICS: LazyLock<HelpTopics>`, the ordered registry from A2. It emits a `cargo:rerun-if-changed` line for the directory and each file.

**Generated JSON.** The shape is `{ "areas": [{ "title", "intro", "topics": [{ "id", "title", "blocks": [...], "related": [...], "guide" }] }] }`. Each block is `{"paragraph": "…"}`, `{"list": [...]}` or `{"table": {"columns", "rows"}}`. JSON is written by hand, since there is no serde dependency.

### A2. `related` and the `HelpTopics` registry (in ferrox-ui `core`)

- **`HelpTopic::related(ids)`** and **`related_ids() -> &[String]`**. These are ids, not topics, so statics don't have to reference each other. The registry resolves them.
- **`HelpTopics`** holds areas, in order, and their topics. It is cheap to clone, using an `Arc`. It offers:
  - `HelpTopics::new(areas: Vec<HelpArea>)`
  - `get(id) -> Option<HelpTopic>`
  - `areas()`
  - `iter()`, which returns topics in reading order
  - `resolve_related(&topic) -> Vec<HelpTopic>`

  **`HelpArea { title, intro, topics }`** is a public struct, `#[non_exhaustive]`, with a constructor.

  Hand-written topics, like Conrad's or the demo's, can be put in a registry too, but they don't have to be. `HelpTopics::new` debug-asserts that ids are unique and that every related id resolves, so hand-built registries get the same check in tests.
- **`provide_help_topics(topics)`** / **`use_help_topics() -> Option<HelpTopics>`** put the registry in context. Apps call `provide_help_topics(help::TOPICS.clone())` at the root.

### A3. Related links in the `HelpButton` popover

- When the registry is in context and the topic has `related` ids, the popover shows a "Related" list of topic titles. A new `related_label` prop holds the "Related" text, for i18n.
- Tapping one swaps the popover's content to that topic in place, as Conrad's `HelpDialog` does. Focus moves to the new title. Reopening the `?` always starts at its own topic.
- "Full guide" and "Ask about this" follow the topic on screen, so `HelpRequest` gets the topic the user is actually reading.
- Without a registry, nothing changes: related ids alone aren't rendered.
- **Refactor:** `render_block` moves from `help_button.rs` to its own `core/help_blocks.rs`, as `HelpBlocks { blocks }`. A3 and A4 both use it. The CSS classes become `help-blocks__*`. The popover keeps its `help-popover__*` classes for layout only, so existing themes aren't broken; `docs/help.md` lists the change.

### A4. `HelpGuide` component

```rust
<HelpGuide topics=help::TOPICS.clone() />   // or omit `topics` to use the context
```

**What it renders:**

- A contents list grouped by area, with links to `#id`.
- Each area as a heading with its intro.
- Each topic as `<section id=topic-id>`, containing:
  - the title as a heading;
  - the blocks, via `HelpBlocks`;
  - "Related" links to `#id`;
  - when inside a `HelpAssistant`, an "Ask about this" button that sends a `HelpRequest` with component `"guide"`.

**Behavior:**

- On mount, and whenever the hash changes, it scrolls to the hash's section and moves focus to its heading.
- The app owns the route (`/help`), the page frame (`Page` with a title and back button) and the side-menu entry. `HelpGuide` is only the body.
- It has a `class` prop and `contents_label` / `related_label` / `ask_label` props.
- Accessibility:
  - the contents list is a `nav` named by `contents_label`;
  - each section is labelled by its heading.

### A5. Demo, docs, release

- **Demo.** The demo's help topics move into `demo/help/*.md`, which `demo/build.rs` reads. They gain related links, and a `/help-guide` route shows `HelpGuide`. The existing demo page keeps working on the generated statics.
- **e2e tests** (Chromium + Firefox):
  - a related link swaps the popover and focus moves;
  - reopening resets to the button's own topic;
  - "Ask" carries the swapped topic;
  - the guide renders areas and sections;
  - `/help-guide#id` scrolls and focuses;
  - the guide's "Ask" works.
- **Docs.**
  - `docs/help-markdown.md`: the format contract, moved from rusty-skillet.
  - `docs/help.md`: new sections for `ferrox-help-build`, `related`, `HelpTopics`, `HelpGuide` and `HelpBlocks`, and the CSS class change.
  - Agent hints in `agent-kit/core/AGENTS.md`.
  - `README.md`: the deferred-registry note becomes current.
  - `CHANGELOG.md`.
  - `plans/help.md`: open point 6 marked done, pointing here.
- **Release.** The workspace version goes from 0.1.1 to **0.2.0** (new public API and a new crate; no breaking changes), with one tag, `v0.2.0`.

## Pieces (each reviewed before the next)

1. **`ferrox-help-build`: parser, validation and Rust output.**
   - Unit tests cover each validation failure with its `file:line` message, and the generated code for a small fixture.
   - The generator is run once against rusty-skillet's `docs/help` as a check.
   - Its generated `.related(..)` calls need A2, so A2's `related` builder lands in this piece too. It is a few lines.
2. **JSON output**, with a round-trip test against the fixture.
3. **`HelpTopics` / `HelpArea` / context**, and the `HelpBlocks` extraction, with unit tests.
4. **Related links in the `HelpButton` popover**, with e2e tests.
5. **`HelpGuide`**, with e2e tests.
6. **Demo moved to Markdown, docs and CHANGELOG, then tag `v0.2.0`.**

After this, rusty-skillet's C2 and C3 (wiring the props and the `/help` route) can start, as can garçon's B4 (help endpoints and tools), which uses the JSON from piece 2.

## Open questions

1. **The guide's page name.**
   - The component is called `HelpGuide`, and the route `/help` is the app's choice.
   - **My recommendation is to keep both.**
2. **Whether the generated statics should be `pub`.**
   - They are generated into the app's own module, so the app chooses visibility through its `mod help`.
   - **My recommendation is that the generator emits `pub`, leaving the module's visibility to the app.**
3. **Where the rusty-skillet format spec moves.**
   - The format spec in rusty-skillet's `docs/help/README.md` moves here in piece 6.
   - rusty-skillet's README then keeps only its writing conventions and links to ferrox for the format.
   - **My recommendation is to do this move.**
