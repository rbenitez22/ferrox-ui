use leptos::prelude::*;
use leptos::{component, view, IntoView};

/// The space between children, on the theme's spacing tokens.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Gap {
    None,
    /// `--space-xs` (4px).
    Xs,
    /// `--space-sm` (8px).
    Sm,
    /// `--space-md` (16px).
    #[default]
    Md,
    /// `--space-lg` (24px).
    Lg,
    /// `--space-xl` (32px).
    Xl,
}

impl Gap {
    fn suffix(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Xs => "xs",
            Self::Sm => "sm",
            Self::Md => "md",
            Self::Lg => "lg",
            Self::Xl => "xl",
        }
    }
}

/// Where children sit across the flow: a `Stack`'s width, a `Cluster`'s
/// height.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Align {
    /// As wide (or tall) as the container.
    Stretch,
    Start,
    Center,
    End,
}

impl Align {
    fn suffix(self) -> &'static str {
        match self {
            Self::Stretch => "stretch",
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

/// Where a `Cluster`'s children sit along its row.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Justify {
    #[default]
    Start,
    Center,
    End,
    /// The first at the start, the last at the end, the space shared between.
    Between,
}

impl Justify {
    fn suffix(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
            Self::Between => "between",
        }
    }
}

/// The class list for a `.stack`.
pub fn stack_classes(gap: Gap, align: Align, extra: &str) -> String {
    let mut classes = vec!["stack".to_string(), format!("stack--gap-{}", gap.suffix()), format!("stack--align-{}", align.suffix())];
    if !extra.is_empty() { classes.push(extra.to_string()); }
    classes.join(" ")
}

/// The class list for a `.cluster`.
pub fn cluster_classes(gap: Gap, align: Align, justify: Justify, extra: &str) -> String {
    let mut classes = vec![
        "cluster".to_string(),
        format!("cluster--gap-{}", gap.suffix()),
        format!("cluster--align-{}", align.suffix()),
        format!("cluster--justify-{}", justify.suffix()),
    ];
    if !extra.is_empty() { classes.push(extra.to_string()); }
    classes.join(" ")
}

/// Children one under another, a token gap between: a form's fields, a
/// column of panels, a card's lines. The space between things on a page
/// comes from here, not from the app's CSS.
///
/// ```ignore
/// <Stack gap=Gap::Lg>
///     <Panel title="Details">…</Panel>
///     <Panel title="History">…</Panel>
/// </Stack>
/// ```
#[component]
pub fn Stack(
    #[prop(optional)] gap: Gap,
    /// Across the column: `Stretch` (full width, the default), `Start`,
    /// `Center`, `End`.
    #[prop(default = Align::Stretch)] align: Align,
    /// Extra classes, appended to the generated ones. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || stack_classes(gap, align, &class.get().unwrap_or_default())>{children()}</div>
    }
}

/// Children side by side, wrapping onto the next line when they don't fit,
/// a token gap between: a row of buttons, tags, a label and its value.
///
/// ```ignore
/// <Cluster justify=Justify::End>
///     <Button family=ButtonFamily::Cancel on_click=move || cancel()>"Cancel"</Button>
///     <Button on_click=move || save()>"Save"</Button>
/// </Cluster>
/// ```
#[component]
pub fn Cluster(
    #[prop(default = Gap::Sm)] gap: Gap,
    /// Across the row: `Center` by default.
    #[prop(default = Align::Center)] align: Align,
    /// Along the row: `Start` by default.
    #[prop(optional)] justify: Justify,
    /// Extra classes, appended to the generated ones. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=move || cluster_classes(gap, align, justify, &class.get().unwrap_or_default())>{children()}</div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes() {
        assert_eq!(stack_classes(Gap::default(), Align::Stretch, ""), "stack stack--gap-md stack--align-stretch");
        assert_eq!(
            cluster_classes(Gap::Sm, Align::Center, Justify::Between, "x"),
            "cluster cluster--gap-sm cluster--align-center cluster--justify-between x"
        );
    }
}
