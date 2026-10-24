# `DataLayoutSwitch`

Lets the user pick a `DataView`'s layout: a group (`role="group"`, named
"Layout") of `IconButton`s, one per choice, the current one pressed
(`aria-pressed`). It sits wherever the app wants, typically a `Panel`'s
actions.

<!-- example: data-layout-switch -->
```rust
use ferrox_ui::{Card, DataLayout, DataLayoutSwitch, DataView, Panel};

let layout = RwSignal::new(DataLayout::default());

view! {
    <Panel title="Recipes" actions=move || view! { <DataLayoutSwitch layout=layout /> }>
        <DataView
            items=recipes
            key_of=|r: &Recipe| r.id.clone()
            item=|r: Recipe| view! { <Card title=r.name>"…"</Card> }
            label="Recipes"
            layout=layout
        />
    </Panel>
}
```
<!-- /example -->

| Prop | Default | |
|---|---|---|
| `layout` | required | `RwSignal<DataLayout>`, two-way: the app owns it, so remembering the choice is the app's |
| `choices` | `[Grid(Md), List]` | the layouts offered, in order |
| `labels` | `DataLayoutLabels::default()` | the group's name ("Layout") and each layout's: "Small grid", "Grid", "Large grid", "List", "Strip", "Cloud". Change some with `DataLayoutLabels { list: "Rows".into(), ..Default::default() }` |
| `class` | | extra classes on the group |

Icons (`layout_icon(layout)`): `LAYOUT_GRID` for every grid size, `LIST`,
`GALLERY_HORIZONTAL` (Row), `TAGS` (Wrap).
