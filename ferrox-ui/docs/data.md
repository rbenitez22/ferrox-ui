# data (feature `data`)

Record lists: many records drawn through one item template, laid out as
the screen needs, with their loading, error and empty states, and two ways
to reach the rest of a long list.


Putting them together:

```rust
use ferrox_ui::{Card, DataLayout, DataLayoutSwitch, DataView, Pagination, Panel};

let layout = RwSignal::new(DataLayout::default());
let page = RwSignal::new(1);

view! {
    <Panel title="Recipes" actions=move || view! { <DataLayoutSwitch layout=layout /> }>
        <DataView
            items=Signal::derive(move || recipes_on(page.get()))
            key_of=|r: &Recipe| r.id.clone()
            item=|r: Recipe| view! { <Card title=r.name.clone() href=format!("/recipes/{}", r.id)>…</Card> }
            label="Recipes"
            layout=layout
            loading=loading
            error=error
            on_retry=move || reload()
        />
        <Pagination page=page page_count=page_count />
    </Panel>
}
```

**Pages or infinite scroll?** The app's choice, per list. Pages
(`Pagination`) where users come back to a place: admin lists, search
results, anything worth linking to. Infinite scroll (`DataView`'s
`on_more`) for feeds read from the top. Not both on one list.

None of them fetch, sort, filter or route: whatever loads the records owns
that, and owns the signals (`layout`, `page`), so remembering a choice (in
storage, in the URL) is the app's. Selecting records, tables with columns
and sorting, and virtualised rendering for thousands of items are not here
(see Planned in the [README](../README.md#planned)).

Which to use: [choosing.md](choosing.md#data).

## Pages

- [`DataView<T>`](data-view.md)
- [`DataLayoutSwitch`](data-layout-switch.md)
- [`Pagination`](pagination.md)
