use leptos::context::Provider;
use leptos::prelude::*;
use leptos::{component, view, IntoView};

use super::help_scope::scope_context;

// ---------------------------------------------------------------------------
// HelpSignal
// ---------------------------------------------------------------------------

/// A fact that may mean the user needs help: reported by a component, or by
/// the app. Facts, not judgments: whether to offer help is the app's policy.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct HelpSignal {
    pub kind: HelpSignalKind,
    /// What reported it ("field", "dialog", "help", "app", …).
    pub component: String,
    /// Its label or title (empty for the app's own reports).
    pub about: String,
    /// The topic of the `?` on that component, if it has help.
    pub topic_id: Option<String>,
    /// The enclosing `HelpScope`s' context, outermost first.
    pub context: Vec<(String, String)>,
    /// Only what the app passes to `report`.
    pub detail: Option<String>,
}

/// What happened.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum HelpSignalKind {
    /// A field's error appeared.
    ErrorShown,
    /// A field's error went away.
    ErrorCleared,
    /// The same error through another edit; `edits` since it appeared.
    ErrorPersisted { edits: u32 },
    /// A dialog's affirm action was tried (click or Enter) while not ready.
    SubmitRefused,
    /// A help topic was opened.
    HelpOpened,
    /// A help topic was closed within a second of opening (not by asking).
    HelpDismissed,
    /// A combobox was left with text that matched nothing; `times` in a row.
    NoMatch { times: u32 },
    /// The app's own, with its own name (`HelpReporter::report`).
    App(String),
}

impl HelpSignalKind {
    /// A stable name for a policy or a log: `error_shown`, `help_opened`, …;
    /// the app's own kind as it gave it.
    pub fn name(&self) -> &str {
        match self {
            Self::ErrorShown => "error_shown",
            Self::ErrorCleared => "error_cleared",
            Self::ErrorPersisted { .. } => "error_persisted",
            Self::SubmitRefused => "submit_refused",
            Self::HelpOpened => "help_opened",
            Self::HelpDismissed => "help_dismissed",
            Self::NoMatch { .. } => "no_match",
            Self::App(kind) => kind,
        }
    }
}

// ---------------------------------------------------------------------------
// HelpSignals
// ---------------------------------------------------------------------------

/// The observer, as seen by a reporter.
#[derive(Clone, Copy)]
struct Observer(UnsyncCallback<HelpSignal>);

/// Receives every [`HelpSignal`] reported inside it: by its components
/// (an error that persists, a refused submit, help dismissed at once, …)
/// and by the app ([`help_signals`]). Without one, nothing is recorded.
///
/// `on_signal` is where the app's policy lives: count, set thresholds and
/// cool-downs, then offer help without interrupting. Signals stay in the
/// page unless `on_signal` sends them somewhere.
///
/// ```ignore
/// <HelpSignals on_signal=move |signal: HelpSignal| policy.observe(signal)>
///     <Router>…</Router>
/// </HelpSignals>
/// ```
#[component]
pub fn HelpSignals(#[prop(into)] on_signal: UnsyncCallback<HelpSignal>, children: Children) -> impl IntoView {
    view! { <Provider value=Observer(on_signal)>{children()}</Provider> }
}

/// Reports signals to the enclosing [`HelpSignals`], with the context of
/// the `HelpScope`s where it was made. Outside any, reporting does nothing.
/// Made in a component's body (where context is found), used anywhere
/// later, an event handler included.
#[derive(Clone, Copy)]
pub struct HelpReporter {
    observer: Option<Observer>,
    context: Signal<Vec<(String, String)>>,
}

/// A reporter for here: call it in a component's body, keep it, and report
/// from handlers.
///
/// ```ignore
/// let signals = help_signals();
/// let on_check = move || if !balanced() { signals.report("imbalance", Some(difference())) };
/// ```
pub fn help_signals() -> HelpReporter {
    HelpReporter { observer: use_context::<Observer>(), context: scope_context() }
}

impl HelpReporter {
    /// Reports the app's own signal: `kind` names it for the policy;
    /// `detail` goes with it as given (only what the user would expect an
    /// assistant to see).
    pub fn report(&self, kind: impl Into<String>, detail: Option<String>) {
        self.emit(HelpSignalKind::App(kind.into()), "app", String::new(), None, detail);
    }

    /// Whether a [`HelpSignals`] receives what this reports.
    pub fn is_observed(&self) -> bool {
        self.observer.is_some()
    }

    pub(crate) fn emit(
        &self,
        kind: HelpSignalKind,
        component: &str,
        about: String,
        topic_id: Option<String>,
        detail: Option<String>,
    ) {
        let Some(observer) = self.observer else { return };
        observer.0.run(HelpSignal {
            kind,
            component: component.to_string(),
            about,
            topic_id,
            context: self.context.get_untracked(),
            detail,
        });
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;

    #[test]
    fn names() {
        assert_eq!(HelpSignalKind::ErrorPersisted { edits: 3 }.name(), "error_persisted");
        assert_eq!(HelpSignalKind::HelpDismissed.name(), "help_dismissed");
        assert_eq!(HelpSignalKind::App("imbalance".into()).name(), "imbalance");
    }

    #[test]
    fn outside_help_signals_reporting_does_nothing() {
        Owner::new().with(|| {
            let signals = help_signals();
            assert!(!signals.is_observed());
            signals.report("imbalance", None);
        });
    }

    #[test]
    fn a_report_reaches_the_observer_with_its_kind_and_detail() {
        Owner::new().with(|| {
            let seen = Rc::new(RefCell::new(Vec::new()));
            let into = seen.clone();
            provide_context(Observer(UnsyncCallback::new(move |s: HelpSignal| into.borrow_mut().push(s))));
            let signals = help_signals();
            assert!(signals.is_observed());
            signals.report("imbalance", Some("0.50".into()));
            let seen = seen.borrow();
            assert_eq!(seen.len(), 1);
            assert_eq!(seen[0].kind, HelpSignalKind::App("imbalance".into()));
            assert_eq!(seen[0].component, "app");
            assert_eq!(seen[0].detail.as_deref(), Some("0.50"));
            assert!(seen[0].context.is_empty());
        });
    }
}
