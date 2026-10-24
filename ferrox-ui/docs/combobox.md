# `Combobox<T>`

A text field that filters a list and lets one item be chosen: the ARIA
combobox pattern (`role="combobox"`, a `listbox` of `option`s,
`aria-activedescendant` for the active one). Use it when a `Select` would be
too long to scan.

<!-- example: combobox -->
```rust
use ferrox_ui::{Combobox, Field};

// `accounts: Signal<Vec<Account>>` is loaded by the app.
let account_text = RwSignal::new(String::new());
let chosen = RwSignal::new(None::<Account>);

view! {
    <Field label="Account" error=account_error>
        <Combobox
            items=accounts
            key_of=|a: &Account| a.id.clone()
            label_of=|a: &Account| a.name.clone()
            text=account_text
            on_select=move |a| chosen.set(a)
            placeholder="Type to search"
            on_refresh=move || reload_accounts()
            refreshing=accounts_refreshing
            on_create=move |name: String| create_account(name)
        />
    </Field>
}
```
<!-- /example -->

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
| `on_create` | none | `UnsyncCallback<String>`: offers to create an item from the typed text when nothing matches; gets the text trimmed |
| `create_label` | `Create “{text}”` | `Callback<String, String>`: the offer's text, given the typed text |
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
- **Creating** (with `on_create`): while the trimmed text is not empty and
  matches no item, a "Create “milk”" button sits under the list. A click
  presses it, and so does Enter when no option is active. It's a button,
  not an option: creating is a different act from choosing. The list closes
  and `on_create` gets `"milk"`. What happens next is the app's: add it to
  a static list, `POST` it, open a dialog for more. Typically the app then
  adds the item to `items`, sets `text` to its label and treats it as
  chosen.
- **Its controls** (clear, the chevron, refresh, create) are buttons out of the Tab
  order, so a row of comboboxes costs no extra keystrokes. Pressing one
  doesn't take focus from the field.
- **The list is in the top layer** (a popover, with the offer to create
  under it), placed under the field and
  as wide as it, and it follows the field as the page scrolls. A `Dialog`
  body or any `overflow` parent can't clip it.

It doesn't fetch. webapp-lib's `AutocompleteInputModel` is the adapter that
loads `items`. Groups and multi-select are planned.
