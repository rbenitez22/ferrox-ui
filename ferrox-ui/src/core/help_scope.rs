use leptos::context::Provider;
use leptos::prelude::*;
use leptos::{component, view, IntoView};

use super::help_request::HelpRequest;

// ---------------------------------------------------------------------------
// HelpContext
// ---------------------------------------------------------------------------

/// Key/value pairs a `HelpScope` adds to every help request inside it: a
/// fixed list, or a closure that follows the page.
///
/// Keys are free-form. Recommended: `domain`, `screen`, `section`, `entity`
/// (they map straight onto memory tags for an assistant).
#[derive(Clone, Copy)]
pub struct HelpContext(Signal<Vec<(String, String)>>);

impl HelpContext {
    #[cfg(test)]
    fn pairs(&self) -> Vec<(String, String)> {
        self.0.get_untracked()
    }
}

impl<K: Into<String>, V: Into<String>, const N: usize> From<[(K, V); N]> for HelpContext {
    fn from(pairs: [(K, V); N]) -> Self {
        Self(Signal::stored(pairs.into_iter().map(|(k, v)| (k.into(), v.into())).collect()))
    }
}

impl<K: Into<String>, V: Into<String>> From<Vec<(K, V)>> for HelpContext {
    fn from(pairs: Vec<(K, V)>) -> Self {
        Self(Signal::stored(pairs.into_iter().map(|(k, v)| (k.into(), v.into())).collect()))
    }
}

impl<F> From<F> for HelpContext
where
    F: Fn() -> Vec<(String, String)> + Send + Sync + 'static,
{
    fn from(pairs: F) -> Self {
        Self(Signal::derive(pairs))
    }
}

// ---------------------------------------------------------------------------
// HelpScope
// ---------------------------------------------------------------------------

/// The pairs gathered from every enclosing scope, outermost first.
#[derive(Clone, Copy)]
pub(crate) struct ScopeContext(Signal<Vec<(String, String)>>);

/// The context a help request made here carries (empty outside any scope).
/// A signal, read when the request is made, so a scope that follows the page
/// sends what is current.
pub(crate) fn scope_context() -> Signal<Vec<(String, String)>> {
    use_context::<ScopeContext>().map_or_else(|| Signal::stored(Vec::new()), |scope| scope.0)
}

/// Declares what an assistant should know about everything inside it:
/// this screen, this account, this section. Scopes nest; a request gathers
/// them outermost first. A scope reaches only its own children.
///
/// ```ignore
/// <HelpScope context=[("screen", "reconcile")]>
///     <HelpScope context=move || vec![("account".to_string(), account.get())]>
///         …
///     </HelpScope>
/// </HelpScope>
/// ```
#[component]
pub fn HelpScope(#[prop(into)] context: HelpContext, children: Children) -> impl IntoView {
    let parent = use_context::<ScopeContext>();
    let gathered = Signal::derive(move || {
        let mut pairs = parent.map(|p| p.0.get()).unwrap_or_default();
        pairs.extend(context.0.get());
        pairs
    });
    view! { <Provider value=ScopeContext(gathered)>{children()}</Provider> }
}

// ---------------------------------------------------------------------------
// HelpAssistant
// ---------------------------------------------------------------------------

/// The app's assistant, as seen by a `?`.
#[derive(Clone, Copy)]
pub(crate) struct Assistant(UnsyncCallback<HelpRequest>);

/// The assistant a `?` here would ask, if any.
pub(crate) fn assistant() -> Option<UnsyncCallback<HelpRequest>> {
    use_context::<Assistant>().map(|a| a.0)
}

/// Turns on "Ask about this" in every help popover inside it, usually once
/// around the whole app. `on_ask` receives the [`HelpRequest`]; the app
/// decides what asking means (open its chat, call a model with
/// `to_prompt_context()`, search its memory by the context's tags).
///
/// Without one, help is just its topics.
///
/// ```ignore
/// <HelpAssistant on_ask=move |request: HelpRequest| chat.ask(request)>
///     <Router>…</Router>
/// </HelpAssistant>
/// ```
#[component]
pub fn HelpAssistant(#[prop(into)] on_ask: UnsyncCallback<HelpRequest>, children: Children) -> impl IntoView {
    view! { <Provider value=Assistant(on_ask)>{children()}</Provider> }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_from_arrays_vectors_and_closures() {
        Owner::new().with(|| {
            let fixed: HelpContext = [("screen", "reconcile")].into();
            assert_eq!(fixed.pairs(), vec![("screen".to_string(), "reconcile".to_string())]);
            let listed: HelpContext = vec![("a", "1"), ("b", "2")].into();
            assert_eq!(listed.pairs().len(), 2);
            let account = RwSignal::new("checking".to_string());
            let live: HelpContext = (move || vec![("account".to_string(), account.get())]).into();
            account.set("savings".into());
            assert_eq!(live.pairs(), vec![("account".to_string(), "savings".to_string())]);
        });
    }

    #[test]
    fn outside_any_scope_there_is_no_context_and_no_assistant() {
        Owner::new().with(|| {
            assert!(scope_context().get_untracked().is_empty());
            assert!(assistant().is_none());
        });
    }
}
