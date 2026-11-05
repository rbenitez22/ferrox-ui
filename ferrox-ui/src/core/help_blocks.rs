use leptos::prelude::*;
use leptos::{component, view, IntoView};

use super::help_topic::HelpBlock;

/// A topic's blocks: paragraphs, lists and tables, as plain text. Shared
/// by the `?` popover and the full guide so both read the same.
#[component]
pub fn HelpBlocks(blocks: Vec<HelpBlock>) -> impl IntoView {
    view! { <div class="help-blocks">{blocks.iter().map(render_block).collect_view()}</div> }
}

fn render_block(block: &HelpBlock) -> AnyView {
    match block {
        HelpBlock::Paragraph(text) => view! { <p class="help-blocks__paragraph">{text.clone()}</p> }.into_any(),
        HelpBlock::List(items) => view! {
            <ul class="help-blocks__list">
                {items.iter().map(|item| view! { <li>{item.clone()}</li> }).collect_view()}
            </ul>
        }.into_any(),
        HelpBlock::Table { columns, rows } => view! {
            <table class="help-blocks__table">
                <thead><tr>
                    {columns.iter().map(|c| view! { <th class="help-blocks__heading" scope="col">{c.clone()}</th> }).collect_view()}
                </tr></thead>
                <tbody>
                    {rows.iter().map(|row| view! {
                        <tr>{row.iter().map(|cell| view! { <td class="help-blocks__cell">{cell.clone()}</td> }).collect_view()}</tr>
                    }).collect_view()}
                </tbody>
            </table>
        }.into_any(),
    }
}
