# Help (in `core`)

Any component with a label or a title can carry help. Give it a topic and a
discreet `?` appears beside the label. It opens the topic in a popover: a
short, structured explanation that works offline. Inside a `HelpAssistant`,
the popover also offers **Ask about this**, which hands the app a
`HelpRequest`: the topic, which component, its label, and the context the app
declared around it. The app sends that to its chat, a model, or a memory
store. ferrox-ui never calls a model.

**When to give help:** one topic per thing that can carry a `?`, and only
where a user would ask. A screen of `?`s is noise. A topic too small to stand
on its own means the `?` belongs on the thing around it.

## `HelpTopic`

The explanation, as structured data (not Markdown or HTML, so nothing parses
a string or trusts tags in one). The app owns the wording and its
translation. It's cheap to clone.

<!-- example: help-topic -->
```rust
use std::sync::LazyLock;

use ferrox_ui::HelpTopic;

static SPLIT_AMOUNT: LazyLock<HelpTopic> = LazyLock::new(|| {
    HelpTopic::new("split-amount", "Amounts on a split") // id, title
        .paragraph("Each line moves money into or out of one account.")
        .list(["Positive: money comes in", "Negative: money goes out"])
        .table(["Sign", "Means"], [["+", "in"], ["-", "out"]])
        .guide("/help#split-amount") // "Full guide"
});
```
<!-- /example -->

The `id` is stable: an assistant and a guide find the topic by it. The
`title` names the `?`.

## The `help` prop

| Component | Where the `?` sits | Also |
|---|---|---|
| `Field` | right after the label, outside the `<label>` | `help_skip_tab`, `help_shares_value` |
| `Checkbox` | after its words, outside the `<label>` (clicking it never ticks the box) | `help_skip_tab`, `help_shares_value` ("true" / "false") |
| `Switch` | beside the switch, not inside it | `help_skip_tab` |
| `Toolbar` | right after the title | |
| `Dialog`, `ConfirmDialog`, `Sheet` | in the title bar, just before the × | |
| `SideMenu` | in the title bar, just before the × | a request names it as a `sheet` |
| `Panel` | right after the title (after the toggle, on a collapsible panel) | |

<!-- example: help-prop -->
```rust
use ferrox_ui::{Field, Input};

view! {
    <Field label="Amount" help=SPLIT_AMOUNT.clone()>
        <Input value=amount />
    </Field>
}
```
<!-- /example -->

`help` takes a `MaybeHelp`, which converts from a `HelpTopic` or an
`Option<HelpTopic>`. `help_skip_tab` keeps the `?` out of the Tab order for a
field in a row (it stays clickable and reachable by a screen reader).

## `HelpReveal`: only while in use

A form with help on every field shows a column of `?`s. With
`HelpReveal::WhileActive`, a control's `?` shows only while the control
holds focus or the pointer, or its topic is open. Set it on a `HelpScope`
for every Field, Checkbox and Switch inside, or on one control with
`help_reveal`; the control's own wins.

<!-- example: help-reveal -->
```rust
use ferrox_ui::{Field, HelpReveal, HelpScope, Input};

view! {
    <HelpScope reveal=HelpReveal::WhileActive>
        <Field label="Payee" help=PAYEE.clone()>
            <Input value=payee />
        </Field>
        <Field label="Category" help=CATEGORY.clone() help_reveal=HelpReveal::Always>
            <Input value=category />
        </Field>
    </HelpScope>
}
```
<!-- /example -->

- A hidden `?` is transparent, not removed: nothing moves when it appears,
  Tab still reaches it (and reaching it shows it), a screen reader finds it,
  and on touch, tapping the control shows it.
- Only the controls that repeat down a form take it. A Dialog's, Sheet's,
  Panel's or Toolbar's `?` is always shown: it's the screen part's main help.
- Default: `HelpReveal::Always`, so nothing changes until you ask.

## `HelpButton`

The `?` on its own, for anywhere else: beside a heading, in a table header.
The components above place one for you.

<!-- example: help-button -->
```rust
use ferrox_ui::HelpButton;

view! {
    <h2>"Statement balance" <HelpButton topic=STATEMENT_BALANCE.clone() /></h2>
}
```
<!-- /example -->

- **It names what it explains:** "Help: {title}", as its label and tooltip,
  so a screen reader knows which `?` it landed on.
- The popover is the browser's own: Esc or a click elsewhere closes it, one
  is open at a time, and Esc gives focus back to the `?`. It sits below the
  `?` (above when there's no room), stays inside the viewport, and follows
  the page as it scrolls.
- Props: `topic`, `skip_tab`, `kind` / `about` / `value` (what a request says
  carries the `?`; components set these), `id` (for F1: see `HelpKeys`), `help_label` (`"Help"`),
  `guide_label` (`"Full guide"`), `ask_label` (`"Ask about this"`), `class`.
- Put it where block content is allowed: not inside a `<label>` or a
  `<button>`.

## `HelpScope`: context for an assistant

Declare what an assistant should know where you know it. Scopes nest; a
request gathers them outermost first. A scope reaches only its own children.

<!-- example: help-scope -->
```rust
use ferrox_ui::{Field, HelpScope, Input};

view! {
    <HelpScope context=[("domain", "bookkeeping")]>
        <HelpScope context=[("screen", "reconcile")]>
            <HelpScope context=move || vec![
                ("entity".to_string(), "account".to_string()),
                ("account_id".to_string(), account.get()),
            ]>
                <Field label="Statement balance" help=STATEMENT_BALANCE.clone()>
                    <Input value=balance />
                </Field>
            </HelpScope>
        </HelpScope>
    </HelpScope>
}
```
<!-- /example -->

`context` is an array or a `Vec` of pairs, or a closure returning them for
context that follows the page. It's read when the user asks, so it's always
current.

**Recommended keys.** The keys are free-form; these names let an assistant
rely on them:

| Key | Meaning | Example |
|---|---|---|
| `domain` | the app or subject area | `bookkeeping` |
| `screen` | the screen the user is on | `reconcile` |
| `section` | a part of that screen | `splits` |
| `entity` | the kind of thing in view | `transaction` |
| `<entity>_id` | its id, so a tool can fetch it | `transaction_id` |

Put `domain` in a scope at the app's root, and `screen`, `section` and ids
where they are known. Stable mappings can live in the app's `on_ask`
handler instead. A rule of thumb: the page declares what only the page
knows, such as the current entity's id.

Because the keys are free-form, an app (or the adapter that connects it to
its assistant) can add its own, such as routing hints for which tools an
assistant should try. Document those with the app or adapter, not here.
Treat any such key as a **hint**: what a model may do is decided by the
assistant's client, never by the page.

## `HelpAssistant` and `HelpRequest`

Wrap the app (or a part of it). Every help popover inside offers "Ask about
this".

<!-- example: help-assistant -->
```rust
use ferrox_ui::{HelpAssistant, HelpRequest};

view! {
    <HelpAssistant on_ask=move |request: HelpRequest| ask_assistant(request)>
        <p>"The app: every help popover inside offers \"Ask about this\"."</p>
    </HelpAssistant>
}
```
<!-- /example -->

| `HelpRequest` field | |
|---|---|
| `topic_id`, `topic_title` | the topic |
| `component` | what carries the `?`: `field`, `checkbox`, `switch`, `toolbar`, `dialog`, `alertdialog`, `sheet`, `panel`, or `help` for a bare `HelpButton` |
| `label` | that component's label or title |
| `context` | every enclosing scope's pairs, outermost first |
| `value` | the control's value, only when the component opted in |

`to_prompt_context()` renders it as plain lines for a prompt:

```text
topic: split-amount (Amounts on a split)
component: field
label: Amount
screen: reconcile
account_id: checking
```

`HelpRequest::new(&topic, component, label)` with `.with_context(…)` and
`.with_value(…)` builds one, for testing a handler. Without a
`HelpAssistant`, help is just the topics; nothing else changes.

## `HelpKeys`: F1 and Shift+F1

Wrap the app once. F1 opens the help of whatever has focus; Shift+F1 asks
the assistant about it directly (without a `HelpAssistant`, it opens the
help, as F1).

<!-- example: help-keys -->
```rust
use ferrox_ui::HelpKeys;

view! {
    <HelpKeys>
        <p>"The app: F1 opens the focused control's help, Shift+F1 asks about it."</p>
    </HelpKeys>
}
```
<!-- /example -->

- **The innermost help wins.** Every component with a `help` prop marks
  itself (`data-help`, naming its `?`), and F1 takes the one closest to the
  focused element: in a field inside a dialog, the field's; elsewhere in
  the dialog, the dialog's.
- It opens the topic as a click would, and Esc gives focus back to the
  control you were on.
- A `?` that's hidden (`HelpReveal::WhileActive`) or out of the Tab order
  (`help_skip_tab`) still answers: that's where F1 helps most.
- Inside, every `?` says its shortcuts (`aria-keyshortcuts`: `F1`, plus
  `Shift+F1` with an assistant), and the browser's own F1 never runs, even
  where there's no help to open. Outside, F1 is the browser's.
- A shortcut on top of the visible `?`, never the only way in: on macOS F1
  is often a brightness key.

A bare `HelpButton` is found the same way once what it explains names it:

<!-- example: help-keys-bare -->
```rust
use ferrox_ui::HelpButton;

view! {
    <div data-help="balance-help">
        <input aria-label="Statement balance" />
        <HelpButton topic=STATEMENT_BALANCE.clone() id="balance-help" />
    </div>
}
```
<!-- /example -->

## `HelpSignals`: noticing when help is needed

Components report facts that may mean the user is stuck; the app decides
what they mean. Wrap the app once; without it, nothing is recorded.

<!-- example: help-signals -->
```rust
use ferrox_ui::{HelpSignal, HelpSignals};

view! {
    <HelpSignals on_signal=move |signal: HelpSignal| observe(signal)>
        <p>"The app: what its components report arrives in on_signal."</p>
    </HelpSignals>
}
```
<!-- /example -->

Each `HelpSignal` has its `kind`, what reported it (`component`, `about`),
the `topic_id` of its `?` if it has help, the enclosing scopes' `context`,
and a `detail` only the app sets. `kind.name()` is a stable name for a
policy or a log:

| `HelpSignalKind` | `name()` | Reported by |
|---|---|---|
| `HelpOpened` | `help_opened` | a `?`, as its topic opens |
| `HelpDismissed` | `help_dismissed` | a `?` closed within a second (not by asking) |
| `App(kind)` | the app's `kind` | the app, through `help_signals()` |

The app's own signals are the strongest: only it knows that an entry won't
balance. Make a reporter in a component's body (that's where it finds the
observer and the scopes), and report from anywhere later:

<!-- example: help-signals-report -->
```rust
use ferrox_ui::{help_signals, Button};

let signals = help_signals();
view! {
    <Button on_click=move || signals.report("imbalance", Some(difference()))>"Check"</Button>
}
```
<!-- /example -->

- **Facts, not judgments.** Nothing here guesses from dwell time or
  clicks. Thresholds, cool-downs and the offer itself belong to the app's
  `on_signal`.
- Signals stay in the page; only what `on_signal` does with them leaves it.

## Privacy

A request carries only what the developer declared: the topic, the
component, its label, and scope context. A control's **value** is sent only
when that component says `help_shares_value=true`. `PasswordInput` never
offers its value, so a password can't be sent even then. Keep ids and
private details out of `HelpScope` unless the assistant should see them.
