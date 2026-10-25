use leptos::context::Provider;
use leptos::ev::keydown;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, KeyboardEvent};

/// Marks that F1 works here, for the `?`s' `aria-keyshortcuts`.
#[derive(Clone, Copy)]
struct HelpKeysOn;

/// The shortcuts a `?` here answers, for its `aria-keyshortcuts`: none
/// outside `HelpKeys`; F1, and Shift+F1 when there's an assistant to ask.
pub(crate) fn help_keys_shortcuts(has_assistant: bool) -> Option<&'static str> {
    use_context::<HelpKeysOn>().map(|_| if has_assistant { "F1 Shift+F1" } else { "F1" })
}

/// F1 opens the help of whatever has focus; Shift+F1 asks the assistant
/// about it directly (as F1, without a [`HelpAssistant`](crate::HelpAssistant)).
///
/// - It finds the help from the focused element's closest `[data-help]`:
///   every component with a `help` prop marks itself, so the innermost wins
///   (a field inside a dialog opens the field's help). For a bare
///   [`HelpButton`](crate::HelpButton), give it an `id` and put
///   `data-help="<that id>"` on what it explains.
/// - Opt-in, since it takes over a key: inside, the browser's own F1 never
///   runs, even where there's no help. Place it once, around the app.
/// - A shortcut on top of the visible `?`, never the only way in: on macOS,
///   F1 is often a brightness key.
///
/// ```ignore
/// <HelpKeys>
///     <Router>…</Router>
/// </HelpKeys>
/// ```
#[component]
pub fn HelpKeys(children: Children) -> impl IntoView {
    let listener = window_event_listener(keydown, on_key);
    on_cleanup(move || listener.remove());
    view! { <Provider value=HelpKeysOn>{children()}</Provider> }
}

fn on_key(event: KeyboardEvent) {
    if event.key() != "F1" || event.ctrl_key() || event.alt_key() || event.meta_key() {
        return;
    }
    event.prevent_default();
    let Some(button) = help_for_focus() else { return };
    if event.shift_key() && let Some(ask) = ask_button(&button) {
        ask.click();
        return;
    }
    button.click();
}

/// The `?` for the focused element: its closest `[data-help]` names it.
/// None inside an open topic, where F1 has nothing more to open.
fn help_for_focus() -> Option<HtmlElement> {
    let active = document().active_element()?;
    if active.closest(".help-popover").ok().flatten().is_some() {
        return None;
    }
    let id = active.closest("[data-help]").ok()??.get_attribute("data-help")?;
    document().get_element_by_id(&id)?.dyn_into().ok()
}

/// The "Ask about this" in a `?`'s topic, if there's an assistant.
fn ask_button(button: &HtmlElement) -> Option<HtmlElement> {
    let popover = document().get_element_by_id(&button.get_attribute("aria-controls")?)?;
    popover.query_selector(".help-popover__ask").ok()??.dyn_into().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortcuts_only_inside_help_keys() {
        Owner::new().with(|| {
            assert_eq!(help_keys_shortcuts(true), None);
            provide_context(HelpKeysOn);
            assert_eq!(help_keys_shortcuts(false), Some("F1"));
            assert_eq!(help_keys_shortcuts(true), Some("F1 Shift+F1"));
        });
    }
}
