//! Router and page frame: a header with page links and the theme bar, then the page.

use ferrox_theme::provide_theme;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes, A};
use leptos_router::path;

use crate::pages::{buttons::ButtonsPage, feedback::FeedbackPage, icons::IconsPage, themes::ThemesPage};
use crate::theme_bar::ThemeBar;

#[component]
pub fn App() -> impl IntoView {
    provide_theme();
    view! {
        <Router>
            <div class="demo-app">
                <header class="demo-header">
                    <span class="demo-title">"ferrox-ui demo"</span>
                    <nav class="demo-nav">
                        <A href="/">"Themes"</A>
                        <A href="/buttons">"Buttons"</A>
                        <A href="/icons">"Icons"</A>
                        <A href="/feedback">"Feedback"</A>
                    </nav>
                </header>
                <ThemeBar />
                <main class="demo-main">
                    <Routes fallback=|| view! { <p>"Page not found."</p> }>
                        <Route path=path!("/") view=ThemesPage />
                        <Route path=path!("/buttons") view=ButtonsPage />
                        <Route path=path!("/icons") view=IconsPage />
                        <Route path=path!("/feedback") view=FeedbackPage />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
