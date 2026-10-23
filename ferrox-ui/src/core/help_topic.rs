use std::sync::Arc;

/// One part of a help topic. Structured rather than Markdown or HTML, so
/// nothing parses a string or trusts tags inside one.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum HelpBlock {
    Paragraph(String),
    List(Vec<String>),
    Table { columns: Vec<String>, rows: Vec<Vec<String>> },
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Topic {
    id: String,
    title: String,
    blocks: Vec<HelpBlock>,
    guide: Option<String>,
}

/// One thing a `?` can explain: a title, its blocks, and where the same
/// words sit in a full guide. Cheap to clone (the content is shared).
///
/// One topic per thing that can carry a `?`. The app owns the wording (and
/// its translation).
///
/// ```ignore
/// static AMOUNT: LazyLock<HelpTopic> = LazyLock::new(|| {
///     HelpTopic::new("split-amount", "Amounts on a split")
///         .paragraph("Each line moves money into or out of one account.")
///         .list(["Positive: money in", "Negative: money out"])
///         .guide("/help#split-amount")
/// });
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HelpTopic(Arc<Topic>);

impl HelpTopic {
    /// A topic with an id (stable: an assistant and a guide find it by this)
    /// and a title (what the `?` is named after).
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self(Arc::new(Topic { id: id.into(), title: title.into(), blocks: Vec::new(), guide: None }))
    }

    pub fn paragraph(self, text: impl Into<String>) -> Self {
        self.with_block(HelpBlock::Paragraph(text.into()))
    }

    pub fn list<I: Into<String>>(self, items: impl IntoIterator<Item = I>) -> Self {
        self.with_block(HelpBlock::List(items.into_iter().map(Into::into).collect()))
    }

    pub fn table<C: Into<String>, R: IntoIterator<Item = C>>(
        self,
        columns: impl IntoIterator<Item = C>,
        rows: impl IntoIterator<Item = R>,
    ) -> Self {
        let columns = columns.into_iter().map(Into::into).collect();
        let rows = rows.into_iter().map(|row| row.into_iter().map(Into::into).collect()).collect();
        self.with_block(HelpBlock::Table { columns, rows })
    }

    /// Where the same words sit among the rest ("Full guide").
    pub fn guide(mut self, href: impl Into<String>) -> Self {
        Arc::make_mut(&mut self.0).guide = Some(href.into());
        self
    }

    pub fn id(&self) -> &str {
        &self.0.id
    }

    pub fn title(&self) -> &str {
        &self.0.title
    }

    pub fn blocks(&self) -> &[HelpBlock] {
        &self.0.blocks
    }

    pub fn guide_href(&self) -> Option<&str> {
        self.0.guide.as_deref()
    }

    fn with_block(mut self, block: HelpBlock) -> Self {
        Arc::make_mut(&mut self.0).blocks.push(block);
        self
    }
}

/// A component's `help` prop: a topic, or none. Converts from a
/// [`HelpTopic`] or an `Option<HelpTopic>`, so a component can pass its own
/// on to another (`ConfirmDialog` to `Dialog`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MaybeHelp(pub Option<HelpTopic>);

impl From<HelpTopic> for MaybeHelp {
    fn from(topic: HelpTopic) -> Self {
        Self(Some(topic))
    }
}

impl From<Option<HelpTopic>> for MaybeHelp {
    fn from(topic: Option<HelpTopic>) -> Self {
        Self(topic)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_in_order() {
        let topic = HelpTopic::new("split-amount", "Amounts on a split")
            .paragraph("Each line moves money.")
            .list(["In", "Out"])
            .table(["Sign", "Means"], [["+", "in"], ["-", "out"]])
            .guide("/help#split-amount");
        assert_eq!(topic.id(), "split-amount");
        assert_eq!(topic.title(), "Amounts on a split");
        assert_eq!(topic.guide_href(), Some("/help#split-amount"));
        assert_eq!(topic.blocks(), &[
            HelpBlock::Paragraph("Each line moves money.".into()),
            HelpBlock::List(vec!["In".into(), "Out".into()]),
            HelpBlock::Table {
                columns: vec!["Sign".into(), "Means".into()],
                rows: vec![vec!["+".into(), "in".into()], vec!["-".into(), "out".into()]],
            },
        ]);
    }

    #[test]
    fn a_clone_shares_its_content() {
        let topic = HelpTopic::new("a", "A").paragraph("text");
        let clone = topic.clone();
        assert!(Arc::ptr_eq(&topic.0, &clone.0));
    }
}
