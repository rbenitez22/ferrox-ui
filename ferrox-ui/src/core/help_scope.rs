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
/// Keys are free-form. Recommended: `domain`, `screen`, `section`, `entity`,
/// `<entity>_id`. An app may add its own (routing hints for its assistant,
/// say); they are hints only: a page never decides what a model may do.
#[derive(Clone, Copy)]
pub struct HelpContext(Signal<Vec<(String, String)>>);

impl Default for HelpContext {
    fn default() -> Self {
        Self(Signal::stored(Vec::new()))
    }
}

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
// HelpReveal
// ---------------------------------------------------------------------------

/// When a control's `?` shows: always, or only while the control is in use.
///
/// For a form with help on many fields, where a `?` by every one is noise.
/// A hidden `?` is only transparent: it keeps its place, Tab still reaches
/// it (and shows it), and a screen reader still finds it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HelpReveal {
    #[default]
    Always,
    /// Shown while the control holds focus or the pointer, or its help is
    /// open.
    WhileActive,
}

/// The nearest enclosing scope's `reveal`.
#[derive(Clone, Copy)]
struct ScopeReveal(HelpReveal);

/// Whether a control's `?` shows only while it's active: its own choice,
/// else the nearest scope's, else always shown.
pub(crate) fn reveals_while_active(own: Option<HelpReveal>) -> bool {
    own.or_else(|| use_context::<ScopeReveal>().map(|scope| scope.0)).unwrap_or_default() == HelpReveal::WhileActive
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
/// `reveal` sets when the `?`s of the controls inside it (Field, Checkbox,
/// Switch) show, unless a control sets its own `help_reveal`; an inner
/// scope's wins.
///
/// ```ignore
/// <HelpScope context=[("screen", "reconcile")]>
///     <HelpScope context=move || vec![("account".to_string(), account.get())]>
///         …
///     </HelpScope>
/// </HelpScope>
/// ```
#[component]
pub fn HelpScope(
    #[prop(optional, into)] context: HelpContext,
    /// When the `?`s of the controls inside show (unset: as outside).
    #[prop(optional)] reveal: Option<HelpReveal>,
    children: Children,
) -> impl IntoView {
    let parent = use_context::<ScopeContext>();
    let gathered = Signal::derive(move || {
        let mut pairs = parent.map(|p| p.0.get()).unwrap_or_default();
        pairs.extend(context.0.get());
        pairs
    });
    match reveal {
        Some(reveal) => view! {
            <Provider value=ScopeContext(gathered)>
                <Provider value=ScopeReveal(reveal)>{children()}</Provider>
            </Provider>
        }.into_any(),
        None => view! { <Provider value=ScopeContext(gathered)>{children()}</Provider> }.into_any(),
    }
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

    #[test]
    fn a_controls_own_reveal_wins_over_its_scopes() {
        Owner::new().with(|| {
            assert!(!reveals_while_active(None));
            assert!(reveals_while_active(Some(HelpReveal::WhileActive)));
            provide_context(ScopeReveal(HelpReveal::WhileActive));
            assert!(reveals_while_active(None));
            assert!(!reveals_while_active(Some(HelpReveal::Always)));
        });
    }
}
