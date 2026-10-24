# `ConfirmDialog`

The standard destructive confirmation (`role="alertdialog"`). The confirm
button is always in the danger family and comes last. Its label is the verb
("Delete", "Void", "Archive"), not "OK". The body names exactly what is about
to be lost. Focus opens on the ×, never on the confirm button.

<!-- example: confirm-dialog -->
```rust
use ferrox_ui::ConfirmDialog;

view! {
    <ConfirmDialog
        open=confirming
        title="Delete account?"
        confirm_label="Delete"
        saving=deleting
        on_confirm=move || delete_account() // set `deleting`, then `confirming` false when done
    >
        <p>"This deletes “Groceries” and its 214 transactions."</p>
    </ConfirmDialog>
}
```
<!-- /example -->

Props: `open` (`RwSignal<bool>`; Cancel, the × and Esc close it), `title`,
`confirm_label`, `on_confirm` (set `open` false when the action is done),
`on_cancel` (optional: told after Cancel, the × or Esc), `saving` (the confirm button sweeps and refuses clicks),
`cancel_label` (`"Cancel"`), `close_label` (`"Close"`), `help` (a `?` before
the ×).
