use leptos::prelude::*;

/// What an app shell (`AppShell`) tells the pages inside it: the app's
/// name, that there is already a `<main>`, and the current path, so a page
/// can tell a navigation (focus moves to it) from the first load or a
/// re-render on the same path (it doesn't).
#[derive(Clone, Copy)]
pub(crate) struct ShellContext {
    pub title: Signal<String>,
    pub path: Signal<String>,
    /// The path a page last took focus for (or the first page's).
    pub settled_path: StoredValue<Option<String>>,
}

impl ShellContext {
    pub(crate) fn new(title: Signal<String>, path: Signal<String>) -> Self {
        Self { title, path, settled_path: StoredValue::new(None) }
    }

    /// Whether a page appearing now arrived by navigation: not the first
    /// page shown, and on a path no page has taken focus for yet. Records it.
    pub(crate) fn arrived_by_navigation(&self) -> bool {
        let path = self.path.get_untracked();
        let before = self.settled_path.get_value();
        self.settled_path.set_value(Some(path.clone()));
        before.is_some_and(|before| before != path)
    }
}

/// The enclosing app shell, if any.
pub(crate) fn shell() -> Option<ShellContext> {
    use_context::<ShellContext>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_a_new_path_after_the_first_page_is_a_navigation() {
        Owner::new().with(|| {
            let path = RwSignal::new("/".to_string());
            let shell = ShellContext::new(Signal::stored("App".to_string()), path.into());
            assert!(!shell.arrived_by_navigation(), "the first page");
            assert!(!shell.arrived_by_navigation(), "re-rendered on the same path");
            path.set("/settings".into());
            assert!(shell.arrived_by_navigation());
            assert!(!shell.arrived_by_navigation(), "a second page on that path");
        });
    }
}
