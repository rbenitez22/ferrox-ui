use std::collections::HashMap;
use std::sync::Arc;

use leptos::prelude::{provide_context, use_context};

use super::help_topic::HelpTopic;

/// A group of topics read together, such as one file of an app's help:
/// a title, an optional intro, and topics in reading order.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct HelpArea {
    pub title: String,
    pub intro: Option<String>,
    pub topics: Vec<HelpTopic>,
}

impl HelpArea {
    pub fn new(title: impl Into<String>, topics: impl IntoIterator<Item = HelpTopic>) -> Self {
        Self { title: title.into(), intro: None, topics: topics.into_iter().collect() }
    }

    /// A paragraph introducing the area in a full guide.
    pub fn intro(mut self, text: impl Into<String>) -> Self {
        self.intro = Some(text.into());
        self
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Registry {
    areas: Vec<HelpArea>,
    /// Topic id → (area index, topic index); the first one wins on a repeat.
    index: HashMap<String, (usize, usize)>,
}

/// All of an app's help topics, in reading order, found by id. It resolves
/// a topic's `related` ids and feeds a full guide. Cheap to clone.
///
/// `ferrox-help-build` generates one as `TOPICS`; topics written in Rust can
/// be gathered by hand:
///
/// ```ignore
/// let topics = HelpTopics::new([HelpArea::new("Basics", [AMOUNT.clone(), BALANCE.clone()])]);
/// provide_help_topics(topics);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HelpTopics(Arc<Registry>);

impl HelpTopics {
    /// A registry of `areas`. In debug builds, panics on a repeated id or a
    /// related id that names no topic (see [`HelpTopics::problems`]).
    pub fn new(areas: impl IntoIterator<Item = HelpArea>) -> Self {
        let areas: Vec<HelpArea> = areas.into_iter().collect();
        let mut index = HashMap::new();
        for (a, area) in areas.iter().enumerate() {
            for (t, topic) in area.topics.iter().enumerate() {
                index.entry(topic.id().to_string()).or_insert((a, t));
            }
        }
        let topics = Self(Arc::new(Registry { areas, index }));
        debug_assert!(topics.problems().is_empty(), "help topics: {}", topics.problems().join("; "));
        topics
    }

    /// Repeated ids and related ids that name no topic, as messages.
    pub fn problems(&self) -> Vec<String> {
        let mut problems = Vec::new();
        let mut seen = HashMap::new();
        for topic in self.iter() {
            if seen.insert(topic.id(), ()).is_some() {
                problems.push(format!("`{}` is used by more than one topic", topic.id()));
            }
            for id in topic.related_ids() {
                if !self.0.index.contains_key(id) {
                    problems.push(format!("`{}` lists related `{id}`, which is no topic", topic.id()));
                }
            }
        }
        problems
    }

    /// The topic with this id.
    pub fn get(&self, id: &str) -> Option<HelpTopic> {
        let (a, t) = *self.0.index.get(id)?;
        Some(self.0.areas[a].topics[t].clone())
    }

    pub fn areas(&self) -> &[HelpArea] {
        &self.0.areas
    }

    /// Every topic, in reading order.
    pub fn iter(&self) -> impl Iterator<Item = &HelpTopic> {
        self.0.areas.iter().flat_map(|area| area.topics.iter())
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn is_empty(&self) -> bool {
        self.iter().next().is_none()
    }

    /// The topics `topic` lists as related, in its order, skipping unknown ids.
    pub fn related(&self, topic: &HelpTopic) -> Vec<HelpTopic> {
        topic.related_ids().iter().filter_map(|id| self.get(id)).collect()
    }
}

/// Makes `topics` the registry for everything below: related links in `?`
/// popovers, and a `HelpGuide` with no `topics` of its own.
pub fn provide_help_topics(topics: HelpTopics) {
    provide_context(topics);
}

/// The registry provided above, if any.
pub fn use_help_topics() -> Option<HelpTopics> {
    use_context::<HelpTopics>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn topic(id: &str, related: &[&str]) -> HelpTopic {
        HelpTopic::new(id, id.to_uppercase()).paragraph("text").related(related.iter().copied())
    }

    fn sample() -> HelpTopics {
        HelpTopics::new([
            HelpArea::new("First", [topic("a", &["c", "b"]), topic("b", &[])]).intro("Start here."),
            HelpArea::new("Second", [topic("c", &["a"])]),
        ])
    }

    #[test]
    fn finds_topics_in_reading_order() {
        let topics = sample();
        let order: Vec<_> = topics.iter().map(HelpTopic::id).collect();
        assert_eq!(order, ["a", "b", "c"]);
        assert_eq!(topics.len(), 3);
        assert_eq!(topics.get("c").map(|t| t.title().to_string()), Some("C".into()));
        assert_eq!(topics.get("zzz"), None);
        assert_eq!(topics.areas()[0].intro.as_deref(), Some("Start here."));
    }

    #[test]
    fn resolves_related_in_the_topics_order() {
        let topics = sample();
        let a = topics.get("a").unwrap();
        let related: Vec<_> = topics.related(&a).iter().map(|t| t.id().to_string()).collect();
        assert_eq!(related, ["c", "b"]);
    }

    #[test]
    fn reports_problems() {
        // Built without the debug check, to look at what it would report.
        let areas = vec![HelpArea::new("A", [topic("a", &["missing"]), topic("a", &[])])];
        let mut index = HashMap::new();
        index.insert("a".to_string(), (0, 0));
        let topics = HelpTopics(Arc::new(Registry { areas, index }));
        assert_eq!(topics.problems(), [
            "`a` lists related `missing`, which is no topic",
            "`a` is used by more than one topic",
        ]);
    }

    #[test]
    #[should_panic(expected = "help topics")]
    #[cfg(debug_assertions)]
    fn new_checks_in_debug_builds() {
        HelpTopics::new([HelpArea::new("A", [topic("a", &["nope"])])]);
    }

    #[test]
    fn empty() {
        assert!(HelpTopics::new([]).is_empty());
    }
}
