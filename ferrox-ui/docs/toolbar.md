# `Toolbar`, `ToolbarGroup`

A row of buttons. Buttons sit at the right, and an optional `title` takes the
free space on the left. `compact` tightens the row for icon buttons and small
buttons. Groups are split by a hairline.

```rust
use ferrox_ui::{Toolbar, ToolbarGroup};

view! {
    <Toolbar title="Accounts">
        <Button on_click=move || add()>"New account"</Button>
    </Toolbar>

    <Toolbar compact=true>
        <ToolbarGroup>
            <IconButton label="Search"><Icon icon=SEARCH/></IconButton>
            <IconButton label="Camera" pressed=camera><Icon icon=CAMERA/></IconButton>
        </ToolbarGroup>
        <ToolbarGroup>
            <IconButton label="Help"><Icon icon=HELP/></IconButton>
        </ToolbarGroup>
    </Toolbar>
}
```

`Toolbar` takes `title`, `compact`, `help` (a `?` right after the title) and
`class`; `ToolbarGroup` takes `class`.
`toolbar_classes(compact, extra)` returns the class list.
