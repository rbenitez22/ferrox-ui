use std::ops::Range;

use ferrox_icons::{Icon, CHEVRON_LEFT, CHEVRON_RIGHT};
use leptos::html;
use leptos::prelude::*;
use leptos::{component, view, IntoView};

use crate::IconButton;

/// The items on each page: `len` items, `per_page` to a page (`0` counts as
/// `1`). No items, no pages.
pub fn carousel_pages(len: usize, per_page: usize) -> Vec<Range<usize>> {
    let per_page = per_page.max(1);
    (0..len.div_ceil(per_page)).map(|i| i * per_page..((i + 1) * per_page).min(len)).collect()
}

/// The page (1-based) a track scrolled to `scroll_left` shows, each page
/// `width` wide: the nearest one, clamped into `1..=count`.
pub fn page_at(scroll_left: f64, width: f64, count: usize) -> usize {
    if width <= 0.0 {
        return 1;
    }
    let nearest = (scroll_left / width).round().max(0.0) as usize + 1;
    nearest.clamp(1, count.max(1))
}

/// The words of a [`Carousel`]. `page` and `slide` are templates: `{n}` is a
/// page number, `{count}` the number of pages. Change some with
/// `CarouselLabels { next: "More".into(), ..Default::default() }`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CarouselLabels {
    pub previous: String,
    pub next: String,
    /// A dot's name: `"Page {n} of {count}"`.
    pub page: String,
    /// A page's name, read when moving into it: `"{n} of {count}"`.
    pub slide: String,
}

impl Default for CarouselLabels {
    fn default() -> Self {
        Self {
            previous: "Previous page".into(),
            next: "Next page".into(),
            page: "Page {n} of {count}".into(),
            slide: "{n} of {count}".into(),
        }
    }
}

impl CarouselLabels {
    pub fn page_label(&self, n: usize, count: usize) -> String {
        fill(&self.page, n, count)
    }

    pub fn slide_label(&self, n: usize, count: usize) -> String {
        fill(&self.slide, n, count)
    }
}

fn fill(template: &str, n: usize, count: usize) -> String {
    template.replace("{n}", &n.to_string()).replace("{count}", &count.to_string())
}

/// Items in pages, swiped sideways: a launcher's tiles, a gallery.
///
/// - The items are split into pages of `per_page`; each page is a `<ul>`
///   laid out by `page_class` (the app's grid).
/// - Swiping is the browser's: the track scrolls sideways and snaps to a
///   page, so touch, trackpad and Shift+wheel all work.
/// - Previous / Next and one dot per page scroll to a page, and follow a
///   swipe. With one page there are no controls.
/// - `page` (1-based, two-way) is optional: pass one to own the page.
///
/// ```ignore
/// <Carousel items=apps key_of=|a: &App| a.id.clone() item=|a: App| view! { <AppTile app=a /> }
///     per_page=8 page_class="launcher__grid" label="Apps" />
/// ```
#[component]
pub fn Carousel<T, IV>(
    #[prop(into)] items: Signal<Vec<T>>,
    /// Stable identity per item.
    key_of: impl Fn(&T) -> String + Clone + Send + Sync + 'static,
    /// Renders one item.
    item: impl Fn(T) -> IV + Clone + Send + Sync + 'static,
    /// How many items to a page.
    per_page: usize,
    /// The carousel's accessible name: "Apps".
    #[prop(into)] label: Signal<String>,
    /// The current page, 1-based. Two-way; the carousel keeps its own if none.
    #[prop(optional)] page: Option<RwSignal<usize>>,
    #[prop(optional)] labels: CarouselLabels,
    /// Classes on each page's `<ul>`: the app's layout for one page.
    #[prop(optional, into)] page_class: MaybeProp<String>,
    /// Extra classes on the carousel. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
    IV: IntoView + 'static,
{
    let page = page.unwrap_or_else(|| RwSignal::new(1));
    let labels = StoredValue::new(labels);
    let track = NodeRef::<html::Div>::new();
    let count = Memo::new(move |_| items.with(|items| carousel_pages(items.len(), per_page).len()));
    let current = Memo::new(move |_| page.get().clamp(1, count.get().max(1)));
    let go = move |n: usize| page.set(n.clamp(1, count.get_untracked().max(1)));

    // A page set by code or a control: scroll to it, unless the track is
    // there already (a swipe set it).
    Effect::new(move |_| {
        let n = current.get();
        let Some(track) = track.get() else { return };
        let width = track.client_width();
        if page_at(f64::from(track.scroll_left()), f64::from(width), count.get_untracked()) != n {
            track.set_scroll_left((n as i32 - 1) * width);
        }
    });
    // A swipe: the dots follow the track.
    let on_scroll = move |_| {
        let Some(track) = track.get_untracked() else { return };
        let n = page_at(f64::from(track.scroll_left()), f64::from(track.client_width()), count.get_untracked());
        if n != current.get_untracked() {
            page.set(n);
        }
    };

    let pages = move || (1..=count.get()).collect::<Vec<_>>();
    let page_items = move |n: usize| {
        move || {
            items.with(|items| {
                carousel_pages(items.len(), per_page).get(n - 1).map_or_else(Vec::new, |range| items[range.clone()].to_vec())
            })
        }
    };

    view! {
        <section
            class=move || match class.get() {
                Some(extra) if !extra.is_empty() => format!("carousel {extra}"),
                _ => "carousel".to_string(),
            }
            aria-roledescription="carousel"
            aria-label=move || label.get()
        >
            <div class="carousel__track" node_ref=track on:scroll=on_scroll>
                <For each=pages key=|n| *n let:n>
                    {
                        let key_of = key_of.clone();
                        let item = item.clone();
                        view! {
                            <div
                                class="carousel__page"
                                role="group"
                                aria-roledescription="slide"
                                aria-label=move || labels.with_value(|l| l.slide_label(n, count.get()))
                            >
                                <ul class=move || match page_class.get() {
                                    Some(extra) if !extra.is_empty() => format!("carousel__items {extra}"),
                                    _ => "carousel__items".to_string(),
                                }>
                                    <For each=page_items(n) key=key_of.clone() let:entry>
                                        <li class="carousel__item">{item(entry)}</li>
                                    </For>
                                </ul>
                            </div>
                        }
                    }
                </For>
            </div>
            {move || (count.get() > 1).then(|| view! {
                <div class="carousel__controls">
                    <IconButton
                        label=labels.with_value(|l| l.previous.clone())
                        disabled=Signal::derive(move || current.get() <= 1)
                        on_click=move || go(current.get_untracked() - 1)
                    >
                        <Icon icon=CHEVRON_LEFT />
                    </IconButton>
                    <div class="carousel__dots">
                        <For each=pages key=|n| *n let:n>
                            <button
                                type="button"
                                class="carousel__dot"
                                aria-label=move || labels.with_value(|l| l.page_label(n, count.get()))
                                aria-current=move || (current.get() == n).then_some("page")
                                on:click=move |_| go(n)
                            ></button>
                        </For>
                    </div>
                    <IconButton
                        label=labels.with_value(|l| l.next.clone())
                        disabled=Signal::derive(move || current.get() >= count.get())
                        on_click=move || go(current.get_untracked() + 1)
                    >
                        <Icon icon=CHEVRON_RIGHT />
                    </IconButton>
                </div>
            })}
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn items_split_into_pages_the_last_one_short() {
        assert_eq!(carousel_pages(9, 8), vec![0..8, 8..9]);
        assert_eq!(carousel_pages(16, 8), vec![0..8, 8..16]);
        assert_eq!(carousel_pages(3, 8), vec![0..3]);
    }

    #[test]
    fn no_items_no_pages_and_zero_per_page_counts_as_one() {
        assert!(carousel_pages(0, 8).is_empty());
        assert_eq!(carousel_pages(2, 0), vec![0..1, 1..2]);
    }

    #[test]
    fn the_nearest_page_to_the_scroll_position() {
        assert_eq!(page_at(0.0, 300.0, 3), 1);
        assert_eq!(page_at(140.0, 300.0, 3), 1);
        assert_eq!(page_at(160.0, 300.0, 3), 2);
        assert_eq!(page_at(600.0, 300.0, 3), 3);
    }

    #[test]
    fn page_at_stays_in_range() {
        assert_eq!(page_at(5000.0, 300.0, 3), 3);
        assert_eq!(page_at(-40.0, 300.0, 3), 1);
        assert_eq!(page_at(100.0, 0.0, 3), 1);
        assert_eq!(page_at(0.0, 300.0, 0), 1);
    }

    #[test]
    fn labels_fill_their_templates() {
        let labels = CarouselLabels::default();
        assert_eq!(labels.page_label(2, 3), "Page 2 of 3");
        assert_eq!(labels.slide_label(1, 2), "1 of 2");
    }
}
