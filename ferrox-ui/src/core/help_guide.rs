use ferrox_icons::{Icon, MESSAGE_CIRCLE};
use leptos::ev::hashchange;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, MouseEvent};

use super::help_blocks::HelpBlocks;
use super::help_request::HelpRequest;
use super::help_scope::{assistant, scope_context};
use super::help_topic::HelpTopic;
use super::help_topics::{use_help_topics, HelpArea, HelpTopics};
use crate::ids::unique_id;

/// "Ask about this" in the guide: the assistant's callback and the context
/// around the guide.
type Ask = Option<(UnsyncCallback<HelpRequest>, Signal<Vec<(String, String)>>)>;

/// The class list for a guide.
pub fn help_guide_classes(extra: &str) -> String {
    if extra.is_empty() { "help-guide".to_string() } else { format!("help-guide {extra}") }
}

/// Every help topic on one page, in reading order: a contents list, then
/// each area with its intro and topics. The body of an app's `/help` page;
/// the app owns the route and the page around it.
///
/// - Each topic is a `<section id="{topic id}">`, so `/help#recipe-search`
///   (what `ferrox-help-build`'s `guide_prefix("/help#")` links to) opens at
///   it. On arriving, and when the hash changes, the topic is scrolled to
///   and its title focused. Contents and related links do the same in place
///   and update the URL's hash.
/// - Related topics are links to their sections.
/// - Inside a [`HelpAssistant`](crate::HelpAssistant), each topic offers "Ask
///   about this": a [`HelpRequest`] with component `"guide"`, the topic's
///   title as label, and the enclosing scopes' context.
/// - `topics` defaults to the registry from
///   [`provide_help_topics`](crate::provide_help_topics); with neither, the
///   guide is empty.
///
/// A sticky app bar can cover a topic scrolled to the top: set
/// `--help-guide-scroll-margin` to its height (4.5rem by default).
///
/// ```ignore
/// <Page title="Help">
///     <HelpGuide />
/// </Page>
/// ```
#[component]
pub fn HelpGuide(
    /// The topics; the provided registry by default.
    #[prop(optional, into)] topics: Option<HelpTopics>,
    #[prop(into, default = "Contents".into())] contents_label: Signal<String>,
    #[prop(into, default = "Related".into())] related_label: Signal<String>,
    #[prop(into, default = "Ask about this".into())] ask_label: Signal<String>,
    /// Extra classes on the guide.
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let topics = topics.or_else(use_help_topics);
    let listener = window_event_listener(hashchange, |_| go_to_hash());
    on_cleanup(move || listener.remove());
    request_animation_frame(go_to_hash);

    let Some(topics) = topics else {
        return view! { <div class=move || help_guide_classes(&class.get().unwrap_or_default())></div> }.into_any();
    };
    let ask = assistant().map(|on_ask| (on_ask, scope_context()));
    let contents = contents(&topics, contents_label);
    let areas = topics
        .areas()
        .iter()
        .map(|area| area_view(area, &topics, related_label, ask_label, ask))
        .collect_view();
    view! {
        <div class=move || help_guide_classes(&class.get().unwrap_or_default())>
            {contents}
            {areas}
        </div>
    }
    .into_any()
}

/// The contents: each area's title over links to its topics.
fn contents(topics: &HelpTopics, label: Signal<String>) -> impl IntoView + use<> {
    let areas = topics.areas().iter().map(|area| {
        let links = area.topics.iter().map(|topic| view! { <li>{topic_link(topic)}</li> }).collect_view();
        view! {
            <li class="help-guide__contents-area">
                <span class="help-guide__contents-title">{area.title.clone()}</span>
                <ul class="help-guide__contents-topics">{links}</ul>
            </li>
        }
    }).collect_view();
    view! {
        <nav class="help-guide__contents" aria-label=move || label.get()>
            <ul class="help-guide__contents-list">{areas}</ul>
        </nav>
    }
}

/// One area: its heading, intro and topics.
fn area_view(
    area: &HelpArea,
    topics: &HelpTopics,
    related_label: Signal<String>,
    ask_label: Signal<String>,
    ask: Ask,
) -> impl IntoView + use<> {
    let heading_id = unique_id("ferrox-help-area");
    let labelled_by = heading_id.clone();
    let intro = area.intro.clone().map(|text| view! { <p class="help-guide__intro">{text}</p> });
    let sections = area
        .topics
        .iter()
        .map(|topic| topic_view(topic, topics.related(topic), related_label, ask_label, ask))
        .collect_view();
    view! {
        <section class="help-guide__area" aria-labelledby=labelled_by>
            <h2 class="help-guide__area-title" id=heading_id>{area.title.clone()}</h2>
            {intro}
            {sections}
        </section>
    }
}

/// One topic: its title with the Ask icon beside it, blocks, related links.
fn topic_view(
    topic: &HelpTopic,
    related: Vec<HelpTopic>,
    related_label: Signal<String>,
    ask_label: Signal<String>,
    ask: Ask,
) -> impl IntoView + use<> {
    let heading_id = unique_id("ferrox-help-topic");
    let labelled_by = heading_id.clone();
    let related = (!related.is_empty()).then(|| {
        let links = related.iter().map(|t| view! { <li>{topic_link(t)}</li> }).collect_view();
        view! {
            <div class="help-guide__related">
                <span class="help-guide__related-title">{move || related_label.get()}</span>
                <ul class="help-guide__related-list">{links}</ul>
            </div>
        }
    });
    let ask = ask.map(|(on_ask, context)| {
        let topic = topic.clone();
        let title = topic.title().to_string();
        let send = move |_| {
            let request = HelpRequest::new(&topic, "guide", topic.title()).with_context(context.get_untracked());
            on_ask.run(request);
        };
        // The popover's chat icon: named by `ask_label`, its tooltip naming the topic.
        view! {
            <button
                type="button"
                class="icon-btn help-guide__ask"
                aria-label=move || ask_label.get()
                title=move || format!("{}: {title}", ask_label.get())
                on:click=send
            >
                <Icon icon=MESSAGE_CIRCLE />
            </button>
        }
    });
    view! {
        <section class="help-guide__topic" id=topic.id().to_string() aria-labelledby=labelled_by>
            <div class="help-guide__topic-header">
                <h3 class="help-guide__topic-title" id=heading_id tabindex="-1">{topic.title().to_string()}</h3>
                {ask}
            </div>
            <HelpBlocks blocks=topic.blocks().to_vec() />
            {related}
        </section>
    }
}

/// A link to a topic's section that moves there in place: the URL's hash
/// changes, the router doesn't navigate.
fn topic_link(topic: &HelpTopic) -> impl IntoView + use<> {
    let id = topic.id().to_string();
    let href = format!("#{id}");
    let follow = move |event: MouseEvent| {
        if event.button() != 0 || event.ctrl_key() || event.meta_key() || event.shift_key() || event.alt_key() {
            return; // a new tab or window: the browser's
        }
        event.prevent_default();
        let hash = format!("#{id}");
        if let Ok(history) = window().history() {
            let _ = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&hash));
        }
        go_to(&id);
    };
    view! { <a class="help-guide__link" href=href on:click=follow>{topic.title().to_string()}</a> }
}

/// Goes to the topic the URL's hash names, if it's on this page.
fn go_to_hash() {
    let hash = window().location().hash().unwrap_or_default();
    let id = hash.trim_start_matches('#');
    if !id.is_empty() {
        go_to(id);
    }
}

/// Scrolls a topic's section into view and focuses its title.
fn go_to(id: &str) {
    let Some(section) = document().get_element_by_id(id) else { return };
    if !section.class_list().contains("help-guide__topic") {
        return;
    }
    section.scroll_into_view();
    if let Ok(Some(title)) = section.query_selector(".help-guide__topic-title")
        && let Ok(title) = title.dyn_into::<HtmlElement>()
    {
        let _ = title.focus();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes() {
        assert_eq!(help_guide_classes(""), "help-guide");
        assert_eq!(help_guide_classes("x"), "help-guide x");
    }
}
