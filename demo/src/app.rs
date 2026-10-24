//! Router and page frame: a header with the menu button and page links, the
//! side menu, the theme bar, then the page.

use ferrox_theme::provide_theme;
use ferrox_ui::SideMenuButton;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::path;

use crate::menu::DemoMenu;
use crate::pages::{buttons::ButtonsPage, feedback::FeedbackPage, forms::FormsPage, help::HelpPage, icons::IconsPage, layout::LayoutPage, navigation::NavigationPage, overlay::OverlayPage, themes::ThemesPage};
use crate::theme_bar::ThemeBar;

#[component]
pub fn App() -> impl IntoView {
    provide_theme();
    let menu = RwSignal::new(false);
    view! {
        <Router>
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
                    </nav>
                </header>
                <DemoMenu open=menu />
                <ThemeBar />
                <main class="demo-main">
                    <Routes fallback=|| view! { <p>"Page not found."</p> }>
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
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
