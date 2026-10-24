use leptos::prelude::*;
use leptos::view;

/// Which heading element a titled component renders, so it sits at the
/// right place in the page's outline.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum HeadingLevel {
    /// The page's own title.
    H1,
    /// A section of the page.
    #[default]
    H2,
    H3,
    H4,
}

impl HeadingLevel {
    /// The element's tag name.
    pub fn tag(self) -> &'static str {
        match self {
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
        }
    }
}

/// A heading of `level` with this class (and id, if any), holding `children`.
pub(crate) fn heading(level: HeadingLevel, class: &'static str, id: Option<String>, children: AnyView) -> AnyView {
    match level {
        HeadingLevel::H1 => view! { <h1 class=class id=id>{children}</h1> }.into_any(),
        HeadingLevel::H2 => view! { <h2 class=class id=id>{children}</h2> }.into_any(),
        HeadingLevel::H3 => view! { <h3 class=class id=id>{children}</h3> }.into_any(),
        HeadingLevel::H4 => view! { <h4 class=class id=id>{children}</h4> }.into_any(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tags() {
        assert_eq!(HeadingLevel::default(), HeadingLevel::H2);
        assert_eq!(HeadingLevel::H1.tag(), "h1");
        assert_eq!(HeadingLevel::H4.tag(), "h4");
    }
}
