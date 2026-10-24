# `Select`

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
            on_create=move || new_category_dialog.set(true)
            create_label="New category…"
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
| `on_create` | none | adds a `+` button after the select (and after refresh), labelled `create_label` (`"New…"`); disabled with the select |
| `name`, `required`, `disabled`, `class` | | `class` goes on the `<select>` |

The `<select>` is rendered once. Only its options and which one is selected
react, so it keeps focus and its choice when the options change. Loading the
options is the caller's job; webapp-lib's `SelectMenu` is an adapter that
fetches them. So is creating one: `on_create` only says "new" was pressed.
Usually the app opens a dialog, then adds the option and sets `value`.
