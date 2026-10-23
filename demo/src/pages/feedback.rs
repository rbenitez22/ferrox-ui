//! Feedback: spinners, the loading overlay and alerts (later skeletons and
//! toasts), so each pattern can be checked in every theme.

use std::time::Duration;

use ferrox_icons::{Icon, HELP, SEARCH};
use ferrox_ui::{Alert, AlertSeverity, Button, IconButton, LoadingOverlay, Spinner, SpinnerKind};
use leptos::prelude::*;

use crate::section::Section;

const OVERLAY_MS: u64 = 2000;

#[component]
pub fn FeedbackPage() -> impl IntoView {
    view! {
        <Spinners />
        <Overlay />
        <Alerts />
    }
}

/// Each kind where it belongs. Hidden until asked for, so the delayed
/// reveal can be watched (and tested) from the moment a spinner mounts.
#[component]
fn Spinners() -> impl IntoView {
    let shown = RwSignal::new(false);
    view! {
        <Section title="Spinners">
            <div class="demo-row">
                <Button on_click=move |_| shown.update(|s| *s = !*s)>
                    {move || if shown.get() { "Hide spinners" } else { "Show spinners" }}
                </Button>
            </div>
            <Show when=move || shown.get()>
                <h3 class="demo-role-name">"region"</h3>
                <div class="demo-region">
                    <Spinner label="Loading accounts" />
                </div>
                <h3 class="demo-role-name">"inline (in a row action's place)"</h3>
                <div class="demo-row">
                    <IconButton title="Search"><Icon icon=SEARCH /></IconButton>
                    <Spinner kind=SpinnerKind::Inline label="Deleting the row" />
                    <IconButton title="Help"><Icon icon=HELP /></IconButton>
                </div>
                <h3 class="demo-role-name">"field"</h3>
                <div class="demo-row">
                    <label class="demo-field">
                        "Rate"
                        <input value="1.0842" readonly=true />
                        <Spinner kind=SpinnerKind::Field label="Looking up the rate" />
                    </label>
                </div>
            </Show>
        </Section>
    }
}

/// The full-page overlay, shown for two seconds.
#[component]
fn Overlay() -> impl IntoView {
    let shown = RwSignal::new(false);
    let show = move |_| {
        shown.set(true);
        set_timeout(move || shown.set(false), Duration::from_millis(OVERLAY_MS));
    };
    view! {
        <Section title="Loading overlay">
            <p>"Only for when nothing on screen is valid any more: boot, auth, the post-login redirect."</p>
            <div class="demo-row">
                <Button on_click=show>"Show the overlay for 2s"</Button>
            </div>
            <Show when=move || shown.get()>
                <LoadingOverlay text="Signing in…" />
            </Show>
        </Section>
    }
}

/// Each severity, one without its icon, and one that can be dismissed.
#[component]
fn Alerts() -> impl IntoView {
    let dismissed = RwSignal::new(false);
    view! {
        <Section title="Alerts">
            <div class="demo-stack">
                <Alert severity=AlertSeverity::Info>"Changes apply to new entries only."</Alert>
                <Alert severity=AlertSeverity::Success>"The statement was reconciled."</Alert>
                <Alert severity=AlertSeverity::Warning>"This account has no transactions this year."</Alert>
                <Alert severity=AlertSeverity::Error>"The server refused the change: the period is closed."</Alert>
                <Alert severity=AlertSeverity::Info icon=false>"No icon: icon=false."</Alert>
                <Show
                    when=move || !dismissed.get()
                    fallback=move || view! {
                        <div class="demo-row">
                            <Button on_click=move |_| dismissed.set(false)>"Show the dismissible alert again"</Button>
                        </div>
                    }
                >
                    <Alert severity=AlertSeverity::Error on_dismiss=move |_| dismissed.set(true)>
                        "Dismissible: the close button calls on_dismiss, and the page removes the alert."
                    </Alert>
                </Show>
            </div>
        </Section>
    }
}
