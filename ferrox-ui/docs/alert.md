# `Alert`

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
        <Alert severity=AlertSeverity::Error on_close=move || error.set(None)>{e}</Alert>
    })}
}
```

| Prop | Default | |
|---|---|---|
| `severity` | required | `Info`, `Success`, `Warning` (non-blocking: if it should stop a submit, it's an `Error`), `Error` |
| `icon` | `true` | draws the severity's icon (ferrox-icons `INFO`, `SUCCESS`, `WARNING`, `ERROR`), so severity doesn't rest on colour alone |
| `on_close` | none | shows a close button (×) that calls it; the caller removes the alert |
| `close_label` | `"Close"` | the close button's label |
| `class` | | |

Only `Error` interrupts a screen reader (`role="alert"`); the others are
`role="status"`. There's no primary alert, because an alert reports a state
and doesn't act. `alert_classes(severity, extra)` returns the class list, and
`AlertSeverity` has `class()`, `icon()` and `role()`.
