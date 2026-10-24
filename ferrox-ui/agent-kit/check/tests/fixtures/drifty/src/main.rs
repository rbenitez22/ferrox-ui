// A fixture for ferrox-check: every rule broken once, and the setup missing.
use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <button on:click=move |_| ()>"Save"</button>
        <input value="x" />
        <select>
            <option>"One"</option>
        </select>
        <textarea></textarea>
        <dialog open=true>"Hi"</dialog>
        <div role="menu"></div>
        <div style="margin: 4px">"Squeezed"</div>
        // ferrox-check: allow raw-input
        <input type="file" />
        <input type="range" /> // ferrox-check: allow raw-input
        <span>"button and style= in text don't count: only the tags"</span>
    }
}
