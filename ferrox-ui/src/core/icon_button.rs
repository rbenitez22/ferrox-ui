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
/// - `label` is required: the glyph has no text, so it is the button's only
///   name. It is set as `aria-label`, and as `title` for a tooltip.
/// - `pressed` makes it a switch (`aria-pressed`); on reads as tinted.
/// - `children` is the icon: ferrox-icons' `Icon`, or any `<svg class="icon">`.
///
/// ```ignore
/// <IconButton label="Search" on_click=move |_| open_search()>
///     <Icon icon=SEARCH/>
/// </IconButton>
/// <IconButton label="Camera" pressed=camera_on on_click=move |_| camera_on.update(|v| *v = !*v)>…</IconButton>
/// <IconButton label="Remove line" danger=true on_click=move |_| remove()>…</IconButton>
/// ```
#[component]
pub fn IconButton(
    /// The button's name (`aria-label`), and its tooltip.
    #[prop(into)] label: Signal<String>,
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
            title=move || label.get()
            aria-label=move || label.get()
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
