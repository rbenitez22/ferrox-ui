# overlay (feature `overlay`)

Modals built on the browser's own `<dialog>`, opened with `showModal()`. The
browser makes the page behind inert, keeps focus inside, and puts the modal in
the top layer, above everything including `LoadingOverlay`. All three share
the same behaviour:

- **`open` is an `RwSignal<bool>`, and the modal closes itself.** Set it
  true to open. Every dismissive way out (Esc, the ×, the Cancel button, a
  sheet's backdrop, the browser closing the element itself) sets it false,
  then calls `on_close` / `on_cancel` if you gave one, so the app's state
  and the element never disagree. An affirm action closes it by setting
  `open` false when its work is done (after any saving state).
- **Focus** goes to an element marked `autofocus`, otherwise the first
  control (the ×). On close it returns to whatever opened the modal.
- **The page doesn't scroll** while a modal is open (`<body>` gets
  `.dialog-open`). This is counted, so a confirm opened from a dialog keeps
  the page locked until both close.
- **The body is mounted only while open**, so each opening starts fresh.


**Watch your descendant selectors.** A `<dialog>` stays where it's declared
in the DOM, so an app rule like `.section h2 { margin: … }` also reaches the
dialog's title. Scope such rules to direct children (`.section > h2`) where
dialogs are declared.

Which to use: [choosing.md](choosing.md#overlay).

## Pages

- [`Dialog`, `DialogAction`](dialog.md)
- [`ConfirmDialog`](confirm-dialog.md)
- [`Sheet`](sheet.md)
