# help: a `?` on any component, and context for an assistant

Components can carry help. When an app gives a component a help topic, a
discreet `?` appears beside its label or title. It opens the topic: a short,
structured explanation that works offline. When the app also provides an
assistant, the same popover offers **"Ask about this"**. That hands the app a
`HelpRequest` with the topic, the component, and every bit of context the app
declared around it (page → section → field). The app sends it to its chat,
LLM or Patina recall. ferrox-ui never talks to a model itself.

Status: approved (2026-10-23). Done (2026-10-23): pieces 1–4. Forms piece 4 (Combobox) is next, on `core/anchor.rs`.

Notes from piece 1:
- The `?` opens the popover declaratively (`popovertarget`), so the browser
  handles opening, Esc, light dismiss, one-open-at-a-time and focus going
  back to the `?` on Esc. The component only places the popover (in
  `on:toggle`), focuses it so a screen reader reads it, and mirrors
  `aria-expanded`.
- `core/anchor.rs` splits the maths (`position`, unit-tested: below, above,
  the roomier side, clamped at the edges) from the DOM (`place`, and
  `follow`, which re-places it every frame while open, so it tracks page
  scroll, scrolling parents and layout shifts alike). It sets `left`/`top`
  from script and `data-side` for tests: the one inline style, because the
  position is computed.
- `ids` is now compiled always, since core uses it.
- Piece 2: the hook is a scoped component, `<HelpAssistant on_ask=…>`, not
  the planned `provide_help_assistant()` function. A plain `provide_context`
  leaks to siblings (the lesson from `Field`), and a component makes the
  scope explicit: wrap the whole app, or only part of it.
- `HelpContext` converts from an array or a `Vec` of pairs, or from a
  closure returning them (context that follows the page).
- A `?` keeps the scope's context as a signal and reads it when "Ask about
  this" is clicked, so a changing account is sent as it is then (an early
  version took a snapshot at creation; caught in review).
- `HelpButton` gains `kind`, `about` and `value` (what the request says
  carries the `?`, its label, an opt-in value) and `ask_label`. The Ask
  button closes the popover declaratively (`popovertargetaction="hide"`).
- Piece 3: every `help` prop takes `MaybeHelp` (from a `HelpTopic` or an
  `Option<HelpTopic>`), because Leptos can't forward an optional prop as an
  `Option`; `ConfirmDialog` passes its own on to `Dialog`. One crate-internal
  helper, `component_help`, places the `?` for every component.
- Field's value reaches a help request through a slot in `FieldContext`
  that `Input`, `Textarea` and `Select` fill (`offer_value`). `PasswordInput`
  never does, so a password can't be sent even with `help_shares_value`.
- With help, a toolbar's title and `?` share the free space together
  (`.toolbar__heading`), so the `?` stays by the title. Without help the
  markup is unchanged.
- A dialog's `component` in the request is `"alertdialog"` when `alert` is
  set (so `ConfirmDialog` reads as one).
- Recommended context keys, including `mcp_server` / `mcp_tool` as routing
  hints, are in the README (added at the user's request).

## Prior art: conrad

conrad already has the static half (`ui/src/help/`):

- **`HelpButton`.** It names what it explains: "Help: Debits and credits", as
  `aria-label` and `title`, so a screen reader knows which `?` it landed on.
  It's an `.icon-btn`. In a row of fields it can leave the Tab order
  (`tabIndex=-1`), so it doesn't cost a stop on every entry.
- **Topics are data, not markup.** A topic is a title, structured blocks
  (`paragraph`, `list`, `table`) and related topics. A dialog and the full
  `/help` page render the same blocks; neither parses Markdown or trusts a
  string of HTML.
- **There's one body of content and two ways in.** A `?` opens one topic; the
  guide shows them all in order, and the `?` links to its place there ("Full
  guide").
- **Granularity:** one topic per thing that can carry a `?`.
- **`help` slots** on `Dialog` (title bar, before the ×) and `FormField`
  (beside the label, never inside the `<label>`: a button inside a label is a
  nested control).

The assistant half (context, the request, the hook) is new.

## Design

### Where it lives: `core`, no feature

`Field` (forms), `Dialog` (overlay) and `Toolbar` (core) all take a `help`
prop. Leptos component props can't be switched on by a Cargo feature, so help
belongs in `core`. It's small, and it costs nothing unless a topic is given.

### Topics

```rust
/// One thing a `?` can explain. Cheap to clone (shared inside).
#[derive(Clone, Debug, PartialEq)]
pub struct HelpTopic { /* Arc<…> */ }

impl HelpTopic {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self;
    pub fn paragraph(self, text: impl Into<String>) -> Self;
    pub fn list<I: Into<String>>(self, items: impl IntoIterator<Item = I>) -> Self;
    pub fn table(self, columns: …, rows: …) -> Self;
    /// "Full guide": where the same words sit among the rest.
    pub fn guide(self, href: impl Into<String>) -> Self;
    pub fn id(&self) -> &str;
    pub fn title(&self) -> &str;
}

#[non_exhaustive]
pub enum HelpBlock { Paragraph(String), List(Vec<String>), Table { columns: Vec<String>, rows: Vec<Vec<String>> } }
```

```rust
static AMOUNT: LazyLock<HelpTopic> = LazyLock::new(|| {
    HelpTopic::new("split-amount", "Amounts on a split")
        .paragraph("Each line moves money into or out of one account.")
        .list(["Positive: money in", "Negative: money out"])
        .guide("/help#split-amount")
});
```

Topics come from the app, so the app owns their wording and translation. A
topic registry, with `related` ids checked against it, comes later (open
point 6).

### HelpButton and its popover

```rust
#[component]
pub fn HelpButton(
    topic: HelpTopic,
    /// Leave the Tab order: for a `?` in a row of fields (still clickable,
    /// still reached by a screen reader).
    #[prop(optional)] skip_tab: bool,
    /// "Help: {title}" by default.
    #[prop(into, default = "Help".into())] help_label: Signal<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView
```

- An `.icon-btn` with the `HELP` icon, labelled "Help: {title}".
- It opens a **popover** (open point 1): the native `popover` attribute, in
  the top layer, with `aria-expanded` and `aria-controls`. Esc closes it, and
  so does a click elsewhere (light dismiss). Focus returns to the `?`.
- The popover shows the title, the blocks, "Full guide" when given, and "Ask
  about this" when an assistant is provided. Long topics scroll inside a
  capped height.
- **Anchored positioning** goes in a shared core module,
  `core/anchor.rs`. It places a top-layer element under its anchor, flips
  above when there's no room below, keeps it inside the viewport, and follows
  scroll and resize. The forms Combobox list needs exactly this (forms
  piece 4), so it's built once, here.

### Context: `HelpScope`

The app declares what an assistant should know, at the level where it's
known. Scopes nest, and a request collects them outermost first.

```rust
<HelpScope context=[("screen", "reconcile")]>
    <HelpScope context=move || vec![("account", account_id.get())]>
        <Field label="Statement balance" help=STATEMENT_BALANCE.clone()>
            <Input value=balance />
        </Field>
    </HelpScope>
</HelpScope>
```

- `context` takes a list of key/value pairs, or a signal or closure of one,
  so the context can follow the page (the account being reconciled).
- It's provided through `<Provider>`, like `Field`'s wiring, so a scope
  reaches only its own children.
- **Keys are free-form.** The README recommends a few conventional ones
  (`domain`, `screen`, `section`, `entity`), because those map straight onto
  Patina tags (`recall_tagged`).

### The assistant hook and the request

```rust
/// What "Ask about this" hands the app.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct HelpRequest {
    pub topic_id: String,
    pub topic_title: String,
    /// What carries the `?`: "field", "checkbox", "switch", "dialog", "sheet", "toolbar".
    pub component: &'static str,
    /// The component's label or title ("Statement balance").
    pub label: String,
    /// Every enclosing `HelpScope`'s pairs, outermost first.
    pub context: Vec<(String, String)>,
    /// The control's current value, only when the component opted in.
    pub value: Option<String>,
}

impl HelpRequest {
    /// Plain text for a prompt: one `key: value` per line, topic and
    /// component included. No serde dependency needed to use it.
    pub fn to_prompt_context(&self) -> String;
}

/// Turns on "Ask about this" for every `?` below; usually once, at the root.
pub fn provide_help_assistant(on_ask: impl Fn(HelpRequest) + 'static);
```

- **Without an assistant**, help is just the static topics. Nothing else
  changes.
- **ferrox-ui has no chat UI and no model call.** The app decides what "ask"
  does: open a `Sheet` with its chat, call Claude with
  `to_prompt_context()` plus the topic's words, or recall Patina memories
  tagged with the context. webapp-lib can offer that wiring as an adapter
  later (a separate story there).
- **Privacy by default:** a request carries only what the developer
  declared. A field's value is sent only when that component says
  `help_shares_value=true`, and never for a `PasswordInput`, which doesn't
  offer the option.
- A `?` appears only where a topic is given. There's no "ask anywhere" `?` on
  components without one (open point 4).

### The `help` prop on existing components

| Component | Where the `?` sits | `label` in the request |
|---|---|---|
| `Field` | right after the label text, outside the `<label>` | the label |
| `Checkbox` | after its words, outside the `<label>` | the words |
| `Switch` | beside the button (it can't go inside: a button in a button) | the words |
| `Dialog`, `ConfirmDialog` | title bar, before the × | the title |
| `Sheet` | title bar, before the × | the title |
| `Toolbar` | after the title | the title |

`Field`, `Checkbox` and `Switch` also take `help_skip_tab` (conrad's
`tabIndex=-1`). `Field` and `Checkbox` take `help_shares_value`.

### CSS: `core/help.css`

- `.help-btn`: an `.icon-btn` sized for sitting beside text.
- `.help-popover`, `__title`, `__body`, the block styles (`__list`,
  `__table`), and `__actions` for Full guide and Ask about this.
- In a title bar, the `?`'s hover uses `--neutral-tint`, like the ×.

## Pieces

Each one passes `scripts/release-check.sh` and can be reviewed on its own.
The demo gets a `/help` page.

1. **Anchored popovers, topics and HelpButton.** `core/anchor.rs`,
   `HelpTopic`/`HelpBlock`, `HelpButton`, `help.css`. e2e:
   - the label is "Help: {title}";
   - it opens in the top layer with `aria-expanded`;
   - paragraph, list and table render;
   - the Full guide link;
   - Esc and an outside click close it, and focus returns to the `?`;
   - `skip_tab` takes it out of the Tab order;
   - it stays inside the viewport near each edge, flips above near the
     bottom, and follows scrolling.
2. **HelpScope, HelpRequest and the assistant hook.** The demo's fake
   assistant opens a Sheet that shows the request it got. e2e:
   - no "Ask about this" without an assistant;
   - with one, the request has the topic, component and label;
   - nested scopes come outermost first, and reactive context updates;
   - a scope doesn't leak to its siblings;
   - no value unless opted in;
   - `to_prompt_context()` (also unit-tested).
3. **The `help` prop on Field, Checkbox, Switch, Dialog, ConfirmDialog,
   Sheet, Toolbar.** e2e: each placement, never inside a `<label>` or a
   button, the request's component and label, `help_skip_tab`,
   `help_shares_value`.
4. **Docs.** A README help section: when to give help, granularity, the
   context conventions for assistants and Patina (folding in the "Help
   context: recommended keys" table added 2026-10-23, with `mcp_server` /
   `mcp_tool`), privacy, and the class reference. Plus the root README and plan status. webapp-lib gets a note
   about the assistant adapter as a future story.

After this, forms piece 4 (Combobox) builds on `core/anchor.rs`.

### Follow-up: F1 (proposed by the user 2026-10-23, not started)

F1 opens the help for whatever has focus; Shift+F1 asks the assistant
directly. It's opt-in (a prop on `HelpAssistant`, or a `<HelpKeys/>`
component), since it takes over a key.

- Each component with help links its control to its `?` (e.g. a
  `data-help` id), so F1 finds the exact `?` by walking up from
  `document.activeElement`.
- The `?` gets `aria-keyshortcuts="F1"`.
- Check that `preventDefault` stops Chrome's and Firefox's own F1 help on
  Windows and Linux. On macOS, F1 is usually brightness without fn, so it's
  a shortcut on top of the visible `?`, never the only way in.
- Most useful where the `?` uses `skip_tab` (rows of fields).

### Follow-up: noticing when a user needs help (discussed 2026-10-23, not started)

The user's example: entering a transaction whose splits keep failing the
balance check. The design splits the work into three layers.

1. **Components report facts, not judgments (ferrox-ui, opt-in).** One
   observer, `<HelpSignals on_signal=…>`, scoped like `HelpAssistant`.
   Components emit generic events, each with the enclosing `HelpScope`
   context:
   - a Field's error appeared, cleared, or persisted through N edits;
   - a Dialog's submit was refused;
   - help was opened, or dismissed within a second;
   - a Combobox was left with no match, repeatedly.

   There's no guessing from raw behaviour (dwell, rage clicks): it's noisy
   and reads as surveillance.
2. **The app reports domain signals through the same channel.** A public
   `report_help_signal(kind, detail)`, for example "imbalance" with the
   number of edits it has persisted and (opt-in) the amounts. Only the app
   knows these, and they're the strongest signals.
3. **A policy decides whether to offer help (webapp-lib or the app, not
   ferrox-ui).** It sets thresholds and cool-downs (for example, imbalance
   for 3 edits or 30s, offered once per entry).
   - It **offers and never interrupts**: a "suggested" state on the relevant
     `?` (ferrox-ui provides the state, e.g. `data-suggested`), or a quiet
     "Need a hand…?" link, announced with `aria-live`. It never steals focus
     and never auto-opens a chat.
   - The offer opens the right topic, or asks the assistant with the context
     already filled in.

**Payoff:** high for the few known pain points the app already detects
(imbalance, reconciliation that won't tie out, failing import mappings),
low for generic struggle detection. **Privacy:** signals stay local unless
the app sends them; values are opt-in, as for help requests.

**In ferrox-ui:** the observer and the factual events, `report_help_signal`,
and the suggested state on `HelpButton`. Thresholds, wording and the offer
UI live in the app or a webapp-lib adapter (with the assistant adapter
noted in `webapp-lib/plans/crate-split.md`).

## Decisions (2026-10-23)

All as recommended:

1. A topic opens in a popover; "Full guide" covers long reading.
2. Help lives in `core`, always on.
3. Context is free-form key/value pairs, with recommended keys (`domain`,
   `screen`, `section`, `entity`).
4. No "ask anywhere": a `?` appears only where a topic is given.
5. Field values are opt-in per component (`help_shares_value`), and never
   offered for passwords.
6. The topic registry comes later.
