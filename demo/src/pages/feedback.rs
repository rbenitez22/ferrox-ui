//! Feedback: spinners and the loading overlay (later alerts, skeletons and
//! toasts), so each pattern can be checked in every theme.

use std::time::Duration;

use ferrox_icons::{Icon, HELP, SEARCH};
use ferrox_ui::{Button, IconButton, LoadingOverlay, Spinner, SpinnerKind};
use leptos::prelude::*;

use crate::section::Section;

const OVERLAY_MS: u64 = 2000;

#[component]
pub fn FeedbackPage() -> impl IntoView {
    view! {
        <Spinners />
        <Overlay />
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
