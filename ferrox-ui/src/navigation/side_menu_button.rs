use ferrox_icons::{Icon, MENU};
use leptos::prelude::*;
use leptos::{component, view, IntoView};

use crate::IconButton;

/// The button that opens a [`SideMenu`](crate::SideMenu): an `IconButton`
/// drawing `MENU`, named by `label`, with `aria-haspopup="dialog"` and
/// `aria-expanded` following `open`.
///
/// ```ignore
/// <SideMenuButton open=menu />
/// ```
#[component]
pub fn SideMenuButton(
    /// The menu's open state, shared with its `SideMenu`.
    open: RwSignal<bool>,
    /// The button's only label (also its tooltip).
    #[prop(into, default = "Menu".into())] label: Signal<String>,
    /// Extra classes, appended to the button's own. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <IconButton
            label=label
            class=class
            on_click=move || open.set(true)
            attr:aria-haspopup="dialog"
            attr:aria-expanded=move || open.get().to_string()
        >
            <Icon icon=MENU />
        </IconButton>
    }
}
