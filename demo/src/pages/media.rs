//! Media: a carousel of 20 tiles, 6 to a page, with its page echoed and set
//! from code, and one that fits on a single page (no controls).

use ferrox_ui::{Button, ButtonFamily, Card, Carousel};
use leptos::prelude::*;

use crate::section::Section;

/// Tiles 1..=`count`, as the carousels' items.
fn tiles(count: usize) -> Vec<usize> {
    (1..=count).collect()
}

/// A tile with a button in it, so the keyboard can reach every page.
fn tile(n: usize) -> impl IntoView {
    view! {
        <Card title=format!("Tile {n}") compact=true>
            <button type="button" class="btn btn--secondary btn--sm">{format!("Open {n}")}</button>
        </Card>
    }
}

#[component]
pub fn MediaPage() -> impl IntoView {
    view! {
        <Paged />
        <OnePage />
    }
}

#[component]
fn Paged() -> impl IntoView {
    let page = RwSignal::new(1_usize);
    view! {
        <Section title="Carousel">
            <div class="demo-row">
                <Button family=ButtonFamily::Secondary on_click=move || page.set(3)>"Go to page 3 from code"</Button>
            </div>
            <Carousel
                items=Signal::stored(tiles(20))
                key_of=|n: &usize| n.to_string()
                item=tile
                per_page=6
                page=page
                page_class="demo-tiles"
                label="Tiles"
            />
            <dl class="demo-echo">
                <dt>"page"</dt><dd data-echo="carousel-page">{move || page.get().to_string()}</dd>
            </dl>
        </Section>
    }
}

#[component]
fn OnePage() -> impl IntoView {
    view! {
        <Section title="One page">
            <Carousel
                items=Signal::stored(tiles(4))
                key_of=|n: &usize| n.to_string()
                item=tile
                per_page=6
                page_class="demo-tiles"
                label="Few tiles"
            />
        </Section>
    }
}
