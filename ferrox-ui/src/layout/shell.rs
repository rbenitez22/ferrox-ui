use leptos::prelude::*;

/// What an app shell (`AppShell`) tells the pages inside it: the app's
/// name, and that there is already a `<main>`. Counts the pages shown, so a
/// page knows whether it arrived by navigation.
#[derive(Clone, Copy)]
#[expect(dead_code, reason = "read by `Page`, the next piece")]
pub(crate) struct ShellContext {
    pub title: Signal<String>,
    pub pages_shown: StoredValue<u32>,
}

impl ShellContext {
    pub(crate) fn new(title: Signal<String>) -> Self {
        Self { title, pages_shown: StoredValue::new(0) }
    }
}

/// The enclosing app shell, if any.
pub(crate) fn shell() -> Option<ShellContext> {
    use_context::<ShellContext>()
}
