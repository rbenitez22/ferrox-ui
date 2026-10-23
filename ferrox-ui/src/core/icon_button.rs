use leptos::prelude::*;
use leptos::{component, view, IntoView};
use web_sys::MouseEvent;

/// The class list for an `.icon-btn`.
pub fn icon_button_classes(danger: bool, extra: &str) -> String {
    let mut classes = vec!["icon-btn"];
    if danger { classes.push("icon-btn--danger"); }
    if !extra.is_empty() { classes.push(extra); }
    classes.join(" ")
}

/// A compact glyph button (`.icon-btn`): 1.75rem square, no border,
/// answers in colour rather than size. For toolbars and row actions.
///
/// - `title` is required: the glyph has no text, so it is the button's only
///   label. It is also set as `aria-label`.
/// - `pressed` makes it a switch (`aria-pressed`); on reads as tinted.
/// - `children` is the icon (an SVG; see the SVG icon story).
///
/// ```ignore
/// <IconButton title="Search" on_click=move |_| open_search()>
///     <svg class="icon" …/>
/// </IconButton>
/// <IconButton title="Camera" pressed=camera_on on_click=move |_| camera_on.update(|v| *v = !*v)>…</IconButton>
/// <IconButton title="Remove line" danger=true on_click=move |_| remove()>…</IconButton>
/// ```
#[component]
pub fn IconButton(
    #[prop(into)] title: String,
    #[prop(optional, into)] pressed: Option<Signal<bool>>,
    #[prop(optional)] danger: bool,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] on_click: Option<UnsyncCallback<MouseEvent>>,
    /// Extra classes, appended to the generated ones. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let on_click = move |ev: MouseEvent| {
        if let Some(cb) = &on_click { cb.run(ev); }
    };

    view! {
        <button
            type="button"
            class=move || icon_button_classes(danger, &class.get().unwrap_or_default())
            title=title.clone()
            aria-label=title
            aria-pressed=move || pressed.map(|p| if p.get() { "true" } else { "false" })
            disabled=move || disabled.get()
            on:click=on_click
        >
            {children()}
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_and_danger_classes() {
        assert_eq!(icon_button_classes(false, ""), "icon-btn");
        assert_eq!(icon_button_classes(true, "extra"), "icon-btn icon-btn--danger extra");
    }
}
