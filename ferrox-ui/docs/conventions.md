# Conventions (every component)

- `class`: extra classes, appended to the component's own. A string, or a
  `Signal<String>` to make it reactive. A bare closure doesn't convert to
  `MaybeProp<String>`, so wrap it in `Signal::derive`.
- No `style` prop. Styling goes through classes and tokens.
- Enums (`ButtonFamily`, `SpinnerKind`, `AlertSeverity`, …) are
  `#[non_exhaustive]`: match them with a `_` arm.
- Each component has a `*_classes` function that returns the class list it
  renders, for markup that needs the same look without the component.
- Every human-readable word is a prop, typed `Signal<String>` (a literal
  or a signal), with an English default where it has one. One word is an
  `x_label` prop ("Close", "Cancel", "Retry"); a set of words for repeated
  parts is an `XLabels` struct with `Default` (`PaginationLabels`).
- `label` names a control (its visible label, or its accessible name);
  `title` is a heading's text. `IconButton` takes `label`; `Panel`, `Card`
  and the overlays take `title`. (`Button`'s `title` is a plain tooltip.)
- Every callback is an `on_*` prop, and none passes a DOM event: write
  `on_click=move || save()`. Where a component hands you something, it's
  the value itself: `on_select=move |item| …`, `on_create=move |text| …`.
- Every × is "close" (`on_close`, `close_label`); a dismissive button is
  "cancel" (`cancel_label`).
- Content you hand a component (its children, `actions`, `header`, a
  `ContextMenu`'s `menu`) is built once and kept, so closures inside it may
  move values. The exceptions rebuild on purpose, and there a closure must
  be able to run again (clone what it needs, or keep it in a signal): a
  `Dialog`'s, `Sheet`'s and `ConfirmDialog`'s body and a `SideMenu`'s
  contents (fresh on each opening), and `DataView`'s `item` and
  `empty_view` (once per item, and each time the list empties). A callback
  (`on_*`) can always run many times.
- Components with a label or a title take `help` (a `HelpTopic`): see
  [Help](help.md).

## The page

Put `class="ferrox-app"` on `<body>` (the starter does): the page takes
the theme's background, text colour and body font, `<body>`'s margin goes,
and plain links (an `<a>` with no class of its own) take the theme's link
colour, underlined on hover. Components' own links keep their own look.

## The examples

Every example in these pages is compiled (`tests/doc_examples.rs`, copied in
by `tests/docs_current.rs`), so it matches the code. Each assumes
`use leptos::prelude::*;` and names its own ferrox imports. Names like
`save()`, `reload()` or `recipes` stand for the app's own functions and data.
