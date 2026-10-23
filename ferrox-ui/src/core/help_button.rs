use ferrox_icons::{Icon, HELP};
use leptos::html;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use wasm_bindgen::JsCast;
use web_sys::{Element, Event, HtmlElement};

use super::anchor::{follow, place};
use super::help_topic::{HelpBlock, HelpTopic};
use crate::ids::unique_id;

/// The class list for a help button: an `.icon-btn` sized to sit by text.
pub fn help_button_classes(extra: &str) -> String {
    if extra.is_empty() { "icon-btn help-btn".to_string() } else { format!("icon-btn help-btn {extra}") }
}

/// A `?` that opens a help topic in a popover beside it.
///
/// - **It names what it explains:** "Help: {title}", as its label and
///   tooltip, so a screen reader knows which `?` it landed on.
/// - The popover is the browser's own (`popover="auto"`, in the top layer):
///   Esc or a click elsewhere closes it, and Esc gives focus back to the `?`.
///   It sits below the `?` (above when there is no room), inside the
///   viewport, and follows it while the page scrolls.
/// - It shows the topic's title and blocks, and "Full guide" when the topic
///   has one.
/// - `skip_tab` leaves the Tab order: for a `?` in a row of fields, where it
///   would cost a stop on every entry. It stays clickable and reachable by a
///   screen reader.
///
/// Put it where block content is allowed (not inside a `<label>` or a
/// `<button>`); components that take a `help` prop place it for you.
///
/// ```ignore
/// <h2>"Statement balance" <HelpButton topic=STATEMENT_BALANCE.clone() /></h2>
/// ```
#[component]
pub fn HelpButton(
    topic: HelpTopic,
    #[prop(optional)] skip_tab: bool,
    /// The word before the title in the button's label.
    #[prop(into, default = "Help".into())] help_label: Signal<String>,
    #[prop(into, default = "Full guide".into())] guide_label: Signal<String>,
    /// Extra classes on the button. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let popover_id = unique_id("ferrox-help");
    let title_id = format!("{popover_id}-title");
    let (controls, target, labelled_by) = (popover_id.clone(), popover_id.clone(), title_id.clone());
    let open = RwSignal::new(false);
    let button_ref = NodeRef::<html::Button>::new();
    let popover_ref = NodeRef::<html::Div>::new();

    let title = topic.title().to_string();
    let label = Signal::derive({
        let title = title.clone();
        move || format!("{}: {title}", help_label.get())
    });

    let on_toggle = move |_: Event| {
        let (Some(button), Some(popover)) = (button_ref.get_untracked(), popover_ref.get_untracked()) else { return };
        let is_open = popover.matches(":popover-open").unwrap_or(false);
        open.set(is_open);
        if is_open {
            let anchor: Element = button.unchecked_into();
            let floating: HtmlElement = popover.clone().unchecked_into();
            place(&anchor, &floating);
            let _ = popover.focus();
            follow(anchor, floating);
        }
    };

    let blocks = topic.blocks().iter().map(render_block).collect_view();
    let guide = topic.guide_href().map(|href| view! {
        <div class="help-popover__actions">
            <a class="help-popover__link" href=href.to_string()>{move || guide_label.get()}</a>
        </div>
    });

    view! {
        <button
            type="button"
            node_ref=button_ref
            class=move || help_button_classes(&class.get().unwrap_or_default())
            aria-label=label
            title=label
            aria-expanded=move || if open.get() { "true" } else { "false" }
            aria-controls=controls
            popovertarget=target
            tabindex=skip_tab.then_some("-1")
        >
            <Icon icon=HELP />
        </button>
        <div
            popover="auto"
            id=popover_id
            node_ref=popover_ref
            class="help-popover"
            role="dialog"
            aria-labelledby=labelled_by
            tabindex="-1"
            on:toggle=on_toggle
        >
            <h3 class="help-popover__title" id=title_id>{title}</h3>
            <div class="help-popover__body">{blocks}</div>
            {guide}
        </div>
    }
}

fn render_block(block: &HelpBlock) -> AnyView {
    match block {
        HelpBlock::Paragraph(text) => view! { <p class="help-popover__paragraph">{text.clone()}</p> }.into_any(),
        HelpBlock::List(items) => view! {
            <ul class="help-popover__list">
                {items.iter().map(|item| view! { <li>{item.clone()}</li> }).collect_view()}
            </ul>
        }.into_any(),
        HelpBlock::Table { columns, rows } => view! {
            <table class="help-popover__table">
                <thead><tr>
                    {columns.iter().map(|c| view! { <th class="help-popover__heading">{c.clone()}</th> }).collect_view()}
                </tr></thead>
                <tbody>
                    {rows.iter().map(|row| view! {
                        <tr>{row.iter().map(|cell| view! { <td class="help-popover__cell">{cell.clone()}</td> }).collect_view()}</tr>
                    }).collect_view()}
                </tbody>
            </table>
        }.into_any(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes() {
        assert_eq!(help_button_classes(""), "icon-btn help-btn");
        assert_eq!(help_button_classes("x"), "icon-btn help-btn x");
    }
}
