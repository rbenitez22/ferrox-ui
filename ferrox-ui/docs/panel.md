# `Panel`

A `<section>` named by a real heading: a header band with the title on the
left and the actions on the right, then the body. One component serves a
page's own title (`H1`) and a section inside a page (`H2`, the default).

```rust
use ferrox_icons::{Icon, PLUS, SEARCH};
use ferrox_ui::{HeadingLevel, IconButton, Panel, PanelTone};

view! {
    <Panel title="Shopping Lists" heading_level=HeadingLevel::H1>
        …
    </Panel>

    <Panel
        title="Items"
        actions=move || view! {
            <IconButton label="Search items" on_click=move || search()><Icon icon=SEARCH /></IconButton>
            // The app's own permission decides whether "add" shows.
            <Show when=move || can_add_items.get()>
                <IconButton label="Add item" on_click=move || new_item()><Icon icon=PLUS /></IconButton>
            </Show>
        }
    >
        …
    </Panel>

    <Panel title="Load Data" heading_level=HeadingLevel::H1>…</Panel>   // a bar with no buttons
    <Panel title="Transactions" flush=true>…</Panel>                     // a table runs to the edges
}
```

| Prop | Default | |
|---|---|---|
| `title` | required | the heading's text, and the section's name |
| `heading_level` | `H2` | `H1`…`H4`, for the page's outline. Every level looks the same |
| `actions` | none | a closure returning buttons (add, search, export: whatever the app needs), at the right of the band |
| `compact` | `false` | a slimmer band in body type and a tighter body, for one panel among several |
| `flush` | `false` | no padding round the body, for a table or a grid |
| `tone` | `Default` | `Warning`: something to act on. `Danger`: something that can't be undone (the title turns red too). Tints the band and the border |
| `help` | none | a `?` right after the title |
| `class` | | |
| children | | the body |

The title ends in an ellipsis rather than wrapping under the buttons, and a
band with no buttons is as tall as one with them. On a phone the band's
buttons are 44px. `panel_classes(collapsible, compact, flush, tone, extra)`
returns the class list.

**Collapsible.** The title becomes a button inside the heading, so the page's
outline keeps the heading and the whole band is the target.

```rust
view! {
    <Panel title="Danger zone" tone=PanelTone::Danger collapsible=true summary="Delete this book">
        …
    </Panel>
    <Panel title="Notes" collapsible=true expanded=notes_open summary=Signal::derive(move || format!("{} notes", count.get()))>
        …
    </Panel>
}
```

| Prop | Default | |
|---|---|---|
| `collapsible` | `false` | the title opens and closes the body (`aria-expanded`, `aria-controls`; Enter and Space) |
| `start_expanded` | `false` | where it starts |
| `expanded` | none | `RwSignal<bool>`, two-way, for an app that owns or remembers the state; overrides `start_expanded` |
| `summary` | none | one line in the band while it's shut, in body type: "3 overdue" |

- **The body isn't rendered until it's first opened**, so a shut panel that
  would load data doesn't. From then on it stays mounted and is `hidden`
  while shut, so whatever was typed in it survives.
- **While shut** the actions are hidden; the `?` stays, since
  help explains a shut panel too. On a collapsible panel the `?` sits after
  the toggle.
- The section is named by the title alone, never by the summary.
- The chevron points right while shut and down once open (no animation
  under reduced motion). Hovering the toggle tints the whole band.
