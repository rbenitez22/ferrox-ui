use leptos::context::Provider;
use leptos::html;
use leptos_router::hooks::use_location;
use leptos::prelude::*;
use leptos::{component, view, IntoView};

use web_sys::MouseEvent;

use crate::ids::unique_id;
use crate::layout::ShellContext;
use crate::{AppBar, SideMenu, SideMenuButton};

/// The app's frame: a skip link, the [`AppBar`] (its menu button, the app's
/// name, `bar_end`), the [`SideMenu`] holding `nav`, then the page in the
/// app's one `<main>`. The app brings the content (its links, its user
/// menu, its pages); the frame and how it behaves are the same everywhere.
///
/// - The menu button and the side menu share their open state; the menu
///   closes itself when a link in it is followed.
/// - The skip link is the first Tab stop: hidden until focused, it moves
///   focus to the page.
/// - A [`PageGrid`](crate::PageGrid) (or a `Page`) inside doesn't add a
///   second `<main>`.
/// - It must be inside a `<Router>`, around the routes' `<Outlet/>`.
///
/// ```ignore
/// <AppShell title="Cookbook" href="/" nav=|| view! { <NavLink href="/">"Recipes"</NavLink> }>
///     <Outlet />
/// </AppShell>
/// ```
#[component]
pub fn AppShell(
    /// The app's name: in the bar and as the side menu's title.
    #[prop(into)] title: Signal<String>,
    /// Where the name links (usually "/").
    #[prop(optional, into)] href: MaybeProp<String>,
    /// The side menu's links (`NavLink`, `NavGroup`).
    #[prop(into)] nav: ViewFn,
    /// The bar's end: the app's actions, a user `Menu`.
    #[prop(optional, into)] bar_end: Option<ViewFnOnce>,
    /// Pinned to the bottom of the side menu.
    #[prop(optional, into)] menu_footer: Option<ViewFn>,
    #[prop(into, default = "Skip to content".into())] skip_label: Signal<String>,
    /// The menu button's label.
    #[prop(into, default = "Menu".into())] menu_label: Signal<String>,
    /// The name of the side menu's `<nav>`.
    #[prop(into, default = "Sections".into())] nav_label: Signal<String>,
    /// The page: usually the router's `<Outlet/>`.
    children: Children,
) -> impl IntoView {
    let menu = RwSignal::new(false);
    let location = use_location();
    let path = Signal::derive(move || location.pathname.get());
    let main_id = unique_id("ferrox-main");
    let skip_to = format!("#{main_id}");
    let main_ref = NodeRef::<html::Main>::new();
    // The router takes over a `#` link's click, so the browser never moves
    // focus to the target: the link does it.
    let skip = move |ev: MouseEvent| {
        let Some(main) = main_ref.get_untracked() else { return };
        ev.prevent_default();
        let _ = main.focus();
        main.scroll_into_view();
    };
    let start = move || view! { <SideMenuButton open=menu label=menu_label /> };
    let bar = match bar_end {
        Some(end) => view! { <AppBar title=title href=href start=start>{end.run()}</AppBar> }.into_any(),
        None => view! { <AppBar title=title href=href start=start /> }.into_any(),
    };
    let side_menu = match menu_footer {
        Some(footer) => view! { <SideMenu open=menu title=title nav_label=nav_label footer=footer>{nav.run()}</SideMenu> }.into_any(),
        None => view! { <SideMenu open=menu title=title nav_label=nav_label>{nav.run()}</SideMenu> }.into_any(),
    };
    view! {
        <div class="app-shell">
            <a class="app-shell__skip" href=skip_to on:click=skip>{move || skip_label.get()}</a>
            {bar}
            {side_menu}
            <main class="app-shell__main" id=main_id tabindex="-1" node_ref=main_ref>
                <Provider value=ShellContext::new(title, path)>{children()}</Provider>
            </main>
        </div>
    }
}
