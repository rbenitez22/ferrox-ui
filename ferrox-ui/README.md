# ferrox-ui

UI primitives for Leptos CSR apps. Components come in feature groups (`core`
always, `overlay` and more by feature), and they're styled by class-only CSS
on [ferrox-theme](../ferrox-theme)'s tokens, so every theme reaches every
component. Nothing is styled unless it carries a
class: no element selectors, no global pseudo-classes.

Part of the [ferrox-ui workspace](../README.md). Apps using webapp-lib reach
it as `webapp_lib::ui` and don't list it themselves.

## Setup

```toml
[dependencies]
ferrox-ui = { git = "https://github.com/rbenitez22/ferrox-ui", tag = "v0.1.0", features = ["overlay", "forms"] }
```

```rust
fn main() {
    ferrox_theme::init();      // the theme (its CSS is linked in index.html)
    ferrox_ui::init();         // the components' CSS, for every compiled group
    leptos::mount::mount_to_body(App);
}
```

`init()` adds one `<style id="ferrox-ui-…">` per sheet to `<head>`. A second
call adds nothing.

### Groups

| Group | Feature | Contains |
|---|---|---|
| `core` | always on | `Button`, `IconButton`, `Toolbar`, `ToolbarGroup`, `Spinner`, `LoadingOverlay`, `Alert`; help: `HelpTopic`, `HelpButton`, `HelpScope`, `HelpAssistant`, `HelpRequest` |
| `overlay` | `overlay` | `Dialog`, `DialogAction`, `ConfirmDialog`, `Sheet` |
| `forms` | `forms` | `Field`, `Input`, `Textarea`, `PasswordInput`, `Checkbox`, `Switch`, `Select`, `Combobox` |
| `navigation` | `navigation` (turns on `overlay`; brings leptos_router) | `NavList`, `NavLink`, `NavGroup`, `SideMenu`, `SideMenuButton` |
| `layout` | `layout` | `Panel` (also collapsible), `PanelTone`, `HeadingLevel`; `Card` coming (`plans/layout.md`; documented with the group's last piece, see the rustdoc meanwhile) |
| `data`, `menus` | one each, planned | see `plans/ferrox-ui.md` |

### Conventions (every component)

- `class`: extra classes, appended to the component's own. A string, or a
  `Signal<String>` to make it reactive. A bare closure doesn't convert to
  `MaybeProp<String>`, so wrap it in `Signal::derive`.
- No `style` prop. Styling goes through classes and tokens.
- Enums (`ButtonFamily`, `SpinnerKind`, `AlertSeverity`, …) are
  `#[non_exhaustive]`: match them with a `_` arm.
- Each component has a `*_classes` function that returns the class list it
  renders, for markup that needs the same look without the component.
- User-facing text (button labels, "Close", "Cancel", "Dismiss") is a prop
  with an English default.
- A callback with no input is a zero-argument closure:
  `on_close=move || open.set(false)`.
- Components with a label or a title take `help` (a `HelpTopic`): see
  [Help](#help-in-core).

## core

### `Button`

A padded button (`.btn`). The **family** is what the button means. **Quiet**
is a separate axis: how heavy it looks.

```rust
use ferrox_ui::{Button, ButtonFamily, ButtonSize};

view! {
    <Button on_click=move |_| save()>"Save"</Button>
    <Button family=ButtonFamily::Cancel on_click=move |_| close()>"Cancel"</Button>
    <Button family=ButtonFamily::Secondary quiet=true size=ButtonSize::Sm>"Export"</Button>
    <Button saving=saving button_type="submit">"Create"</Button>
}
```

| Prop | Default | |
|---|---|---|
| `family` | `Primary` | `Primary` (affirmative), `Secondary` (committing, not the main action), `Cancel` (dismissive, always an outline), `Danger` (destructive) |
| `quiet` | `false` | outline instead of fill |
| `size` | `Md` | `Sm`, `Md`, `Lg` |
| `saving` | `false` | a request is running: `aria-disabled` and a sweeping border. The button stays focusable and refuses clicks. It never sets `disabled`. |
| `disabled` | `false` | |
| `button_type` | `"button"` | not HTML's `"submit"`; pass `"submit"` for a form's submit button |
| `on_click`, `title`, `class` | | |

`button_classes(family, quiet, size, saving, extra)` returns the class list.

### `IconButton`

A compact icon-only button (`.icon-btn`): 1.75rem square, no border. It
answers hover and press in colour, not size.

```rust
use ferrox_icons::{Icon, CAMERA, DELETE, SEARCH};
use ferrox_ui::IconButton;

view! {
    <IconButton title="Search" on_click=move |_| open_search()><Icon icon=SEARCH/></IconButton>
    <IconButton title="Camera" pressed=camera on_click=move |_| camera.update(|c| *c = !*c)><Icon icon=CAMERA/></IconButton>
    <IconButton title="Remove line" danger=true on_click=move |_| remove()><Icon icon=DELETE/></IconButton>
}
```

| Prop | |
|---|---|
| `title` | required: the button's only label (also `aria-label`) |
| `pressed` | makes it a switch (`aria-pressed`); on reads as tinted |
| `danger` | for a destructive action: drawn in the danger colour, tinted on hover |
| `disabled`, `on_click`, `class` | |
| children | the icon: ferrox-icons' `Icon`, or any `<svg class="icon">` |

`icon_button_classes(danger, extra)` returns the class list.

### `Toolbar`, `ToolbarGroup`

A row of buttons. Buttons sit at the right, and an optional `title` takes the
free space on the left. `compact` tightens the row for icon buttons and small
buttons. Groups are split by a hairline.

```rust
use ferrox_ui::{Toolbar, ToolbarGroup};

view! {
    <Toolbar title="Accounts">
        <Button on_click=move |_| add()>"New account"</Button>
    </Toolbar>

    <Toolbar compact=true>
        <ToolbarGroup>
            <IconButton title="Search"><Icon icon=SEARCH/></IconButton>
            <IconButton title="Camera" pressed=camera><Icon icon=CAMERA/></IconButton>
        </ToolbarGroup>
        <ToolbarGroup>
            <IconButton title="Help"><Icon icon=HELP/></IconButton>
        </ToolbarGroup>
    </Toolbar>
}
```

`Toolbar` takes `title`, `compact`, `help` (a `?` right after the title) and
`class`; `ToolbarGroup` takes `class`.
`toolbar_classes(compact, extra)` returns the class list.

### `Spinner`, `LoadingOverlay`

Choose a spinner by **what stays usable while the work runs**, not by how
long it takes:

| Use it when | Use |
|---|---|
| nothing on screen is valid any more: boot, resolving auth, the post-login redirect. Never a route change or a data fetch. | `LoadingOverlay` |
| any committing action | `Button saving=…` |
| a pane fetching data whose shape it can't promise | `Spinner` (region, the default) |
| an icon-only row action; the spinner takes the icon's place | `Spinner kind=SpinnerKind::Inline` |
| async work that belongs to one field (lookup, typeahead) | `Spinner kind=SpinnerKind::Field` |

Use one indicator per action: a saving button never also raises an overlay.

Every spinner is transparent until `--loading-delay` (400ms), so work that
finishes faster never shows one. Under `prefers-reduced-motion` the spin slows
to 2.4s but doesn't stop; a still spinner reads as "hung".

```rust
use ferrox_ui::{LoadingOverlay, Spinner, SpinnerKind};

view! {
    <Spinner label="Loading accounts"/>
    <Spinner kind=SpinnerKind::Inline label="Deleting the row"/>
    {move || signing_in.get().then(|| view! { <LoadingOverlay text="Signing in…"/> })}
}
```

- `Spinner`: `label` (required; read out by screen readers, not shown),
  `kind` (`Region` by default, `Inline` or `Field`), `class`. It renders
  `role="status"`.
- `LoadingOverlay`: `text` (optional; shown under the spinner and read out;
  without it screen readers hear "Loading…"). A fixed scrim over the whole
  page, `role="status"` with `aria-live="polite"`.

`spinner_status_classes(kind, extra)` returns the wrapper's class list.

### `Alert`

An in-page or in-form message that **never times out**: the reason a submit
failed, a server refusal no single field owns. Messages that go away by
themselves are toasts (planned, `overlay`). Anything the user must act on is
an alert.

```rust
use ferrox_ui::{Alert, AlertSeverity};

view! {
    <Alert severity=AlertSeverity::Error>"The server refused the change: the period is closed."</Alert>
    <Alert severity=AlertSeverity::Info icon=false>"Changes apply to new entries only."</Alert>
    {move || error.get().map(|e| view! {
        <Alert severity=AlertSeverity::Error on_dismiss=move |_| error.set(None)>{e}</Alert>
    })}
}
```

| Prop | Default | |
|---|---|---|
| `severity` | required | `Info`, `Success`, `Warning` (non-blocking: if it should stop a submit, it's an `Error`), `Error` |
| `icon` | `true` | draws the severity's icon (ferrox-icons `INFO`, `SUCCESS`, `WARNING`, `ERROR`), so severity doesn't rest on colour alone |
| `on_dismiss` | none | shows a close button that calls it; the caller removes the alert |
| `dismiss_label` | `"Dismiss"` | the close button's label |
| `class` | | |

Only `Error` interrupts a screen reader (`role="alert"`); the others are
`role="status"`. There's no primary alert, because an alert reports a state
and doesn't act. `alert_classes(severity, extra)` returns the class list, and
`AlertSeverity` has `class()`, `icon()` and `role()`.

## overlay (feature `overlay`)

Modals built on the browser's own `<dialog>`, opened with `showModal()`. The
browser makes the page behind inert, keeps focus inside, and puts the modal in
the top layer, above everything including `LoadingOverlay`. All three share
the same behaviour:

- **`open` drives it.** Every way out (Esc, the ×, the dismiss button, a
  sheet's backdrop) calls `on_close` / `on_cancel`, and the app sets `open`
  to false. If the browser closes the element itself, the callback runs too,
  so the app's state and the element never disagree.
- **Focus** goes to an element marked `autofocus`, otherwise the first
  control (the ×). On close it returns to whatever opened the modal.
- **The page doesn't scroll** while a modal is open (`<body>` gets
  `.dialog-open`). This is counted, so a confirm opened from a dialog keeps
  the page locked until both close.
- **The body is mounted only while open**, so each opening starts fresh.

| Use it when | Use |
|---|---|
| a task or a form with a clear finish | `Dialog` |
| a destructive action needs a decision (every one does: no "undo toast" instead) | `ConfirmDialog` |
| navigation, details or filters that slide in from an edge | `Sheet` |

**Watch your descendant selectors.** A `<dialog>` stays where it's declared
in the DOM, so an app rule like `.section h2 { margin: … }` also reaches the
dialog's title. Scope such rules to direct children (`.section > h2`) where
dialogs are declared.

### `Dialog`, `DialogAction`

A title bar with the ×, the body, and a footer band. **The footer isn't a
slot.** Pass the actions and the component fixes the order: dismiss,
alternate, affirm, with the heaviest button last.

```rust
use ferrox_ui::{Dialog, DialogAction, DialogSize};

let editing = RwSignal::new(false);
let saving = RwSignal::new(false);

view! {
    <Button on_click=move |_| editing.set(true)>"Edit account…"</Button>
    <Dialog
        open=editing
        title="Edit account"
        on_close=move || editing.set(false)
        affirm=DialogAction::new("Save", move || save()).saving(saving)
        alternate=DialogAction::new("Save as draft", move || save_draft())
    >
        <label>"Name" <input autofocus=true /></label>
    </Dialog>
}
```

| Prop | Default | |
|---|---|---|
| `open`, `title`, `on_close` | required | `on_close` is called by Esc, the × and the dismiss button: the same act |
| `affirm` | none | the last button; `DialogAction::new(label, fn)`, plus `.danger()` for a destructive commit, `.saving(signal)`, `.disabled(signal)` |
| `alternate` | none | a lighter way to finish, before `affirm`, always secondary; Enter never reaches it |
| `dismiss_label` | `"Cancel"` | |
| `no_dismiss` | `false` | no dismiss button (the × and Esc still close) |
| `submit_on_enter` | `true` | Enter fires `affirm`, but not from a textarea, a button or a link, and not while it's saving or disabled |
| `size` | `Md` | `Md` (32rem) or `Wide` (52rem, for a body that's a table) |
| `alert` | `false` | `role="alertdialog"`, for a dialog that interrupts to ask for a decision |
| `close_label` | `"Close"` | the ×'s label |
| `help` | none | a `?` in the title bar, before the × |
| `class` | | |

A backdrop click doesn't close a Dialog, so a stray click can't throw away
typed input. On a phone (≤ 768px) a dialog is full width inside an 8px
margin, and the × is 44px. `dialog_classes(size, extra)` returns the class
list.

### `ConfirmDialog`

The standard destructive confirmation (`role="alertdialog"`). The confirm
button is always in the danger family and comes last. Its label is the verb
("Delete", "Void", "Archive"), not "OK". The body names exactly what is about
to be lost. Focus opens on the ×, never on the confirm button.

```rust
use ferrox_ui::ConfirmDialog;

view! {
    <ConfirmDialog
        open=confirming
        title="Delete account?"
        confirm_label="Delete"
        saving=deleting
        on_confirm=move || delete_account()     // set `deleting`, close when done
        on_cancel=move || confirming.set(false)
    >
        <p>"This deletes “Groceries” and its 214 transactions."</p>
    </ConfirmDialog>
}
```

Props: `open`, `title`, `confirm_label`, `on_confirm`, `on_cancel` (Cancel,
the × and Esc), `saving` (the confirm button sweeps and refuses clicks),
`cancel_label` (`"Cancel"`), `close_label` (`"Close"`), `help` (a `?` before
the ×).

### `Sheet`

A modal panel pinned to an edge, with a title bar, the × and a scrolling
body. It slides in over `--react-time`; under reduced motion it just appears,
and closing never waits on an animation. A click on the backdrop closes it,
but only if the press also started there, so a text selection dragged out of
the sheet doesn't close it.

```rust
use ferrox_ui::{Sheet, SheetSide};

view! {
    <IconButton title="Menu" on_click=move |_| menu.set(true)><Icon icon=MENU/></IconButton>
    <Sheet open=menu title="Menu" on_close=move || menu.set(false)>
        <nav>…</nav>
    </Sheet>
}
```

| Prop | Default | |
|---|---|---|
| `open`, `title`, `on_close` | required | `title` is shown and is the sheet's accessible name |
| `side` | `Left` | `Left` / `Right`: full height, `min(20rem, 85vw)` wide. `Bottom`: full width, at most 85vh tall |
| `close_label` | `"Close"` | |
| `help` | none | a `?` in the title bar, before the × |
| `class` | | |

`sheet_classes(side, extra)` returns the class list.

## forms (feature `forms`)

A `Field` puts a label, one control and at most one message together. Each
control is bound two-way to an `RwSignal` (`String` for text, `bool` for a
checkbox or switch): the control writes what the user does, and setting the
signal from code updates the control.

**Where a message goes.** A problem one field can fix goes under that field
(`error`). Anything cross-field, or a server refusal no single field owns,
goes in an `Alert` at the top of the form. A problem never goes in both. A
field shows a hint *or* an error, never both: once the field is wrong, the
message under it says what's wrong.

```rust
use ferrox_ui::{Alert, AlertSeverity, Field, Input, InputKind};

let email = RwSignal::new(String::new());
let email_error = Signal::derive(move || {
    let e = email.get();
    (!e.is_empty() && !e.contains('@')).then(|| "Enter an email address, like name@example.com".to_string())
});

view! {
    {move || refused.get().map(|why| view! { <Alert severity=AlertSeverity::Error>{why}</Alert> })}
    <Field label="Email" hint="Where receipts go" error=email_error>
        <Input value=email kind=InputKind::Email autocomplete="email" />
    </Field>
}
```

**The wiring is automatic.** The control inside a `Field` takes the field's
id (so the `<label for>` names it), `aria-describedby` (the message, while
there is one) and, while there's an error, `aria-invalid` and `.is-invalid`.
Call sites pass no ids, and every field on a page gets a unique one. Put
**one control per field**. Only the field's own children see its wiring: a
control after the field, or outside any field, generates its own id and has
no description.

`Input`, `Textarea`, `PasswordInput`, `Select` and `Combobox` go inside a
`Field`. `Checkbox` and `Switch` carry their own label and don't.

| Component | Use it for |
|---|---|
| `Input` | one line of text: a name, an email, a search |
| `Textarea` | text that may run to several lines |
| `PasswordInput` | a password, to sign in or to set one (with a strength meter) |
| `Checkbox` | a yes/no that a later Save sends |
| `Switch` | an on/off that takes effect as it flips: a filter, a preference written straight back |
| `Select` | a short, fixed list of choices |
| `Combobox` | a long list, filtered by typing |

### `Field`

```rust
use ferrox_ui::Field;

view! {
    <Field label="Name" hint="As it appears on statements" error=name_error help=NAME_HELP.clone()>
        <Input value=name autofocus=true />
    </Field>
}
```

| Prop | | |
|---|---|---|
| `label` | required | the `<label>`, pointing at the control |
| `hint` | none | guidance while the field is fine; hidden while there's an error |
| `error` | none | the field's problem. Present and not empty means invalid |
| `help`, `help_skip_tab`, `help_shares_value` | | a `?` after the label (see [the `help` prop](#the-help-prop)) |
| `class` | | appended to `.field` |
| children | | the one control |

The message is tied to the control by `aria-describedby`, so a screen reader
reads it with the control. An error never times out; it clears when the
field is right. `field_classes(extra)` returns the class list.

### `Input`

A one-line text field (`.form-input`).

```rust
use ferrox_ui::{Input, InputKind};

view! {
    <Field label="Name"><Input value=name autofocus=true /></Field>
    <Input value=query kind=InputKind::Search placeholder="Search accounts" />
}
```

| Prop | Default | |
|---|---|---|
| `value` | required | `RwSignal<String>` |
| `kind` | `Text` | `Text`, `Email`, `Search`, `Tel`, `Url`, `Number` (the `type`). Passwords are `PasswordInput` |
| `placeholder`, `name`, `autocomplete` | none | `autocomplete` is the browser hint (`"email"`, `"off"`, …) |
| `required` | `false` | |
| `readonly`, `disabled` | `false` | may be signals |
| `autofocus` | `false` | where a `Dialog` puts focus when it opens |
| `class` | | |

### `Textarea`

A multi-line text field (`.form-textarea`). It's resized by hand
(vertically), and Enter is a newline: a `Dialog` never submits from it.

```rust
use ferrox_ui::Textarea;

view! { <Field label="Notes"><Textarea value=notes rows=6 /></Field> }
```

Props: `value` (required), `rows` (4), `placeholder`, `name`, `required`,
`readonly`, `disabled`, `autofocus`, `class`.

### `PasswordInput`

A password field (`.form-input` inside `.password-input`) with a show/hide
button at its right end. The button is a real `<button>` with one label
("Show password"); `aria-pressed` says whether the password is showing, and
the eye is crossed out while it is.

```rust
use ferrox_ui::PasswordInput;

view! {
    <Field label="Password"><PasswordInput value=password /></Field>

    <Field label="New password" hint="At least four words, or a long mix">
        <PasswordInput value=new_password autocomplete="new-password" strength=true min_bits=70.0 />
    </Field>
}
```

| Prop | Default | |
|---|---|---|
| `value` | required | `RwSignal<String>` |
| `autocomplete` | `"current-password"` | `"new-password"` when setting one, so a password manager offers to generate it |
| `strength` | `false` | a meter under the field while it has text: a four-step bar and a word. The field is described by its hint and the meter together |
| `min_bits` | `80.0` | the entropy that counts as `Strong` (webapp-lib's `min_password_entropy`) |
| `strength_labels` | English | `StrengthLabels::new("Très faible", "Faible", "Moyen", "Fort")`, for another language |
| `show_label` | `"Show password"` | the button's label |
| `placeholder`, `name`, `required`, `disabled`, `autofocus`, `class` | | `class` goes on the input |

The meter's colours come from its level's class, and the bits are on
`data-bits`, not in the text. A help request never carries a password, even
inside a `Field` with `help_shares_value`.

The calculation is public, for checking a password on submit:

```rust
use ferrox_ui::{password_strength, StrengthLevel};

let strength = password_strength(&new_password.get(), 80.0);  // PasswordStrength { bits, level }
if strength.level != StrengthLevel::Strong { /* say so under the field */ }
```

- `password_entropy(&str) -> f64`: length × log2 of the size of the
  character sets used. It counts characters, not bytes ("é" is one).
- `strength_level(bits, min_bits) -> StrengthLevel`: `VeryWeak` under 25
  bits, `Weak` under 40, `Fair` under `min_bits`, then `Strong`.
  `StrengthLevel::class()` is its meter class.
- `password_strength(&str, min_bits) -> PasswordStrength`: both.

The new-password and confirm pair stays in webapp-lib, built from two
`PasswordInput`s.

### `Checkbox`

A checkbox and its words as one `<label>`, so the words are part of the
target. **A checkbox states an intention that a later Save carries out.**
For a setting that takes effect the moment it flips, use a `Switch`.

```rust
use ferrox_ui::Checkbox;

view! {
    <Checkbox checked=recurring label="Repeat every month" hint="On the same day each month" />
    <Checkbox checked=agreed label="I accept the terms" error=terms_error required=true />
}
```

| Prop | | |
|---|---|---|
| `checked` | required | `RwSignal<bool>` |
| `label` | required | the words beside the box |
| `hint`, `error` | none | one message under it, as a `Field`'s (never both); an error also marks the box |
| `help`, `help_skip_tab`, `help_shares_value` | | a `?` after the words, outside the `<label>`, so clicking it never ticks the box. A shared value is `"true"` / `"false"` |
| `name`, `required`, `disabled`, `class` | | |

It's the native checkbox, coloured with `accent-color` and showing the
theme's focus ring. Space ticks it. `checkbox_classes(invalid, extra)`
returns the class list.

### `Switch`

An on/off that **is** the act: a filter over what the screen shows, or a
preference written straight back. A field that a dialog's Save sends stays a
`Checkbox`. It's a `<button role="switch" aria-checked>` with a track, a knob
and the words inside, so the words name it and are part of the target.

```rust
use ferrox_ui::Switch;

view! { <Switch checked=show_archived label="Show archived accounts" /> }
```

Props: `checked` (required, `RwSignal<bool>`), `label` (required), `disabled`
(off and unusable: still loading, or not this user's to change), `help` and
`help_skip_tab` (a `?` beside the switch, not inside it), `class`. A click or
Space flips it, and the knob slides across. `switch_classes(extra)` returns
the class list.

### `Select`

A dropdown of fixed choices: the native `<select>` (`.form-select`) with the
theme's own caret. `value` is the chosen option's `value`, or empty for none.

```rust
use ferrox_ui::{Select, SelectOption};

let currencies = Signal::derive(|| vec![
    SelectOption::new("EUR", "Euro"),
    SelectOption::new("USD", "US dollar"),
]);

view! {
    <Field label="Currency">
        <Select value=currency options=currencies placeholder="Choose a currency" />
    </Field>

    <Field label="Category">
        <Select
            value=category
            options=categories
            loading=categories_loading
            on_refresh=move || reload_categories()
            refreshing=categories_refreshing
        />
    </Field>
}
```

| Prop | Default | |
|---|---|---|
| `value` | required | `RwSignal<String>` |
| `options` | required | `Signal<Vec<SelectOption>>`; `SelectOption::new(value, label)` |
| `placeholder` | `"Select…"` | the first entry: shown while `value` is empty, and can't be chosen |
| `loading` | `false` | the options are on their way: disabled, `aria-busy`, and the placeholder reads `loading_label` (`"Loading…"`) |
| `on_refresh` | none | adds a refresh button beside the select (`refresh_label`, `"Refresh"`) |
| `refreshing` | `false` | a spinner takes the refresh icon's place, labelled `refreshing_label` (`"Refreshing…"`); the button stays put and refuses clicks |
| `name`, `required`, `disabled`, `class` | | `class` goes on the `<select>` |

The `<select>` is rendered once. Only its options and which one is selected
react, so it keeps focus and its choice when the options change. Loading the
options is the caller's job; webapp-lib's `SelectMenu` is an adapter that
fetches them.

### `Combobox<T>`

A text field that filters a list and lets one item be chosen: the ARIA
combobox pattern (`role="combobox"`, a `listbox` of `option`s,
`aria-activedescendant` for the active one). Use it when a `Select` would be
too long to scan.

```rust
use ferrox_ui::{Combobox, Field};

#[derive(Clone)]
struct Account { id: String, name: String }

// `accounts: Signal<Vec<Account>>` is loaded by the caller.
let account_text = RwSignal::new(String::new());
let chosen = RwSignal::new(None::<Account>);

view! {
    <Field label="Account" error=account_error>
        <Combobox
            items=accounts
            key_of=|a: &Account| a.id.clone()
            label_of=|a: &Account| a.name.clone()
            text=account_text
            on_select=move |a: Option<Account>| chosen.set(a)
            placeholder="Type to search"
            on_refresh=move || reload_accounts()
            refreshing=accounts_refreshing
        />
    </Field>
}
```

| Prop | Default | |
|---|---|---|
| `items` | required | `Signal<Vec<T>>`, owned by whatever loaded them; `T: Clone + Send + Sync + 'static` |
| `key_of` | required | a stable id per item (`data-key` on its option) |
| `label_of` | required | the text shown, and the text matched against |
| `text` | required | `RwSignal<String>`: what's in the field. Choosing an item sets it to the item's label |
| `on_select` | required | `Some(item)` when one is chosen; `None` when the text is cleared, or the field is left with text that names no item |
| `placeholder`, `name`, `required`, `disabled`, `autofocus`, `class` | | `class` goes on the input |
| `clearable` | `true` | a × that clears the text and the choice (`clear_label`, `"Clear"`) |
| `toggleable` | `true` | a chevron that shows and hides the whole list (`toggle_label`, `"Show all"`) |
| `on_refresh` | none | adds a refresh control (`refresh_label`, `"Refresh"`) |
| `refreshing` | `false` | a spinner in the refresh control's place (`refreshing_label`, `"Refreshing…"`) |
| `empty_label` | `"No matches"` | shown in the open list when nothing matches |

How it behaves:

- **Typing** filters by `label_of`, case-insensitive "contains"
  (`matches_text(label, text)` is that test, public).
- **Keys:** ↓ / ↑ move the active option, opening the list if it's closed,
  and stop at the ends. Enter chooses the active option. Tab chooses it and
  moves on. Esc closes the list, or clears the text if it's already closed.
  Inside a `Dialog`, Enter on an option chooses it without submitting, and
  Esc closes the list without closing the dialog.
- **A click** on an option chooses it.
- **Its controls** (clear, the chevron, refresh) are buttons out of the Tab
  order, so a row of comboboxes costs no extra keystrokes. Pressing one
  doesn't take focus from the field.
- **The list is in the top layer** (a popover), placed under the field and
  as wide as it, and it follows the field as the page scrolls. A `Dialog`
  body or any `overflow` parent can't clip it.

It doesn't fetch. webapp-lib's `AutocompleteInputModel` is the adapter that
loads `items`. Groups, create-new and multi-select are planned.

## navigation (feature `navigation`)

Links that say where the user is, and a side menu to hold them. The links
are leptos_router's `<A>`, so every component here goes inside a
`<Router>`. The feature turns on `overlay` (the side menu is a `Sheet`) and
brings in leptos_router. It's the only group that does.

| Use it for | Use |
|---|---|
| a list of places: a sidebar, a settings page's sections | `NavList` of `NavLink`s |
| places that belong together (one service's pages, the account's) | `NavGroup` inside the list, collapsible if it's long or rarely used |
| the app's navigation on a phone, or any screen without room for a sidebar | `SideMenu`, opened by a `SideMenuButton` |

```rust
use ferrox_ui::{NavGroup, NavLink, SideMenu, SideMenuButton};

let menu = RwSignal::new(false);

view! {
    <Router>
        <header class="app-bar">
            <SideMenuButton open=menu />
            <span class="app-bar__title">"Groceries"</span>
        </header>
        <SideMenu open=menu title="Groceries">
            <NavLink href="/" exact=true>"Lists"</NavLink>
            <NavLink href="/load">"Load Data"</NavLink>
            <NavGroup label="Product database" collapsible=true>
                <NavLink href="/units">"Units"</NavLink>
                <NavLink href="/categories">"Categories"</NavLink>
                <NavLink href="/products">"Products"</NavLink>
            </NavGroup>
            <NavGroup label="Account">
                <NavLink href="/account">"Account"</NavLink>
                <NavLink href="/logout">"Logout"</NavLink>
            </NavGroup>
        </SideMenu>
        <Routes fallback=|| "Not found">…</Routes>
    </Router>
}
```

Put the `SideMenu` outside `<Routes>`, beside the header, so it stays
mounted while the user moves between pages.

### `NavList`, `NavLink`

A labelled `<nav>` landmark holding a `<ul>`, one `<li>` per link, so a
screen reader hears the landmark's name and how many links it holds.

```rust
use ferrox_icons::SEARCH;
use ferrox_ui::{NavLink, NavList};

view! {
    <NavList label="Settings">
        <NavLink href="/settings" exact=true>"General"</NavLink>
        <NavLink href="/settings/search" icon=SEARCH>"Search"</NavLink>
    </NavList>
}
```

- `NavList`: `label` (required, the landmark's name), `class`, children
  (`NavLink`s and `NavGroup`s).
- `NavLink`: `href` (required), `exact`, `icon` (an `IconData`, drawn before
  the words and hidden from screen readers), `class` (on the link), children
  (the words). It renders an `<li>`, so it goes in a `NavList` or a
  `NavGroup`.

**The current page.** The router sets `aria-current="page"` on the link to
the page the user is on, and that's what tints it in the primary fill. A
link is current on its own path and on every path below it: `/accounts` is
current on `/accounts/42`. `exact` limits it to its own path, for a link
whose sub-pages have links of their own. `/` is current only on `/`.

Each link is a full-width row. It answers hover and press in colour only,
shows the theme's focus ring, and is at least 44px tall on a phone.
`nav_list_classes(extra)` and `nav_link_classes(extra)` return the class
lists.

### `NavGroup`

Links that belong together, under a heading that names them. A screen reader
hears "Product database, list, 3 items".

```rust
use ferrox_ui::NavGroup;

let remember = RwSignal::new(false);   // store it wherever the app keeps preferences

view! {
    <NavGroup label="Account">…</NavGroup>                                   // plain
    <NavGroup label="Product database" collapsible=true>…</NavGroup>         // starts collapsed
    <NavGroup label="Reports" collapsible=true start_expanded=true>…</NavGroup>
    <NavGroup label="Admin" collapsible=true expanded=remember>…</NavGroup>   // the app owns it
}
```

| Prop | Default | |
|---|---|---|
| `label` | required | the heading, and the name of the group's list |
| `collapsible` | `false` | the heading is a button that shows and hides the links |
| `start_expanded` | `false` | where a collapsible group starts |
| `expanded` | none | `RwSignal<bool>`, two-way, owned by the app; overrides `start_expanded` |
| `class` | | |
| children | | `NavLink`s; one level only, no group inside a group |

- **Plain:** the heading is quiet text (small, muted), not focusable, and a
  hairline splits the group from what comes before it.
- **Collapsible:** the heading is a `<button aria-expanded aria-controls>`
  with a chevron that points right while collapsed and turns down once
  expanded. Enter and Space toggle it. A collapsed group's links are
  `hidden`, so Tab skips them.
- **The current page is never hidden.** When the route moves to one of a
  collapsed group's links, or the page opens on one, the group expands
  itself. That happens on arrival only, so the user can still collapse the
  group around the page they're on.
- **Inside a `SideMenu`** a group's own state starts over each time the
  menu opens (the menu's body is mounted only while open): collapsed, unless
  the current page is in it. To remember it, pass `expanded`.

`nav_group_classes(collapsible, extra)` returns the class list.

### `SideMenu`, `SideMenuButton`

A `Sheet` holding a `NavList` of the children, with an optional footer
pinned to the bottom. It shares an `open` signal with its button.

```rust
use ferrox_ui::{SideMenu, SideMenuButton};

view! {
    <SideMenuButton open=menu />
    <SideMenu
        open=menu
        title="Groceries"
        help=MENU_HELP.clone()
        footer=|| view! { <span>"Version 1.4"</span> }
    >
        …
    </SideMenu>
}
```

**It closes itself.** `open` is an `RwSignal`, not `Sheet`'s `open` plus
`on_close`, because the menu sets it back to false on its own:

- when the user follows a link inside it: a plain left click on any
  `<a href>` without a `target`, so a plain `<A>` closes it too. A ctrl- or
  cmd-click opens a new tab and leaves the menu open;
- when the current page's link is clicked, though the route doesn't change;
- on any route change while it's open, such as the back button;
- on Esc, the × or a backdrop click, as any `Sheet`.

Focus then returns to the `SideMenuButton`. After a navigation, moving focus
into the new page is the app's routing concern, not the menu's.

| `SideMenu` prop | Default | |
|---|---|---|
| `open` | required | `RwSignal<bool>`, shared with `SideMenuButton` |
| `title` | required | the title bar's text, and the sheet's accessible name |
| `nav_label` | `"Sections"` | the name of the `<nav>` inside |
| `side` | `Left` | a `SheetSide` |
| `footer` | none | a closure returning a view, pinned below the links behind a hairline: a version, a legal line |
| `help` | none | a `?` in the title bar, before the × |
| `close_label` | `"Close"` | the ×'s label |
| `class` | | on the sheet, after `side-menu` |
| children | | `NavLink`s and `NavGroup`s |

`SideMenuButton` is an `IconButton` drawing `MENU`, with
`aria-haspopup="dialog"` and an `aria-expanded` that follows `open`. Props:
`open` (required), `label` (`"Menu"`, its only label and its tooltip),
`class`. `side_menu_classes(extra)` returns the sheet's class list.

Who's signed in, the app's title bar and a page frame aren't here: the bar
is the app's (or webapp-lib's), and a page frame is `layout`'s, later.

## Help (in `core`)

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

### `HelpTopic`

The explanation, as structured data (not Markdown or HTML, so nothing parses
a string or trusts tags in one). The app owns the wording and its
translation. It's cheap to clone.

```rust
use std::sync::LazyLock;
use ferrox_ui::HelpTopic;

static SPLIT_AMOUNT: LazyLock<HelpTopic> = LazyLock::new(|| {
    HelpTopic::new("split-amount", "Amounts on a split")          // id, title
        .paragraph("Each line moves money into or out of one account.")
        .list(["Positive: money comes in", "Negative: money goes out"])
        .table(["Sign", "Means"], [["+", "in"], ["-", "out"]])
        .guide("/help#split-amount")                             // "Full guide"
});
```

The `id` is stable: an assistant and a guide find the topic by it. The
`title` names the `?`.

### The `help` prop

| Component | Where the `?` sits | Also |
|---|---|---|
| `Field` | right after the label, outside the `<label>` | `help_skip_tab`, `help_shares_value` |
| `Checkbox` | after its words, outside the `<label>` (clicking it never ticks the box) | `help_skip_tab`, `help_shares_value` ("true" / "false") |
| `Switch` | beside the switch, not inside it | `help_skip_tab` |
| `Toolbar` | right after the title | |
| `Dialog`, `ConfirmDialog`, `Sheet` | in the title bar, just before the × | |
| `SideMenu` | in the title bar, just before the × | a request names it as a `sheet` |

```rust
<Field label="Amount" help=SPLIT_AMOUNT.clone()>
    <Input value=amount />
</Field>
```

`help` takes a `MaybeHelp`, which converts from a `HelpTopic` or an
`Option<HelpTopic>`. `help_skip_tab` keeps the `?` out of the Tab order for a
field in a row (it stays clickable and reachable by a screen reader).

### `HelpButton`

The `?` on its own, for anywhere else: beside a heading, in a table header.
The components above place one for you.

```rust
<h2>"Statement balance" <HelpButton topic=STATEMENT_BALANCE.clone() /></h2>
```

- **It names what it explains:** "Help: {title}", as its label and tooltip,
  so a screen reader knows which `?` it landed on.
- The popover is the browser's own: Esc or a click elsewhere closes it, one
  is open at a time, and Esc gives focus back to the `?`. It sits below the
  `?` (above when there's no room), stays inside the viewport, and follows
  the page as it scrolls.
- Props: `topic`, `skip_tab`, `kind` / `about` / `value` (what a request says
  carries the `?`; components set these), `help_label` (`"Help"`),
  `guide_label` (`"Full guide"`), `ask_label` (`"Ask about this"`), `class`.
- Put it where block content is allowed: not inside a `<label>` or a
  `<button>`.

### `HelpScope`: context for an assistant

Declare what an assistant should know where you know it. Scopes nest; a
request gathers them outermost first. A scope reaches only its own children.

```rust
<HelpScope context=[("mcp_server", "conrad"), ("domain", "bookkeeping")]>
    <HelpScope context=[("screen", "reconcile")]>
        <HelpScope context=move || vec![
            ("entity".into(), "account".into()),
            ("account_id".into(), account.get()),
        ]>
            <Field label="Statement balance" help=STATEMENT_BALANCE.clone()> … </Field>
        </HelpScope>
    </HelpScope>
</HelpScope>
```

`context` is an array or a `Vec` of pairs, or a closure returning them for
context that follows the page. It's read when the user asks, so it's always
current.

**Recommended keys.** The keys are free-form; these names let an assistant
(and a memory store such as Patina) rely on them:

| Key | Meaning | Example |
|---|---|---|
| `domain` | the app or subject area | `bookkeeping` |
| `screen` | the screen the user is on | `reconcile` |
| `section` | a part of that screen | `splits` |
| `entity` | the kind of thing in view | `transaction` |
| `<entity>_id` | its id, so a tool can fetch it | `transaction_id` |
| `mcp_server` | the MCP server that knows about this area | `conrad` |
| `mcp_tool` | the tool best placed to answer here | `conrad_get_transaction` |

Put `mcp_server` in a scope at the app's root, and `mcp_tool` and ids at the
screen or section that knows them. Stable mappings can live in the app's
`on_ask` handler instead. A rule of thumb: the page declares what only the
page knows, such as the current entity's id. MCP keys are **hints**: which
tools a model may call is decided by the client, never by the page, and the
model may pick a better tool. `domain`, `screen` and `entity` map onto
memory tags (Patina's `recall_tagged`).

### `HelpAssistant` and `HelpRequest`

Wrap the app (or a part of it). Every help popover inside offers "Ask about
this".

```rust
use ferrox_ui::{HelpAssistant, HelpRequest};

view! {
    <HelpAssistant on_ask=move |request: HelpRequest| chat.ask(request)>
        <Router>…</Router>
    </HelpAssistant>
}
```

| `HelpRequest` field | |
|---|---|
| `topic_id`, `topic_title` | the topic |
| `component` | what carries the `?`: `field`, `checkbox`, `switch`, `toolbar`, `dialog`, `alertdialog`, `sheet`, or `help` for a bare `HelpButton` |
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

### Privacy

A request carries only what the developer declared: the topic, the
component, its label, and scope context. A control's **value** is sent only
when that component says `help_shares_value=true`. `PasswordInput` never
offers its value, so a password can't be sent even then. Keep ids and
private details out of `HelpScope` unless the assistant should see them.

## CSS classes (core)

All of these are injected by `init()`, built only from theme tokens, and
selected by class only.

| Sheet | Classes |
|---|---|
| `a11y.css` | `.sr-only`: read out by screen readers, takes no space |
| `focus.css` | the focus ring for `.btn` and `.icon-btn` |
| `buttons.css` | `.btn`, `.btn--{primary,secondary,cancel,danger}`, `.btn--quiet`, `.btn--{sm,lg}`, `.is-saving`; `.icon-btn`, `.icon-btn--danger`, `[aria-pressed='true']` |
| `icon.css` | `.icon`: an SVG icon as a block with no pointer events |
| `toolbar.css` | `.toolbar`, `.toolbar--compact`, `.toolbar__title`, `.toolbar__group`, `.toolbar__heading` (a title with its `?`) |
| `spinner.css` | `.spinner`, `.spinner--{region,inline,field}`, `.spinner-status`, `.spinner-status--region`, `.spinner-overlay`, `.spinner-overlay__text`; keyframes `ferrox-spin`, `ferrox-spinner-appear` |
| `alert.css` | `.alert`, `.alert--{info,success,warning,error}`, `.alert__icon`, `.alert__body`, `.alert__dismiss` |
| `help.css` | `.help-btn` (the `?`, with `.icon-btn`), `.help-popover`, `__title`, `__body`, `__paragraph`, `__list`, `__table` / `__heading` / `__cell`, `__actions`, `__link`, `__ask`; `[data-side]` below or above |

## CSS classes (overlay)

| Sheet | Classes |
|---|---|
| `modal.css` | `.dialog-open` on `<body>`: no page scroll while a modal is open |
| `dialog.css` | `.dialog`, `.dialog--wide`, `.dialog::backdrop`, `.dialog__title-bar`, `.dialog__title`, `.dialog__help`, `.dialog__close`, `.dialog__body`, `.dialog__footer` |
| `sheet.css` | `.sheet`, `.sheet--{left,right,bottom}`, `.sheet::backdrop`, `.sheet__title-bar`, `.sheet__title`, `.sheet__help`, `.sheet__close`, `.sheet__body` |

These are injected only when the `overlay` feature is on.

## CSS classes (forms)

| Sheet | Classes |
|---|---|
| `field.css` | `.field`, `.field__label`, `.field__label-row` (a label with its `?`), `.field__hint`, `.field__error` (also under a `Checkbox`) |
| `controls.css` | `.form-input`, `.form-textarea`, `.form-select`, `.is-invalid` on each (the danger border); `.select` (a select and its refresh button) |
| `password.css` | `.password-input`, `.password-input__toggle`; `.password-strength`, `.password-strength--{very-weak,weak,fair,strong}`, `.password-strength__bar`, `.password-strength__fill` |
| `choices.css` | `.checkbox`, `.checkbox.is-invalid`, `.checkbox__row` (the `<label>`), `.checkbox__input`, `.checkbox__label`, `.checkbox__line` (the row with its `?`); `.switch`, `[aria-checked='true']`, `.switch__track`, `.switch__knob`, `.switch-line` (the switch with its `?`) |
| `combobox.css` | `.combobox`, `.combobox__controls`, `.combobox__control`, `.combobox__list`, `.combobox__option`, `.combobox__option.is-active`, `.combobox__empty` |

No rule touches a bare `input`, `select` or `label`: an app's own unclassed
controls are left alone. On a phone (≤ 768px) the controls' text is at least
16px, so iOS doesn't zoom in when one takes focus. These are injected only
when the `forms` feature is on.

## CSS classes (navigation)

| Sheet | Classes |
|---|---|
| `nav.css` | `.nav-list`, `.nav-list__items`; `.nav-link`, `.nav-link__icon`, `.nav-link[aria-current='page']`; `.nav-group`, `.nav-group--collapsible`, `.nav-group__heading`, `.nav-group__toggle` (`[aria-expanded='true']`), `.nav-group__label`, `.nav-group__chevron`, `.nav-group__list` |
| `side-menu.css` | `.side-menu` (with `.sheet`), `.side-menu__content`, `.side-menu__footer` |

These are injected only when the `navigation` feature is on.

A unit test (`css.rs`) fails if any shipped selector doesn't start with a
class.

## Requirements

- `leptos` 0.8 (CSR) and a browser (`web-sys`).
- ferrox-theme's stylesheet linked in the app's `index.html`
  (ferrox-theme-build). Every component reads its tokens.
- ferrox-icons, used for Alert's icons and the close buttons (a dependency,
  not something the app sets up).
- `overlay`: a browser with `<dialog>`/`showModal()` and `@starting-style`
  (Chrome 117+, Firefox 129+, Safari 17.5+). Without `@starting-style` a
  sheet just appears instead of sliding.
- `navigation`: leptos_router 0.8, with every component inside a
  `<Router>`.
- Help and `Combobox`: a browser with the Popover API (`popover`, `popovertarget`; Chrome
  114+, Firefox 125+, Safari 17+).

## Testing

- `cargo test -p ferrox-ui`: class lists, roles, and the class-only rule on
  every sheet.
- The demo (`demo/`) shows every component: `/buttons`, `/feedback`,
  `/overlay`, `/forms`, `/navigation`, `/layout` and `/help` (with a fake assistant that shows the
  request it receives). Its Playwright tests (`demo/e2e/tests/*.spec.ts`)
  run in Chromium and Firefox through `scripts/release-check.sh`.

## Planned

From `plans/ferrox-ui.md`, one group per piece, each with a demo page and e2e
tests: `layout` (Panel), `data` (Data grid). `core` later gains
Skeleton, Badge, Card and a theme toggle. `overlay` later gains
`TypeToConfirmDialog`, Toast, Popover and Tooltip, and the
phone variants of Dialog (a bottom sheet, full screen) when an app needs them.
Help later gains a topic registry (checked `related` links, a `/help` guide
rendered from the same topics) and `help` on new components as they land.
`forms` later gains Combobox groups, create-new and multi-select, and
NumberInput, DateInput, Radio and the rest of the inventory when an app needs
them. `navigation` later gains a persistent sidebar that folds to a rail of
icons, count badges on links (with `core`'s Badge), Breadcrumb, Pagination,
Bottom nav and Stepper.

## Licence

`MIT OR Apache-2.0`.
