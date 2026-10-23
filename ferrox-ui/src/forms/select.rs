use ferrox_icons::{Icon, REFRESH};
use leptos::prelude::*;
use leptos::{component, view, IntoView};

use super::field::{control_classes, wiring};
use crate::{IconButton, Spinner, SpinnerKind};

/// One choice of a [`Select`]: the value it sets and the text it shows.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self { value: value.into(), label: label.into() }
    }
}

/// A dropdown of fixed choices (`.form-select`, the native `<select>`),
/// bound two-way to `value` (the chosen option's `value`; empty for none).
///
/// - The first entry is the `placeholder`. It can't be chosen, and it shows
///   while `value` is empty.
/// - `loading`: the options are on their way. The select is disabled and the
///   placeholder reads `loading_label`.
/// - `on_refresh` adds a refresh button beside it; while `refreshing`, a
///   spinner takes the icon's place and the button refuses clicks.
/// - The `<select>` is rendered once: only its options and which one is
///   selected react, so it keeps focus when they change.
///
/// Inside a `Field` it takes the field's id, description and invalid state.
///
/// ```ignore
/// <Field label="Currency">
///     <Select value=currency options=currencies placeholder="Choose a currency" />
/// </Field>
/// ```
#[component]
pub fn Select(
    value: RwSignal<String>,
    #[prop(into)] options: Signal<Vec<SelectOption>>,
    #[prop(into, default = "Select…".into())] placeholder: Signal<String>,
    #[prop(optional, into)] loading: Signal<bool>,
    #[prop(into, default = "Loading…".into())] loading_label: Signal<String>,
    #[prop(optional, into)] on_refresh: Option<UnsyncCallback<()>>,
    #[prop(optional, into)] refreshing: Signal<bool>,
    /// The refresh button's label.
    #[prop(into, default = "Refresh".to_string())] refresh_label: String,
    #[prop(into, default = "Refreshing…".into())] refreshing_label: Signal<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional)] required: bool,
    #[prop(optional, into)] disabled: Signal<bool>,
    /// Extra classes on the `<select>`. May be reactive.
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    let wired = wiring("ferrox-select");
    let invalid = wired.invalid;

    let refresh = on_refresh.map(|on_refresh| view! {
        <IconButton
            title=refresh_label
            class="select__refresh"
            disabled=refreshing
            on_click=move |_| on_refresh.run(())
        >
            {move || if refreshing.get() {
                view! { <Spinner kind=SpinnerKind::Inline label=refreshing_label /> }.into_any()
            } else {
                view! { <Icon icon=REFRESH /> }.into_any()
            }}
        </IconButton>
    });

    view! {
        <div class="select">
            <select
                id=wired.id
                class=move || control_classes("form-select", invalid.get(), &class.get().unwrap_or_default())
                aria-describedby=move || wired.described_by.get()
                aria-invalid=move || invalid.get().then_some("true")
                aria-busy=move || loading.get().then_some("true")
                name=move || name.get()
                required=required
                disabled=move || disabled.get() || loading.get()
                on:change=move |ev| value.set(event_target_value(&ev))
            >
                <option value="" disabled=true hidden=true prop:selected=move || value.get().is_empty()>
                    {move || if loading.get() { loading_label.get() } else { placeholder.get() }}
                </option>
                <For each=move || options.get() key=|option| option.value.clone() let:option>
                    {
                        let chosen = option.value.clone();
                        view! {
                            <option value=option.value prop:selected=move || value.with(|v| *v == chosen)>
                                {option.label}
                            </option>
                        }
                    }
                </For>
            </select>
            {refresh}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_option_keeps_its_value_and_label() {
        let option = SelectOption::new("eur", "Euro");
        assert_eq!((option.value.as_str(), option.label.as_str()), ("eur", "Euro"));
    }
}
