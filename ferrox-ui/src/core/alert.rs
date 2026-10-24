use ferrox_icons::{Icon, IconData, CLOSE, ERROR, INFO, SUCCESS, WARNING};
use leptos::prelude::*;
use leptos::{component, view, IntoView};

use super::IconButton;

// ---------------------------------------------------------------------------
// AlertSeverity
//
// Status roles only: an alert reports a state, it does not act, so there is
// no primary alert. `Warning` is the non-blocking tier; if something should
// stop a submit, it is an `Error`.
// ---------------------------------------------------------------------------

/// How serious an [`Alert`] is: its colour, its icon, and its role (`status`
/// for info and success, `alert` for warning and error).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum AlertSeverity {
    Info,
    Success,
    /// Non-blocking: never prevents a submit.
    Warning,
    /// Announced at once (`role="alert"`).
    Error,
}

impl AlertSeverity {
    pub fn class(self) -> &'static str {
        match self {
            Self::Info    => "alert--info",
            Self::Success => "alert--success",
            Self::Warning => "alert--warning",
            Self::Error   => "alert--error",
        }
    }

    /// The icon drawn before the text.
    pub fn icon(self) -> IconData {
        match self {
            Self::Info    => INFO,
            Self::Success => SUCCESS,
            Self::Warning => WARNING,
            Self::Error   => ERROR,
        }
    }

    /// `alert` interrupts a screen reader; `status` waits its turn. Only an
    /// error interrupts.
    pub fn role(self) -> &'static str {
        match self {
            Self::Error => "alert",
            _ => "status",
        }
    }
}

/// The class list for an `.alert`.
pub fn alert_classes(severity: AlertSeverity, extra: &str) -> String {
    let mut classes = vec!["alert", severity.class()];
    if !extra.is_empty() { classes.push(extra); }
    classes.join(" ")
}

// ---------------------------------------------------------------------------
// Alert
// ---------------------------------------------------------------------------

/// An in-page or in-form message that **never times out**: the reason a
/// submit failed, a server refusal no single field owns. Messages that go by
/// themselves are toasts (`overlay`, later); anything the user must act on is
/// an alert.
///
/// - `icon` draws the severity's icon before the text (default `true`), so
///   severity does not rest on colour alone.
/// - `on_close` adds a close button that calls it; the caller removes the
///   alert. Without it there is no button.
/// - `close_label` is that button's label (default `"Close"`).
///
/// ```ignore
/// <Alert severity=AlertSeverity::Error>"The server refused the change."</Alert>
/// <Alert severity=AlertSeverity::Info icon=false>"Changes apply to new entries only."</Alert>
/// {move || error.get().map(|e| view! {
///     <Alert severity=AlertSeverity::Error on_close=move || error.set(None)>{e}</Alert>
/// })}
/// ```
#[component]
pub fn Alert(
    severity: AlertSeverity,
    #[prop(default = true)] icon: bool,
    #[prop(optional, into)] on_close: Option<UnsyncCallback<()>>,
    #[prop(into, default = "Close".into())] close_label: Signal<String>,
    /// Extra classes, appended to the generated ones. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || alert_classes(severity, &class.get().unwrap_or_default()) role=severity.role()>
            {icon.then(|| view! { <Icon icon=severity.icon() class="alert__icon" /> })}
            <div class="alert__body">{children()}</div>
            {on_close.map(|close| view! {
                <IconButton label=close_label class="alert__close" on_click=move || close.run(())>
                    <Icon icon=CLOSE />
                </IconButton>
            })}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes() {
        assert_eq!(alert_classes(AlertSeverity::Info, ""), "alert alert--info");
        assert_eq!(alert_classes(AlertSeverity::Error, "extra"), "alert alert--error extra");
    }

    #[test]
    fn only_an_error_interrupts() {
        assert_eq!(AlertSeverity::Error.role(), "alert");
        for severity in [AlertSeverity::Info, AlertSeverity::Success, AlertSeverity::Warning] {
            assert_eq!(severity.role(), "status");
        }
    }

    #[test]
    fn each_severity_has_its_own_icon() {
        assert_eq!(AlertSeverity::Info.icon(), INFO);
        assert_eq!(AlertSeverity::Success.icon(), SUCCESS);
        assert_eq!(AlertSeverity::Warning.icon(), WARNING);
        assert_eq!(AlertSeverity::Error.icon(), ERROR);
    }
}
