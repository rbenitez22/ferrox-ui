# `Spinner`, `LoadingOverlay`

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
    {move || signing_in.get().then(|| view! { <LoadingOverlay label="Signing in…"/> })}
}
```

- `Spinner`: `label` (required; read out by screen readers, not shown),
  `kind` (`Region` by default, `Inline` or `Field`), `class`. It renders
  `role="status"`.
- `LoadingOverlay`: `label` (optional; shown under the spinner and read out;
  without it screen readers hear "Loading…"). A fixed scrim over the whole
  page, `role="status"` with `aria-live="polite"`.

`spinner_status_classes(kind, extra)` returns the wrapper's class list.
