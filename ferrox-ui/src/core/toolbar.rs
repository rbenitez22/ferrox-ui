use leptos::prelude::*;
use leptos::{component, view, IntoView};

/// A row of buttons (`.toolbar`). Buttons sit at the right; an optional
/// `title` takes the free space on the left.
///
/// `compact` tightens the row for `IconButton`s and `ButtonSize::Sm` buttons.
/// Put buttons that belong together in a `ToolbarGroup`; groups are split by
/// a hairline.
///
/// ```ignore
/// <Toolbar title="Accounts">
///     <Button family=ButtonFamily::Primary on_click=move |_| add()>"New account"</Button>
/// </Toolbar>
///
/// <Toolbar compact=true>
///     <ToolbarGroup>
///         <IconButton title="Search" on_click=…>…</IconButton>
///         <IconButton title="Camera" pressed=camera_on on_click=…>…</IconButton>
///     </ToolbarGroup>
///     <ToolbarGroup>
///         <IconButton title="Help" on_click=…>…</IconButton>
///     </ToolbarGroup>
/// </Toolbar>
/// ```
#[component]
pub fn Toolbar(
    #[prop(optional, into)] title: MaybeProp<String>,
    #[prop(optional)] compact: bool,
    /// Extra classes, appended to the generated ones. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || toolbar_classes(compact, &class.get().unwrap_or_default())>
            {move || title.get().map(|t| view! { <span class="toolbar__title">{t}</span> })}
            {children()}
        </div>
    }
}

/// Buttons that belong together inside a `Toolbar`.
#[component]
pub fn ToolbarGroup(
    /// Extra classes, appended to `toolbar__group`. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let classes = move || match class.get() {
        Some(extra) if !extra.is_empty() => format!("toolbar__group {extra}"),
        _ => "toolbar__group".to_string(),
    };
    view! { <div class=classes>{children()}</div> }
}

/// The class list for a `.toolbar`.
pub fn toolbar_classes(compact: bool, extra: &str) -> String {
    let mut classes = vec!["toolbar"];
    if compact { classes.push("toolbar--compact"); }
    if !extra.is_empty() { classes.push(extra); }
    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_adds_modifier() {
        assert_eq!(toolbar_classes(false, ""), "toolbar");
        assert_eq!(toolbar_classes(true, "extra"), "toolbar toolbar--compact extra");
    }
}
