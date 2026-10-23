use std::fmt::Write;

use super::help_topic::HelpTopic;

/// What "Ask about this" hands the app's assistant: which topic, on which
/// component, and everything the app declared around it. The app sends it
/// on (a chat, a model, a memory search); ferrox-ui never does.
///
/// It carries only what the developer declared. A control's value is in it
/// only when that component opted in.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct HelpRequest {
    pub topic_id: String,
    pub topic_title: String,
    /// What carries the `?`: "field", "checkbox", "dialog", … ("help" for a
    /// `HelpButton` on its own).
    pub component: &'static str,
    /// The component's label or title.
    pub label: String,
    /// Every enclosing `HelpScope`'s pairs, outermost first.
    pub context: Vec<(String, String)>,
    /// The control's current value, only when the component opted in.
    pub value: Option<String>,
}

impl HelpRequest {
    /// A request about `topic`, asked from a `component` labelled `label`.
    pub fn new(topic: &HelpTopic, component: &'static str, label: impl Into<String>) -> Self {
        Self {
            topic_id: topic.id().to_string(),
            topic_title: topic.title().to_string(),
            component,
            label: label.into(),
            context: Vec::new(),
            value: None,
        }
    }

    pub fn with_context(mut self, context: Vec<(String, String)>) -> Self {
        self.context = context;
        self
    }

    pub fn with_value(mut self, value: Option<String>) -> Self {
        self.value = value;
        self
    }

    /// Plain text for a prompt, one `key: value` per line: the topic, the
    /// component and its label, the context outermost first, then the value
    /// if there is one.
    ///
    /// ```text
    /// topic: split-amount (Amounts on a split)
    /// component: field
    /// label: Amount
    /// screen: reconcile
    /// account: checking
    /// ```
    pub fn to_prompt_context(&self) -> String {
        let mut text = format!(
            "topic: {} ({})\ncomponent: {}\nlabel: {}\n",
            self.topic_id, self.topic_title, self.component, self.label,
        );
        for (key, value) in &self.context {
            let _ = writeln!(text, "{key}: {value}");
        }
        if let Some(value) = &self.value {
            let _ = writeln!(text, "value: {value}");
        }
        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn topic() -> HelpTopic {
        HelpTopic::new("split-amount", "Amounts on a split")
    }

    #[test]
    fn prompt_context_lists_everything_in_order() {
        let request = HelpRequest::new(&topic(), "field", "Amount")
            .with_context(vec![("screen".into(), "reconcile".into()), ("account".into(), "checking".into())])
            .with_value(Some("12.50".into()));
        assert_eq!(
            request.to_prompt_context(),
            "topic: split-amount (Amounts on a split)\ncomponent: field\nlabel: Amount\nscreen: reconcile\naccount: checking\nvalue: 12.50\n",
        );
    }

    #[test]
    fn no_value_line_without_a_value() {
        let request = HelpRequest::new(&topic(), "help", "Amounts on a split");
        assert!(!request.to_prompt_context().contains("value:"));
    }
}
