// A fixture for ferrox-check: an app with no drift.
use ferrox_ui::{Button, Field, Input, PageGrid};
use leptos::prelude::*;

fn main() {
    ferrox_theme::init();
    ferrox_ui::init();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let name = RwSignal::new(String::new());
    view! {
        <PageGrid>
            <Field label="Name"><Input value=name /></Field>
            <Button on_click=move || ()>"Save"</Button>
            // ferrox-check: allow raw-input
            <input type="file" />
        </PageGrid>
    }
}
