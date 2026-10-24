use leptos::prelude::*;
use leptos::{component, view, IntoView};

use super::heading::{heading, HeadingLevel};

/// The class list for a `.card`.
pub fn card_classes(clickable: bool, compact: bool, extra: &str) -> String {
    let mut classes = vec!["card"];
    if clickable { classes.push("card--clickable"); }
    if compact { classes.push("card--compact"); }
    if !extra.is_empty() { classes.push(extra); }
    classes.join(" ")
}

/// A surface for one thing (a record, an item, a choice): an `<article>`
/// with an optional heading, its own actions, and the body. The item
/// template of a list of cards.
///
/// - **A whole card as one target:** with `href` the title is a link, with
///   `on_click` a button, and its hit area is stretched over the card, so a
///   click anywhere on it follows the link or presses the button. A screen
///   reader meets one link or button, named by the title. A clickable card
///   needs a `title`.
/// - `actions` (edit, delete) sit in the top-right corner, above that
///   stretched area: separate targets that never follow the card.
/// - A plain `<a href>`: inside a `<Router>` the router handles it.
///
/// ```ignore
/// <Card title=employee.name href=format!("/employees/{}", employee.id)
///       actions=move || view! { <IconButton label="Remove" danger=true …>…</IconButton> }>
///     <p>{employee.role}</p>
/// </Card>
/// ```
#[component]
pub fn Card(
    #[prop(optional, into)] title: MaybeProp<String>,
    /// H3 by default: a card usually sits inside a panel or a section.
    #[prop(default = HeadingLevel::H3)] heading_level: HeadingLevel,
    /// The whole card follows this link.
    #[prop(optional, into)] href: MaybeProp<String>,
    /// The whole card is this button. Not with `href`.
    #[prop(optional, into)] on_click: Option<UnsyncCallback<()>>,
    /// The card's own buttons, in its top-right corner.
    #[prop(optional, into)] actions: Option<ViewFnOnce>,
    #[prop(optional)] compact: bool,
    /// Extra classes, appended to the generated ones. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let clickable = href.get_untracked().is_some() || on_click.is_some();
    if clickable && title.get_untracked().is_none() {
        leptos::logging::warn!("ferrox-ui Card: a clickable card needs a `title` to name its link or button");
    }
    let has_header = title.get_untracked().is_some() || actions.is_some();
    let title_text = move || title.get().unwrap_or_default();
    let target = match (href.get_untracked().is_some(), on_click) {
        (true, _) => view! { <a class="card__link" href=move || href.get()>{title_text}</a> }.into_any(),
        (false, Some(on_click)) => {
            view! { <button type="button" class="card__link" on:click=move |_| on_click.run(())>{title_text}</button> }.into_any()
        }
        (false, None) => title_text.into_any(),
    };
    let header = has_header.then(|| {
        let heading = title.get_untracked().is_some().then(|| heading(heading_level, "card__title", None, target));
        let actions = actions.map(|actions| view! { <div class="card__actions">{actions.run()}</div> });
        view! { <div class="card__header">{heading}{actions}</div> }
    });

    view! {
        <article class=move || card_classes(clickable, compact, &class.get().unwrap_or_default())>
            {header}
            <div class="card__body">{children()}</div>
        </article>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes() {
        assert_eq!(card_classes(false, false, ""), "card");
        assert_eq!(card_classes(true, true, ""), "card card--clickable card--compact");
        assert_eq!(card_classes(false, false, "employee"), "card employee");
    }
}
