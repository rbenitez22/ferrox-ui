use ferrox_icons::{Icon, CHEVRON_LEFT, CHEVRON_RIGHT};
use leptos::prelude::*;
use leptos::{component, view, IntoView};

use crate::IconButton;

/// One slot of a [`Pagination`]: a page number, or a gap standing for two
/// or more pages not shown.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PageItem {
    Page(usize),
    Gap,
}

/// The most slots a pagination shows: first, gap, three around the current,
/// gap, last.
const SLOTS: usize = 7;

/// Which page numbers show for `current` (1-based) of `count` pages.
///
/// Up to seven pages all show. Beyond that it is always seven slots: the
/// first and the last, the current with one neighbour each side, and a gap
/// for each hidden run (never a gap for one page: that page shows instead).
/// The same number of slots on every page, so paging doesn't move the
/// buttons. `current` is clamped into `1..=count`.
pub fn page_window(current: usize, count: usize) -> Vec<PageItem> {
    use PageItem::{Gap, Page};
    if count <= SLOTS {
        return (1..=count).map(Page).collect();
    }
    let current = current.clamp(1, count);
    let gap_before = current > 4;
    let gap_after = current + 3 < count;
    match (gap_before, gap_after) {
        (false, _) => (1..=5).map(Page).chain([Gap, Page(count)]).collect(),
        (true, false) => [Page(1), Gap].into_iter().chain((count - 4..=count).map(Page)).collect(),
        (true, true) => vec![Page(1), Gap, Page(current - 1), Page(current), Page(current + 1), Gap, Page(count)],
    }
}

/// The words of a [`Pagination`]. `page` and `position` are templates:
/// `{n}` is a page number; `{page}` and `{count}` the current page and the
/// count. Change some with `PaginationLabels { next: "Later".into(), ..Default::default() }`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PaginationLabels {
    pub previous: String,
    pub next: String,
    /// A page button's name: `"Page {n}"`.
    pub page: String,
    /// What a phone reads out beside the compact `6 / 20`: `"Page {page} of {count}"`.
    pub position: String,
}

impl Default for PaginationLabels {
    fn default() -> Self {
        Self {
            previous: "Previous page".into(),
            next: "Next page".into(),
            page: "Page {n}".into(),
            position: "Page {page} of {count}".into(),
        }
    }
}

impl PaginationLabels {
    pub fn page_label(&self, n: usize) -> String {
        self.page.replace("{n}", &n.to_string())
    }

    pub fn position_label(&self, page: usize, count: usize) -> String {
        self.position.replace("{page}", &page.to_string()).replace("{count}", &count.to_string())
    }
}

/// The key of a slot in the keyed list: a page by its number, so the
/// button pressed keeps its node (and focus) as the window moves.
fn slot_key(index: usize, item: PageItem) -> String {
    match item {
        PageItem::Page(n) => n.to_string(),
        PageItem::Gap if index < 3 => "gap-before".into(),
        PageItem::Gap => "gap-after".into(),
    }
}

/// Moves through pages of anything (a `DataView`, a table, search results):
/// Previous, the page numbers ([`page_window`]) and Next, in a
/// `<nav aria-label>`.
///
/// - `page` is 1-based and two-way: the app slices or fetches that page,
///   and owns it, so putting it in the URL is the app's choice.
/// - The current page is `aria-current="page"`. Previous and Next are
///   disabled at the ends. With one page or none it renders nothing.
/// - On a phone the numbers give way to `6 / 20`.
///
/// ```ignore
/// let page = RwSignal::new(1);
/// <Pagination page=page page_count=Signal::derive(move || total.get().div_ceil(20)) />
/// ```
#[component]
pub fn Pagination(
    /// The current page, 1-based. Two-way.
    page: RwSignal<usize>,
    #[prop(into)] page_count: Signal<usize>,
    /// The navigation's accessible name.
    #[prop(into, default = "Pages".into())] label: Signal<String>,
    #[prop(optional)] labels: PaginationLabels,
    /// Extra classes on the `<nav>`. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let labels = StoredValue::new(labels);
    let current = Memo::new(move |_| page.get().clamp(1, page_count.get().max(1)));
    let go = move |n: usize| page.set(n.clamp(1, page_count.get_untracked().max(1)));
    let slots = move || page_window(current.get(), page_count.get()).into_iter().enumerate().collect::<Vec<_>>();

    move || {
        (page_count.get() > 1).then(|| view! {
            <nav
                class=move || match class.get() {
                    Some(extra) if !extra.is_empty() => format!("pagination {extra}"),
                    _ => "pagination".to_string(),
                }
                aria-label=move || label.get()
            >
                <ul class="pagination__list">
                    <li class="pagination__step">
                        <IconButton
                            label=labels.with_value(|l| l.previous.clone())
                            disabled=Signal::derive(move || current.get() <= 1)
                            on_click=move || go(current.get_untracked() - 1)
                        >
                            <Icon icon=CHEVRON_LEFT />
                        </IconButton>
                    </li>
                    <For each=slots key=|(i, item)| slot_key(*i, *item) let:slot>
                        {match slot.1 {
                            PageItem::Page(n) => view! {
                                <li class="pagination__slot">
                                    <button
                                        type="button"
                                        class="pagination__page"
                                        aria-label=labels.with_value(|l| l.page_label(n))
                                        aria-current=move || (current.get() == n).then_some("page")
                                        on:click=move |_| go(n)
                                    >
                                        {n}
                                    </button>
                                </li>
                            }.into_any(),
                            PageItem::Gap => view! {
                                <li class="pagination__slot pagination__gap" aria-hidden="true">"…"</li>
                            }.into_any(),
                        }}
                    </For>
                    <li class="pagination__position">
                        <span aria-hidden="true">{move || format!("{} / {}", current.get(), page_count.get())}</span>
                        <span class="sr-only">{move || labels.with_value(|l| l.position_label(current.get(), page_count.get()))}</span>
                    </li>
                    <li class="pagination__step">
                        <IconButton
                            label=labels.with_value(|l| l.next.clone())
                            disabled=Signal::derive(move || current.get() >= page_count.get())
                            on_click=move || go(current.get_untracked() + 1)
                        >
                            <Icon icon=CHEVRON_RIGHT />
                        </IconButton>
                    </li>
                </ul>
            </nav>
        })
    }
}

#[cfg(test)]
mod tests {
    use super::PageItem::{Gap, Page};
    use super::*;

    #[test]
    fn up_to_seven_pages_all_show() {
        assert_eq!(page_window(1, 0), vec![]);
        assert_eq!(page_window(1, 1), vec![Page(1)]);
        assert_eq!(page_window(4, 7), (1..=7).map(Page).collect::<Vec<_>>());
    }

    #[test]
    fn near_the_start_one_gap_before_the_last() {
        let start = vec![Page(1), Page(2), Page(3), Page(4), Page(5), Gap, Page(20)];
        assert_eq!(page_window(1, 20), start);
        assert_eq!(page_window(4, 20), start);
    }

    #[test]
    fn in_the_middle_a_gap_each_side() {
        assert_eq!(page_window(6, 20), vec![Page(1), Gap, Page(5), Page(6), Page(7), Gap, Page(20)]);
        assert_eq!(page_window(5, 20), vec![Page(1), Gap, Page(4), Page(5), Page(6), Gap, Page(20)]);
    }

    #[test]
    fn near_the_end_one_gap_after_the_first() {
        let end = vec![Page(1), Gap, Page(16), Page(17), Page(18), Page(19), Page(20)];
        assert_eq!(page_window(20, 20), end);
        assert_eq!(page_window(17, 20), end);
        assert_eq!(page_window(16, 20), vec![Page(1), Gap, Page(15), Page(16), Page(17), Gap, Page(20)]);
    }

    #[test]
    fn always_seven_slots_and_never_a_gap_for_one_page() {
        for count in 8..=30 {
            for current in 1..=count {
                let window = page_window(current, count);
                assert_eq!(window.len(), SLOTS, "{current} of {count}");
                assert!(window.contains(&Page(current)));
                for pair in window.windows(3) {
                    if let [Page(a), Gap, Page(b)] = pair {
                        assert!(b - a > 2, "a gap for one page: {current} of {count}");
                    }
                }
            }
        }
    }

    #[test]
    fn out_of_range_is_clamped() {
        assert_eq!(page_window(0, 20)[0], Page(1));
        assert_eq!(page_window(99, 20).last(), Some(&Page(20)));
    }

    #[test]
    fn labels_fill_their_templates() {
        let labels = PaginationLabels::default();
        assert_eq!(labels.page_label(6), "Page 6");
        assert_eq!(labels.position_label(6, 20), "Page 6 of 20");
    }
}
