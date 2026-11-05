//! What the parser produces: areas of topics, each topic a list of blocks.
//! Mirrors ferrox-ui's `HelpTopic` / `HelpBlock`, plus where each came from.

/// One area file: a heading, an optional intro, and its topics in order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Area {
    pub file: String,
    pub title: String,
    pub intro: Option<String>,
    pub topics: Vec<Topic>,
}

/// One `## Title {#id}` section.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Topic {
    pub id: String,
    pub title: String,
    pub blocks: Vec<Block>,
    /// Ids from the topic's `Related:` line, in the order written.
    pub related: Vec<Related>,
    /// 1-based line of the topic's heading, for error messages.
    pub line: usize,
}

/// A related id and the line it was written on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Related {
    pub id: String,
    pub line: usize,
}

/// The same three kinds as ferrox-ui's `HelpBlock`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Block {
    Paragraph(String),
    List(Vec<String>),
    Table { columns: Vec<String>, rows: Vec<Vec<String>> },
}
