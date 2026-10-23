//! A titled block on a demo page.

use leptos::prelude::*;

#[component]
pub fn Section(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <section class="demo-section">
            <h2>{title}</h2>
            {children()}
        </section>
    }
}
