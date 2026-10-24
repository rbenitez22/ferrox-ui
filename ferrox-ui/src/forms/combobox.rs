use std::sync::Arc;

use ferrox_icons::{Icon, CHEVRON_DOWN, CLOSE, PLUS, REFRESH};
use leptos::html;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, KeyboardEvent, MouseEvent, ScrollIntoViewOptions, ScrollLogicalPosition};

use super::field::{control_classes, offer_value, wiring};
use crate::core::anchor::{follow, place, Align};
use crate::ids::unique_id;
use crate::{Spinner, SpinnerKind};

/// Whether an item's label matches what was typed: case-insensitive
/// "contains".
pub fn matches_text(label: &str, text: &str) -> bool {
    label.to_lowercase().contains(&text.to_lowercase())
}

/// The active option after an arrow key: `step` is +1 (down) or -1 (up).
/// From none, down goes to the first and up to the last; it stops at the
/// ends.
pub(crate) fn step_active(active: Option<usize>, len: usize, step: isize) -> Option<usize> {
    if len == 0 {
        return None;
    }
    let last = len as isize - 1;
    Some(match active {
        None if step > 0 => 0,
        None => last as usize,
        Some(i) => (i as isize + step).clamp(0, last) as usize,
    })
}

/// The text to offer creating from: what was typed, trimmed, when it is not
/// empty and no label matches it.
pub(crate) fn creatable(labels: impl IntoIterator<Item = String>, text: &str) -> Option<&str> {
    let text = text.trim();
    let mut labels = labels.into_iter();
    (!text.is_empty() && !labels.any(|label| matches_text(&label, text))).then_some(text)
}

/// The offer's default text.
fn default_create_label(text: String) -> String {
    format!("Create “{text}”")
}

/// The class list for a combobox's input.
fn input_classes(invalid: bool, extra: &str) -> String {
    control_classes("form-input", invalid, extra)
}

/// Keeps focus in the field: a press on the list or a control would
/// otherwise blur it, and a blur closes the list.
fn keep_focus(ev: MouseEvent) {
    ev.prevent_default();
}

/// A text field that filters a list and lets one item be chosen: the ARIA
/// combobox pattern (`role="combobox"` with a `listbox` of `option`s,
/// `aria-activedescendant` for the active one).
///
/// - Typing filters `items` by `label_of` (case-insensitive "contains");
///   the chevron shows them all.
/// - ↓ / ↑ move through the options (opening the list), Enter chooses, Tab
///   chooses and moves on, Esc closes the list (and, when it is closed,
///   clears the text). A click chooses.
/// - With `on_create`, text that matches nothing brings a "Create “…”"
///   button under the list; a click or Enter presses it, and `on_create`
///   gets the trimmed text. What creating does is the app's.
/// - `on_select` gets `Some(item)` when one is chosen, and `None` when the
///   text is cleared or the field is left with text that names no item.
/// - Its controls (clear, the chevron, refresh) are buttons out of the Tab
///   order, so they cost a row of fields no keystrokes.
/// - The list (and the offer) is in the top layer (a popover), placed under
///   the field and as wide as it: a `Dialog` body or any `overflow` parent
///   can't clip it.
///
/// It does not fetch: whatever loaded `items` owns reloading them
/// (`on_refresh`). Inside a `Field` it takes the field's id, description and
/// invalid state.
///
/// ```ignore
/// <Field label="Account">
///     <Combobox
///         items=accounts
///         key_of=|a: &Account| a.id.clone()
///         label_of=|a: &Account| a.name.clone()
///         text=account_text
///         on_select=move |a: Option<Account>| chosen.set(a)
///     />
/// </Field>
/// ```
#[component]
pub fn Combobox<T>(
    /// The items to choose from.
    #[prop(into)] items: Signal<Vec<T>>,
    /// Stable identity per item (`data-key` on its option).
    key_of: impl Fn(&T) -> String + Send + Sync + 'static,
    /// The text shown, and the text matched against.
    label_of: impl Fn(&T) -> String + Send + Sync + 'static,
    /// The text in the field.
    text: RwSignal<String>,
    #[prop(into)] on_select: UnsyncCallback<Option<T>>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    /// A control that clears the text (and the choice).
    #[prop(default = true)] clearable: bool,
    /// A chevron that shows and hides the whole list.
    #[prop(default = true)] toggleable: bool,
    /// Re-read the items; adds a refresh control.
    #[prop(optional, into)] on_refresh: Option<UnsyncCallback<()>>,
    #[prop(optional, into)] refreshing: Signal<bool>,
    /// Offers to create an item from the typed text when nothing matches.
    #[prop(optional, into)] on_create: Option<UnsyncCallback<String>>,
    /// The offer's text, given the typed text; `Create “{text}”` by default.
    #[prop(optional, into)] create_label: Option<Callback<String, String>>,
    #[prop(into, default = "No matches".into())] empty_label: Signal<String>,
    #[prop(into, default = "Clear".to_string())] clear_label: String,
    #[prop(into, default = "Show all".to_string())] toggle_label: String,
    #[prop(into, default = "Refresh".to_string())] refresh_label: String,
    #[prop(into, default = "Refreshing…".into())] refreshing_label: Signal<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional)] required: bool,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional)] autofocus: bool,
    /// Extra classes on the input. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
{
    let key_of = Arc::new(key_of);
    let label_of = Arc::new(label_of);
    let wired = wiring("ferrox-combobox");
    let invalid = wired.invalid;
    offer_value(text.into());

    let list_id = unique_id("ferrox-combobox-list");
    let option_id = {
        let list_id = list_id.clone();
        move |i: usize| format!("{list_id}-{i}")
    };
    let open = RwSignal::new(false);
    let show_all = RwSignal::new(false);
    let active = RwSignal::new(None::<usize>);
    let input_ref = NodeRef::<html::Input>::new();
    let field_ref = NodeRef::<html::Div>::new();
    let popup_ref = NodeRef::<html::Div>::new();

    let filtered = {
        let label_of = label_of.clone();
        Signal::derive(move || {
            let typed = text.get();
            let all = show_all.get();
            items.get().into_iter().filter(|item| all || (!typed.is_empty() && matches_text(&label_of(item), &typed))).collect::<Vec<T>>()
        })
    };
    let list_shown = Signal::derive(move || open.get() && (show_all.get() || !text.get().is_empty()));
    // What the offer would create, while it is shown.
    let offer = {
        let label_of = label_of.clone();
        let offers = on_create.is_some();
        Signal::derive(move || {
            if !offers || !list_shown.get() {
                return None;
            }
            let typed = text.get();
            items.with(|items| creatable(items.iter().map(|item| label_of(item)), &typed).map(str::to_string))
        })
    };

    // Show or hide the list popover, placed under the field and as wide.
    Effect::new(move |_| {
        let shown = list_shown.get();
        let (Some(popup), Some(field)) = (popup_ref.get(), field_ref.get()) else { return };
        let popup: HtmlElement = popup.unchecked_into();
        let is_open = popup.matches(":popover-open").unwrap_or(false);
        if shown && !is_open {
            let _ = popup.show_popover();
            let anchor: Element = field.unchecked_into();
            place(&anchor, &popup, true, Align::Start);
            follow(anchor, popup, true, Align::Start);
        } else if !shown && is_open {
            let _ = popup.hide_popover();
        }
    });

    // Keep the active option in view as the arrows move it.
    {
        let option_id = option_id.clone();
        Effect::new(move |_| {
            let Some(i) = active.get() else { return };
            if let Some(option) = document().get_element_by_id(&option_id(i)) {
                let options = ScrollIntoViewOptions::new();
                options.set_block(ScrollLogicalPosition::Nearest);
                option.scroll_into_view_with_scroll_into_view_options(&options);
            }
        });
    }

    let focus_field = move || {
        if let Some(input) = input_ref.get_untracked() {
            let _ = input.focus();
        }
    };
    let close = move || {
        open.set(false);
        show_all.set(false);
        active.set(None);
    };
    let choose = {
        let label_of = label_of.clone();
        move |item: T| {
            text.set(label_of(&item));
            close();
            on_select.run(Some(item));
        }
    };
    let create = move |typed: String| {
        if let Some(on_create) = on_create {
            close();
            on_create.run(typed);
        }
    };

    let on_input = move |_| {
        open.set(true);
        show_all.set(false);
        active.set(None);
    };

    let on_keydown = {
        let choose = choose.clone();
        move |ev: KeyboardEvent| {
            let key = ev.key();
            let chosen = || active.get_untracked().filter(|_| list_shown.get_untracked()).and_then(|i| filtered.get_untracked().get(i).cloned());
            match key.as_str() {
                "ArrowDown" | "ArrowUp" => {
                    ev.prevent_default();
                    if !list_shown.get_untracked() {
                        open.set(true);
                        if text.get_untracked().is_empty() {
                            show_all.set(true);
                        }
                    }
                    let step = if key == "ArrowDown" { 1 } else { -1 };
                    active.set(step_active(active.get_untracked(), filtered.get_untracked().len(), step));
                }
                "Enter" => {
                    if let Some(item) = chosen() {
                        ev.prevent_default();
                        choose(item);
                    } else if let Some(typed) = offer.get_untracked() {
                        ev.prevent_default();
                        create(typed);
                    }
                }
                "Tab" => {
                    if let Some(item) = chosen() {
                        choose(item);
                    }
                }
                "Escape" => {
                    // Handled here, so an enclosing dialog does not close too.
                    if list_shown.get_untracked() {
                        ev.prevent_default();
                        ev.stop_propagation();
                        close();
                    } else if !text.get_untracked().is_empty() {
                        ev.prevent_default();
                        ev.stop_propagation();
                        text.set(String::new());
                        on_select.run(None);
                    }
                }
                _ => {}
            }
        }
    };

    // Leaving the field: the list closes; text that names no item is no choice.
    let on_blur = {
        let label_of = label_of.clone();
        move |_| {
            close();
            let typed = text.get_untracked();
            let names_one = items.with_untracked(|items| items.iter().any(|item| label_of(item).eq_ignore_ascii_case(&typed)));
            if !typed.is_empty() && !names_one {
                on_select.run(None);
            }
        }
    };

    let clear = clearable.then(|| view! {
        <button
            type="button"
            class="icon-btn combobox__control"
            tabindex="-1"
            aria-label=clear_label.clone()
            title=clear_label
            hidden=move || text.get().is_empty()
            disabled=move || disabled.get()
            on:mousedown=keep_focus
            on:click=move |_| {
                text.set(String::new());
                close();
                on_select.run(None);
                focus_field();
            }
        >
            <Icon icon=CLOSE />
        </button>
    });
    let toggle = toggleable.then(|| view! {
        <button
            type="button"
            class="icon-btn combobox__control"
            tabindex="-1"
            aria-label=toggle_label.clone()
            title=toggle_label
            disabled=move || disabled.get()
            on:mousedown=keep_focus
            on:click=move |_| {
                if list_shown.get_untracked() {
                    close();
                } else {
                    open.set(true);
                    show_all.set(true);
                }
                focus_field();
            }
        >
            <Icon icon=CHEVRON_DOWN />
        </button>
    });
    let refresh = on_refresh.map(|on_refresh| view! {
        <button
            type="button"
            class="icon-btn combobox__control"
            tabindex="-1"
            aria-label=refresh_label.clone()
            title=refresh_label
            disabled=move || disabled.get() || refreshing.get()
            on:mousedown=keep_focus
            on:click=move |_| on_refresh.run(())
        >
            {move || if refreshing.get() {
                view! { <Spinner kind=SpinnerKind::Inline label=refreshing_label /> }.into_any()
            } else {
                view! { <Icon icon=REFRESH /> }.into_any()
            }}
        </button>
    });

    let create_offer = move || {
        offer.get().map(|typed| {
            let label = match create_label {
                Some(label) => label.run(typed.clone()),
                None => default_create_label(typed.clone()),
            };
            view! {
                <button
                    type="button"
                    class="combobox__create"
                    tabindex="-1"
                    on:mousedown=keep_focus
                    on:click=move |_| {
                        create(typed.clone());
                        focus_field();
                    }
                >
                    <Icon icon=PLUS />
                    <span>{label}</span>
                </button>
            }
        })
    };

    let options = {
        let option_id = option_id.clone();
        move || {
            let items = filtered.get();
            if items.is_empty() {
                return view! {
                    <li class="combobox__empty" role="option" aria-disabled="true" aria-selected="false">{move || empty_label.get()}</li>
                }.into_any();
            }
            items.into_iter().enumerate().map(|(i, item)| {
                let label = label_of(&item);
                let key = key_of(&item);
                let choose = choose.clone();
                view! {
                    <li
                        role="option"
                        id=option_id(i)
                        data-key=key
                        class=move || if active.get() == Some(i) { "combobox__option is-active" } else { "combobox__option" }
                        aria-selected=move || if active.get() == Some(i) { "true" } else { "false" }
                        on:mousedown=keep_focus
                        on:click=move |_| choose(item.clone())
                    >
                        {label}
                    </li>
                }
            }).collect_view().into_any()
        }
    };

    let active_descendant = move || active.get().filter(|_| list_shown.get()).map(&option_id);
    let controls = list_id.clone();

    view! {
        <div class="combobox" node_ref=field_ref>
            <input
                type="text"
                role="combobox"
                node_ref=input_ref
                id=wired.id
                class=move || input_classes(invalid.get(), &class.get().unwrap_or_default())
                aria-describedby=move || wired.described_by.get()
                aria-invalid=move || invalid.get().then_some("true")
                aria-autocomplete="list"
                aria-expanded=move || if list_shown.get() { "true" } else { "false" }
                aria-controls=controls
                aria-activedescendant=active_descendant
                autocomplete="off"
                placeholder=move || placeholder.get()
                name=move || name.get()
                required=required
                disabled=move || disabled.get()
                autofocus=autofocus
                bind:value=text
                on:input=on_input
                on:keydown=on_keydown
                on:blur=on_blur
            />
            <div class="combobox__controls">{clear}{toggle}{refresh}</div>
            <div popover="manual" class="combobox__popup" node_ref=popup_ref>
                <ul id=list_id role="listbox" class="combobox__list">
                    {options}
                </ul>
                {create_offer}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matching_is_case_insensitive_contains() {
        assert!(matches_text("Groceries", "gro"));
        assert!(matches_text("Groceries", "CERI"));
        assert!(!matches_text("Groceries", "rent"));
    }

    #[test]
    fn arrows_start_at_an_end_and_stop_at_the_ends() {
        assert_eq!(step_active(None, 3, 1), Some(0));
        assert_eq!(step_active(None, 3, -1), Some(2));
        assert_eq!(step_active(Some(0), 3, 1), Some(1));
        assert_eq!(step_active(Some(2), 3, 1), Some(2));
        assert_eq!(step_active(Some(0), 3, -1), Some(0));
        assert_eq!(step_active(Some(1), 0, 1), None);
    }

    #[test]
    fn creating_is_offered_only_for_text_that_matches_nothing() {
        let labels = || ["Groceries".to_string(), "Rent".to_string()];
        assert_eq!(creatable(labels(), "  milk "), Some("milk"));
        assert_eq!(creatable(labels(), "gro"), None);
        assert_eq!(creatable(labels(), "   "), None);
    }
}
