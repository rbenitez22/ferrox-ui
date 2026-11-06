use ferrox_icons::{Icon, HELP, MESSAGE_CIRCLE};
use leptos::ev::{on, Custom};
use leptos::html;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use wasm_bindgen::JsCast;
use web_sys::{Element, Event, HtmlElement};

use super::anchor::{follow, place, Align};
use super::help_request::HelpRequest;
use super::help_keys::help_keys_shortcuts;
use super::help_scope::{assistant, scope_context};
use super::help_signals::{help_signals, HelpSignalKind};
use super::help_blocks::HelpBlocks;
use super::help_topic::{HelpTopic, MaybeHelp};
use super::help_topics::{use_help_topics, HelpTopics};
use crate::ids::unique_id;

/// The class list for a help button: an `.icon-btn` sized to sit by text.
pub fn help_button_classes(extra: &str) -> String {
    if extra.is_empty() { "icon-btn help-btn".to_string() } else { format!("icon-btn help-btn {extra}") }
}

/// A `?` that opens a help topic in a popover beside it.
///
/// - **It names what it explains:** "Help: {title}", as its label and
///   tooltip, so a screen reader knows which `?` it landed on.
/// - The popover is the browser's own (`popover="auto"`, in the top layer):
///   Esc or a click elsewhere closes it, and Esc gives focus back to the `?`.
///   It sits below the `?` (above when there is no room), inside the
///   viewport, and follows it while the page scrolls.
/// - It shows the topic's title and blocks, and "Full guide" when the topic
///   has one.
/// - With a registry above it ([`provide_help_topics`](crate::provide_help_topics)),
///   it lists the topic's related topics; choosing one shows it in the same
///   popover, and "Full guide" and "Ask about this" follow what's shown.
///   Each opening starts again at the button's own topic.
/// - Inside a [`HelpAssistant`](crate::HelpAssistant) it also offers "Ask
///   about this", which closes the popover and hands the assistant a
///   [`HelpRequest`]: the topic, `kind` and `about` (what carries the `?`),
///   the enclosing [`HelpScope`](crate::HelpScope)s' context, and `value`
///   only if one is given.
/// - Inside a [`HelpSignals`](crate::HelpSignals) it reports
///   `HelpOpened`, and `HelpDismissed` when closed within a second (not by
///   asking); while the app suggests its topic
///   ([`HelpReporter::suggest`](crate::HelpReporter::suggest)) it wears a
///   ring and says so in its label, until it's opened.
/// - `skip_tab` leaves the Tab order: for a `?` in a row of fields, where it
///   would cost a stop on every entry. It stays clickable and reachable by a
///   screen reader.
///
/// Put it where block content is allowed (not inside a `<label>` or a
/// `<button>`); components that take a `help` prop place it for you.
///
/// ```ignore
/// <h2>"Statement balance" <HelpButton topic=STATEMENT_BALANCE.clone() /></h2>
/// ```
#[component]
pub fn HelpButton(
    topic: HelpTopic,
    #[prop(optional)] skip_tab: bool,
    /// The word before the title in the button's label.
    #[prop(into, default = "Help".into())] help_label: Signal<String>,
    #[prop(into, default = "Full guide".into())] guide_label: Signal<String>,
    #[prop(into, default = "Ask about this".into())] ask_label: Signal<String>,
    /// The heading over related topics.
    #[prop(into, default = "Related".into())] related_label: Signal<String>,
    /// Added to the label while the app suggests this topic.
    #[prop(into, default = ", suggested".into())] suggested_label: Signal<String>,
    /// What carries the `?`, for the request ("field", "dialog", …).
    #[prop(default = "help")] kind: &'static str,
    /// The label or title of what the `?` is about, for the request (the
    /// topic's title by default).
    #[prop(optional, into)] about: MaybeProp<String>,
    /// A value to send with the request. Only pass one the user would expect
    /// an assistant to see.
    #[prop(optional, into)] value: MaybeProp<String>,
    /// The button's id (one is made if not given). F1 (`HelpKeys`) finds a
    /// `?` from the focused element's closest `[data-help]`, which names it
    /// by this id; components that place a `?` set both.
    #[prop(optional, into)] id: Option<String>,
    /// Extra classes on the button. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let popover_id = unique_id("ferrox-help");
    let button_id = id.unwrap_or_else(|| format!("{popover_id}-button"));
    let title_id = format!("{popover_id}-title");
    let (controls, target, labelled_by) = (popover_id.clone(), popover_id.clone(), title_id.clone());
    let open = RwSignal::new(false);
    // Where this `?` sits decides what a request carries.
    let scope_context_here = scope_context();
    let button_ref = NodeRef::<html::Button>::new();
    let shortcuts = help_keys_shortcuts(assistant().is_some());
    // For `HelpSignals`: opened, and closed again at once (not by asking).
    let signals = help_signals();
    let opened_at = StoredValue::new(0.0_f64);
    let asked = StoredValue::new(false);
    let signal = {
        let (topic_id, title) = (topic.id().to_string(), topic.title().to_string());
        move |what: HelpSignalKind| {
            let about = about.get_untracked().unwrap_or_else(|| title.clone());
            signals.emit(what, kind, about, Some(topic_id.clone()), None);
        }
    };
    let popover_ref = NodeRef::<html::Div>::new();

    // The topic on screen: the button's own, or a related one chosen in it.
    let shown = RwSignal::new(topic.clone());
    let registry = use_help_topics();
    let title_ref = NodeRef::<html::H3>::new();

    let title = topic.title().to_string();
    let topic_key = topic.id().to_string();
    let suggested = Signal::derive({
        let topic_key = topic_key.clone();
        move || signals.is_suggested(&topic_key)
    });
    let label = Signal::derive({
        let title = title.clone();
        move || {
            let label = format!("{}: {title}", help_label.get());
            if suggested.get() { format!("{label}{}", suggested_label.get()) } else { label }
        }
    });

    let on_toggle = move |_: Event| {
        let (Some(button), Some(popover)) = (button_ref.get_untracked(), popover_ref.get_untracked()) else { return };
        let is_open = popover.matches(":popover-open").unwrap_or(false);
        open.set(is_open);
        if is_open {
            let anchor: Element = button.unchecked_into();
            let floating: HtmlElement = popover.clone().unchecked_into();
            place(&anchor, &floating, false, Align::Start);
            let _ = popover.focus();
            follow(anchor, floating, false, Align::Start);
        }
    };

    // The signals come from `beforetoggle`, which fires as it happens:
    // Chromium merges an open and a close that come before its `toggle`
    // event into one closed-to-closed `toggle`.
    let own_topic = topic.clone();
    let on_before_toggle = move |event: Event| {
        let opening = js_sys::Reflect::get(&event, &"newState".into()).ok().and_then(|s| s.as_string()).as_deref() == Some("open");
        let now = js_sys::Date::now();
        if opening {
            shown.set(own_topic.clone());
            signals.clear(&topic_key);
            opened_at.set_value(now);
            asked.set_value(false);
            signal(HelpSignalKind::HelpOpened);
        } else if !asked.get_value() && now - opened_at.get_value() < 1000.0 {
            signal(HelpSignalKind::HelpDismissed);
        }
    };

    let body = move || view! { <HelpBlocks blocks=shown.get().blocks().to_vec() /> };
    let related = related_list(registry, shown, related_label, move || refocus(button_ref, popover_ref, title_ref));
    let ask = assistant().map(|on_ask| {
        let own = topic.clone();
        let closes = popover_id.clone();
        let ask = move |_| {
            asked.set_value(true);
            let current = shown.get_untracked();
            // `about` and `value` belong to what carries the `?`, so they go
            // only with its own topic.
            let request = if current.id() == own.id() {
                let label = about.get_untracked().unwrap_or_else(|| current.title().to_string());
                HelpRequest::new(&current, kind, label).with_value(value.get_untracked())
            } else {
                HelpRequest::new(&current, kind, current.title())
            };
            on_ask.run(request.with_context(scope_context_here.get_untracked()));
        };
        // A chat icon in the header: named by `ask_label`, its tooltip
        // names the topic shown. It stays in view while the topic scrolls.
        view! {
            <button
                type="button"
                class="icon-btn help-popover__ask"
                aria-label=move || ask_label.get()
                title=move || format!("{}: {}", ask_label.get(), shown.with(|t| t.title().to_string()))
                popovertarget=closes
                popovertargetaction="hide"
                on:click=ask
            >
                <Icon icon=MESSAGE_CIRCLE />
            </button>
        }
    });
    let guide = move || shown.get().guide_href().map(|href| view! {
        <a class="help-popover__link" href=href.to_string()>{move || guide_label.get()}</a>
    });
    let has_guide = move || shown.with(|t| t.guide_href().is_some());
    let actions = view! {
        <div class="help-popover__actions" hidden=move || !has_guide()>{guide}</div>
    };

    let popover = view! {
        <div
            popover="auto"
            id=popover_id
            node_ref=popover_ref
            class="help-popover"
            role="dialog"
            aria-labelledby=labelled_by
            tabindex="-1"
            on:toggle=on_toggle
        >
            // The header stays; only the topic scrolls under it.
            <div class="help-popover__header">
                <h3 class="help-popover__title" id=title_id node_ref=title_ref tabindex="-1">{move || shown.get().title().to_string()}</h3>
                {ask}
            </div>
            <div class="help-popover__scroll">
                <div class="help-popover__body">{body}</div>
                {related}
                {actions}
            </div>
        </div>
    }
    // Not `on:beforetoggle`: Leptos takes it for a bubbling event and
    // listens on the document, which it never reaches. A custom event is
    // listened for on the element.
    .add_any_attr(on(Custom::<Event>::new("beforetoggle"), on_before_toggle));

    view! {
        <button
            type="button"
            id=button_id
            node_ref=button_ref
            class=move || help_button_classes(&class.get().unwrap_or_default())
            aria-keyshortcuts=shortcuts
            aria-label=label
            title=label
            aria-expanded=move || if open.get() { "true" } else { "false" }
            data-suggested=move || suggested.get().then_some("true")
            aria-controls=controls
            popovertarget=target
            tabindex=skip_tab.then_some("-1")
        >
            <Icon icon=HELP />
        </button>
        {popover}
    }
}

/// The shown topic's related topics as buttons that show them in place;
/// nothing without a registry or related ids.
fn related_list(
    registry: Option<HelpTopics>,
    shown: RwSignal<HelpTopic>,
    label: Signal<String>,
    after_swap: impl Fn() + Clone + Send + Sync + 'static,
) -> impl IntoView {
    let heading_id = unique_id("ferrox-help-related");
    move || {
        let registry = registry.as_ref()?;
        let related = registry.related(&shown.get());
        if related.is_empty() {
            return None;
        }
        let items = related.into_iter().map(|topic| {
            let after_swap = after_swap.clone();
            let name = topic.title().to_string();
            let choose = move |_| {
                shown.set(topic.clone());
                after_swap();
            };
            view! {
                <li><button type="button" class="help-popover__related-link" on:click=choose>{name}</button></li>
            }
        }).collect_view();
        Some(view! {
            <div class="help-popover__related">
                <p class="help-popover__related-title" id=heading_id.clone()>{move || label.get()}</p>
                <ul class="help-popover__related-list" aria-labelledby=heading_id.clone()>{items}</ul>
            </div>
        })
    }
}

/// After a related topic is shown: keep the popover placed by its `?` (its
/// size changed) and move focus to the new title.
fn refocus(button: NodeRef<html::Button>, popover: NodeRef<html::Div>, title: NodeRef<html::H3>) {
    request_animation_frame(move || {
        if let (Some(button), Some(popover)) = (button.get_untracked(), popover.get_untracked()) {
            let anchor: Element = button.unchecked_into();
            let floating: HtmlElement = popover.unchecked_into();
            place(&anchor, &floating, false, Align::Start);
        }
        if let Some(title) = title.get_untracked() {
            let _ = title.focus();
        }
    });
}

/// A component's container classes, plus `help-reveal` when its `?` shows
/// only while it's active.
pub(crate) fn with_reveal(classes: String, reveal: bool) -> String {
    if reveal { format!("{classes} help-reveal") } else { classes }
}

/// The `?` a component places for its `help` prop, if it has a topic:
/// labelled for the request by `about`, sending `value` only if given.
/// `reveal`: shown only while the component's `.help-reveal` container is
/// active (the container's class is the component's to add). With it, the
/// `?`'s id, for the container's `data-help` (so F1 finds it).
pub(crate) fn component_help(
    help: MaybeHelp,
    kind: &'static str,
    about: Signal<String>,
    value: Signal<Option<String>>,
    skip_tab: bool,
    class: &'static str,
    reveal: bool,
) -> Option<(impl IntoView, String)> {
    let class = if reveal { format!("{class} help-btn--reveal") } else { class.to_string() };
    help.0.map(|topic| {
        let id = unique_id("ferrox-help-button");
        let view = view! {
            <HelpButton topic=topic kind=kind about=about value=value skip_tab=skip_tab id=id.clone() class=class />
        };
        (view, id)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes() {
        assert_eq!(help_button_classes(""), "icon-btn help-btn");
        assert_eq!(help_button_classes("x"), "icon-btn help-btn x");
        assert_eq!(with_reveal("field".into(), false), "field");
        assert_eq!(with_reveal("field".into(), true), "field help-reveal");
    }
}
