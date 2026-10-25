# help follow-ups: `?` only while active, F1, and noticing when help is needed

The three follow-ups in `plans/help.md` (proposed 2026-10-23 and
2026-10-24), designed together, since they touch the same `?`. Order
chosen by the user 2026-10-25: these before webapp-lib piece 2.

What exists: `HelpButton` (`src/core/help_button.rs`) is a `?` with a
browser popover; components place one through `component_help` for their
`help` prop (Field, Checkbox, Switch, Dialog, ConfirmDialog, Sheet, Panel,
Toolbar); `HelpScope` adds context and `HelpAssistant` an "Ask about
this", both through Leptos context. Everything is in `core`, no feature.

## A. `?` only while the control is active

**The need:** a form with help on every field shows a column of `?`s.
The developer chooses, per control or per scope, to show a control's `?`
only while that control is in use.

**API.**

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HelpReveal {
    #[default]
    Always,
    /// Shown while the control holds focus or the pointer, or its help is open.
    WhileActive,
}
```

- `help_reveal: Option<HelpReveal>` on **Field, Checkbox, Switch** only:
  the controls that repeat down a form. Dialog, ConfirmDialog, Sheet,
  Panel and Toolbar keep their `?` always shown: there's one per screen
  part, and it's the main one (the user's rule, 2026-10-24).
- `HelpScope reveal=HelpReveal::WhileActive` sets the default for the
  controls inside it; a control's own prop wins. Unset everywhere:
  `Always`, so nothing changes for existing apps.

**What "active" means:** the control's own container (the Field's `div`,
the Checkbox's and Switch's wrapper) matches `:focus-within`, or `:hover`
on a device that hovers (`@media (hover: hover)`), or its `?` has
`aria-expanded="true"`. All CSS: the container gets `.help-reveal` and
its `?` `.help-btn--reveal`; there's no script.

**Hidden, not gone.** The `?` is `opacity: 0` (not `display: none` or
`visibility: hidden`), so:
- it keeps its place: nothing shifts when it appears;
- it stays in the Tab order (unless `help_skip_tab`), and focusing it
  makes its container `:focus-within`, so it appears as it's reached. In a
  Field the `?` comes before the input, so Tab lands on it first and shows
  it;
- a screen reader still finds it;
- touch: tapping the input focuses it, which reveals the `?`.
- F1 (part B) still finds it.
- `prefers-reduced-motion`: no fade.

**Tests (e2e):** hidden at rest and shown on focus, on hover, while open;
Tab reaches it and shows it; the scope default and the control's override;
`Always` unchanged; Dialog's `?` unaffected. Demo: a form on `/help`
with a scope set to `WhileActive`.

## B. F1 and Shift+F1

**The need:** from the keyboard, open the help of what has focus without
finding its `?`; most useful where the `?` uses `skip_tab`.

**API:** `<HelpKeys>children</HelpKeys>`, opt-in, since it takes over a
key; placed once, around the app (like `HelpAssistant`, and independent of
it: F1 needs no assistant).

- **F1** opens the help of the focused control: from
  `document.activeElement`, the closest `[data-help]` names its `?` by id;
  F1 clicks it (opening its popover, which takes focus, as a click does).
  The innermost wins: a field inside a dialog opens the field's help. No
  `[data-help]` found: nothing happens, and the browser's own F1 is still
  stopped (inside `HelpKeys`, F1 is ours).
- **Shift+F1** asks the assistant directly: it clicks the `?`'s "Ask about
  this" (already in its popover, closed). Without an assistant it acts as
  F1.
- Every component that places a `?` puts `data-help="<the ?'s id>"` on its
  own container; `HelpButton` gains a stable `id`. A bare `HelpButton`
  isn't found by F1 unless the app puts `data-help` on its container (a
  documented attribute, a public `HelpButton` `id` prop).
- Inside `HelpKeys`, every `?` gets `aria-keyshortcuts="F1"` (HelpKeys
  provides a context the `?` reads); outside, none, since the key wouldn't
  work.
- `preventDefault` on the keydown: e2e checks the event is
  default-prevented in Chromium and Firefox. On macOS F1 is often a
  brightness key; F1 is a shortcut on top of the visible `?`, never the
  only way in.
- The listener is on `document`, added on mount and removed on cleanup.

**Tests (e2e):** F1 in a Field opens its topic and focus moves into it; in a
field inside a dialog, the field's; Esc returns focus to the `?`; Shift+F1
sends the request (the demo's fake assistant); without an assistant,
Shift+F1 opens the topic; outside `HelpKeys`, F1 is untouched and no
`aria-keyshortcuts`; with `help_skip_tab` and `WhileActive`, F1 still
works.

## C. Noticing when a user needs help

As designed in `plans/help.md` (three layers: components report facts,
the app reports domain signals, a policy outside ferrox-ui decides). Here
is ferrox-ui's part.

**The observer.** `<HelpSignals on_signal=…>children</HelpSignals>`,
scoped like `HelpAssistant`. Without it nothing is recorded or sent:
every report is a no-op.

```rust
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct HelpSignal {
    pub kind: HelpSignalKind,
    /// What reported it ("field", "dialog", …), and its label or title.
    pub component: String,
    pub about: String,
    /// The topic of the `?` on that component, if it has help.
    pub topic_id: Option<String>,
    /// The enclosing HelpScopes' context, outermost first.
    pub context: Vec<(String, String)>,
    /// Only what the app passes (`report`), or a value opted in.
    pub detail: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum HelpSignalKind {
    ErrorShown,
    ErrorCleared,
    /// The same error through another edit; `edits` since it appeared.
    ErrorPersisted { edits: u32 },
    /// The affirm action was tried (click or Enter) while not ready.
    SubmitRefused,
    HelpOpened,
    /// Closed within a second of opening.
    HelpDismissed,
    /// Left with text that matched nothing; `times` in a row.
    NoMatch { times: u32 },
    /// From the app (`report`), with its own kind.
    App(String),
}
```

**Who reports what:**
- **Field:** `ErrorShown`, `ErrorCleared`, `ErrorPersisted` (counting edits
  of its control while the error stays; needs the control's value, which
  Field already registers for `help_shares_value`).
- **Dialog / ConfirmDialog:** `SubmitRefused` when the affirm action is
  pressed or Enter submits while it's disabled or saving.
- **HelpButton:** `HelpOpened`, `HelpDismissed`.
- **Combobox:** `NoMatch` on blur with unmatched text.
- **The app:** `let signals = help_signals();` in a component returns a
  handle; `signals.report("imbalance", Some(detail))` sends
  `HelpSignalKind::App`, with the context where the handle was made.
  *Deviation from `plans/help.md`:* a handle, not a free
  `report_help_signal(kind, detail)`, because Leptos context is read in a
  component body, not reliably in an event handler later.

**The suggested state.** A policy (in the app or webapp-lib) offers help
without interrupting: `signals.suggest(topic_id)` marks every `?` with
that topic as suggested: `data-suggested` on it, a theme-styled ring
(`help.css`), and its label gains ", suggested" for screen readers.
Opening it, or `signals.clear(topic_id)`, ends it. Nothing auto-opens and
focus never moves. A "Need a hand…?" link, wording and thresholds stay in
the app.

**Privacy:** signals stay in the page; only what `on_signal` does with
them leaves it. Values are never included unless the component's
`help_shares_value` is on (never for passwords), as for requests.

**Tests:** unit tests for `HelpSignal`; e2e on `/help` with a signals log in
the demo: each kind from its component, context from scopes, no
reporting outside `HelpSignals`, the suggested state set, announced and
cleared by opening.

## Docs, for every piece

`docs/help.md` (and the pages of the components that gain a prop) gets the
new API with a compiled example; `docs/index.md` is regenerated;
`CHANGELOG.md`; the demo's `/help` page. The drift check needs no new
rule. `plans/help.md` points here, and the plan status in
`plans/ferrox-ui.md` moves on.

## Pieces

Each passes `scripts/release-check.sh` and is reviewed on its own.

1. **A: `HelpReveal`.** The enum, the prop on Field/Checkbox/Switch, the
   `HelpScope` default, CSS, demo, e2e, docs.
   *Done 2026-10-25:* `HelpScope`'s `context` became optional (a scope
   may only set `reveal`). The container's class is folded into its
   class string (`with_reveal`), not a `class:` directive, which a
   reactive `class` string can overwrite. Demo: "Only while in use" on
   `/help`; 5 e2e tests per browser; release check passed (639 e2e).
2. **B: `HelpKeys`.** `data-help` and the `?`'s id on every component with
   help, `HelpKeys` (F1, Shift+F1, `aria-keyshortcuts`), demo, e2e, docs.
   *Done 2026-10-25:* Esc after F1 gives focus back to the *control*
   (the browser restores what had focus before the popover showed), not
   to the `?` as written above: better, since that's where the user was.
   F1 inside an open topic does nothing. `component_help` returns the
   `?`'s id with its view. The demo's `/help` page is inside `HelpKeys`;
   `/layout` is the outside case, checked with a synthetic keydown (a
   real F1 there runs the browser's help, which aborts Firefox's font
   loads). Added to `docs/choosing.md` (and so AGENTS.md): HelpReveal and
   HelpKeys rows.
3. **C1: the observer.** `HelpSignals`, `HelpSignal`/`HelpSignalKind`,
   `help_signals()` and `report`, `HelpOpened`/`HelpDismissed` from
   HelpButton; the demo's signals log; e2e, docs.
   *Done 2026-10-25:* the handle is `HelpReporter` (also
   `is_observed()`); `HelpSignalKind::name()` gives stable names. The
   `?` reports from `beforetoggle`, not `toggle`: Chromium merges an open
   and a close that come before its `toggle` into one closed-to-closed
   event. And `beforetoggle` is attached as a `Custom` event: Leptos
   0.8 (tachys 0.2.19) lists it as bubbling, so `on:beforetoggle`
   listens on the document and never fires.
4. **C2: the components' facts.** Field's error kinds, Dialog's
   `SubmitRefused`, Combobox's `NoMatch`; e2e, docs.
   *Done 2026-10-25:* Field counts committed edits (`change`, on its
   container: no control value needed, so passwords too), and only an
   edit that leaves an error already there (at focus or the last change):
   the edit that makes an error isn't a failed fix. No report for an
   error present at mount. `SubmitRefused`: Enter while not ready (its
   default kept, as before), or a keyboard press on the saving affirm
   button through Button's new `on_refused` (a saving button has
   `pointer-events: none`, and a disabled one gets no click at all).
   Combobox takes its Field's label and topic (`signal_about`). The
   demo's `Trunk.toml` now watches the library crates: Trunk ignores a
   `touch` that changes no content, so library edits never rebuilt it.
5. **C3: the suggested state.** `suggest` / `clear`, `data-suggested`,
   CSS, the label; e2e, docs.
   *Done 2026-10-25:* the app owns the suggested topics: a
   `HelpSuggestions` it makes and passes to `HelpSignals`, since a policy
   lives beside `HelpSignals`, outside it, where `help_signals()` finds no
   observer. A component inside can suggest through its reporter too. A
   suggested `?` also shows where `WhileActive` hides it. Its words are
   `suggested_label`. All five pieces done.

## After these (the user, 2026-10-25)

The demo's side menu on a phone (Firefox on Android): scrolling the menu
to reach its last entries scrolls the page behind instead. Expected: only
the menu scrolls; the page never moves. Fix in `Sheet` (so every sheet and
`SideMenu` gets it): `overscroll-behavior: contain` on its body, a height
that follows the visible viewport (`100dvh`), and the scroll lock on
`<html>` as well as `<body>`; e2e on a phone-sized viewport.
*Done 2026-10-25:* as written, and the dialog body contains its overscroll
too. Headless Chromium can't show the touch bug itself; the e2e checks the
rules are in force and that wheeling the menu to its end leaves the page
at 0. Not yet tried on the user's phone.

## Open points (all approved as recommended, 2026-10-25)

1. **"Active" includes hover** (on devices that hover), recommended:
   pointing at a field to see if it has help is natural. The alternative,
   focus only, is quieter but hides help from someone reading the form
   before filling it in.
2. **`WhileActive` only on the controls** (Field, Checkbox, Switch),
   recommended, per the user's "main component stays shown". Alternative:
   on every component with `help`.
3. **`HelpKeys` as its own wrapper**, recommended: F1 works without an
   assistant. Alternative: a `keys` prop on `HelpAssistant`.
4. **`ErrorPersisted` on every edit, with the count**, recommended: the
   policy picks its threshold. Alternative: ferrox-ui fires once at a
   fixed N, which puts policy into the library.
5. **The signals handle** (`help_signals()`) rather than a free function,
   recommended (above).
