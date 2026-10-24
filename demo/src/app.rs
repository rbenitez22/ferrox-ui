//! Router and page frames: the demo's (a header with the menu button and page
//! links, the side menu, the theme bar, then the page), and the recipes',
//! a small app of its own.

use ferrox_theme::provide_theme;
use ferrox_ui::SideMenuButton;
use leptos::prelude::*;
use leptos_router::components::{Outlet, ParentRoute, Route, Router, Routes, A};
use leptos_router::path;

use crate::menu::DemoMenu;
use crate::recipes::{DetailPage, ListPage, RecipeShell, SettingsPage};
use crate::pages::{buttons::ButtonsPage, data::DataPage, feedback::FeedbackPage, forms::FormsPage, help::HelpPage, icons::IconsPage, layout::LayoutPage, menus::MenusPage, navigation::NavigationPage, overlay::OverlayPage, themes::ThemesPage};
use crate::theme_bar::ThemeBar;

#[component]
pub fn App() -> impl IntoView {
    provide_theme();
    view! {
        <Router>
            <Routes fallback=|| view! { <p>"Page not found."</p> }>
                // The recipes are a small app of their own, with their own
                // shell: outside the demo's header and <main>.
                <ParentRoute path=path!("/recipes") view=RecipeShell>
                    <Route path=path!("") view=ListPage />
                    <Route path=path!("settings") view=SettingsPage />
                    <Route path=path!(":id") view=DetailPage />
                </ParentRoute>
                <ParentRoute path=path!("") view=DemoShell>
                    <Route path=path!("/") view=ThemesPage />
                    <Route path=path!("/buttons") view=ButtonsPage />
                    <Route path=path!("/icons") view=IconsPage />
                    <Route path=path!("/feedback") view=FeedbackPage />
                    <Route path=path!("/overlay") view=OverlayPage />
                    <Route path=path!("/forms") view=FormsPage />
                    <Route path=path!("/help") view=HelpPage />
                    <Route path=path!("/navigation") view=NavigationPage />
                    <Route path=path!("/navigation/details") view=NavigationPage />
                    <Route path=path!("/layout") view=LayoutPage />
                    <Route path=path!("/data") view=DataPage />
                    <Route path=path!("/menus") view=MenusPage />
                </ParentRoute>
            </Routes>
        </Router>
    }
}

/// The demo's frame: a header with the menu button and page links, the side
/// menu, the theme bar, then the page.
#[component]
fn DemoShell() -> impl IntoView {
    let menu = RwSignal::new(false);
    view! {
        <div class="demo-app">
            <header class="demo-header">
                <SideMenuButton open=menu />
                <span class="demo-title">"ferrox-ui demo"</span>
                <nav class="demo-nav">
                    <A href="/">"Themes"</A>
                    <A href="/buttons">"Buttons"</A>
                    <A href="/icons">"Icons"</A>
                    <A href="/feedback">"Feedback"</A>
                    <A href="/overlay">"Overlay"</A>
                    <A href="/forms">"Forms"</A>
                    <A href="/help">"Help"</A>
                    <A href="/navigation">"Navigation"</A>
                    <A href="/layout">"Layout"</A>
                    <A href="/data">"Data"</A>
                    <A href="/menus">"Menus"</A>
                    <A href="/recipes">"Recipes"</A>
                </nav>
            </header>
            <DemoMenu open=menu />
            <ThemeBar />
            <main class="demo-main">
                <Outlet />
            </main>
        </div>
    }
}
