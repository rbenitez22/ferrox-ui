/// The narrowest a grid column may be: the grid fits as many as it can.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum CardSize {
    /// 10rem: small tiles.
    Sm,
    /// 14rem.
    #[default]
    Md,
    /// 20rem: cards with a picture and a few lines.
    Lg,
}

/// How a `DataView` lays out its items.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DataLayout {
    /// As many columns as fit, each at least the card size wide.
    Grid(CardSize),
    /// One full-width row per item, a hairline between.
    List,
    /// One horizontal strip that scrolls sideways, snapping to items.
    Row,
    /// Side by side, wrapping, each item sized to its content (tags, chips).
    Wrap,
}

impl Default for DataLayout {
    fn default() -> Self {
        Self::Grid(CardSize::Md)
    }
}

impl DataLayout {
    /// The modifier class on `.data-view`.
    pub fn class(self) -> &'static str {
        match self {
            Self::Grid(CardSize::Sm) => "data-view--grid-sm",
            Self::Grid(CardSize::Md) => "data-view--grid-md",
            Self::Grid(CardSize::Lg) => "data-view--grid-lg",
            Self::List => "data-view--list",
            Self::Row => "data-view--row",
            Self::Wrap => "data-view--wrap",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_is_a_grid_of_medium_cards() {
        assert_eq!(DataLayout::default(), DataLayout::Grid(CardSize::Md));
        assert_eq!(DataLayout::default().class(), "data-view--grid-md");
    }

    #[test]
    fn each_layout_has_its_class() {
        assert_eq!(DataLayout::Grid(CardSize::Lg).class(), "data-view--grid-lg");
        assert_eq!(DataLayout::List.class(), "data-view--list");
        assert_eq!(DataLayout::Row.class(), "data-view--row");
        assert_eq!(DataLayout::Wrap.class(), "data-view--wrap");
    }
}
