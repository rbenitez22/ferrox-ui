# `Dialog`, `DialogAction`

A title bar with the ×, the body, and a footer band. **The footer isn't a
slot.** Pass the actions and the component fixes the order: cancel,
alternate, affirm, with the heaviest button last.

```rust
use ferrox_ui::{Dialog, DialogAction, DialogSize};

let editing = RwSignal::new(false);
let saving = RwSignal::new(false);

view! {
    <Button on_click=move || editing.set(true)>"Edit account…"</Button>
    <Dialog
        open=editing
        title="Edit account"
        affirm=DialogAction::new("Save", move || save()).saving(saving)
        alternate=DialogAction::new("Save as draft", move || save_draft())
    >
        <label>"Name" <input autofocus=true /></label>
    </Dialog>
}
```

| Prop | Default | |
|---|---|---|
| `open`, `title` | required | `open`: `RwSignal<bool>`; Esc, the × and Cancel set it false (the same act) |
| `on_close` | none | told after that dismissive close |
| `affirm` | none | the last button; `DialogAction::new(label, fn)`, plus `.danger()` for a destructive commit, `.saving(signal)`, `.disabled(signal)` |
| `alternate` | none | a lighter way to finish, before `affirm`, always secondary; Enter never reaches it |
| `cancel_label` | `"Cancel"` | |
| `no_cancel` | `false` | no Cancel button (the × and Esc still close) |
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
