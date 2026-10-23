//! Leptos access to the theme: reactive signals for the choice and the active
//! id, kept in step with [`super::on_change`].
//!
//! ```rust,ignore
//! #[component]
//! fn App() -> impl IntoView {
//!     ferrox_theme::provide_theme();
//!     view! { <Settings/> }
//! }
//!
//! #[component]
//! fn Settings() -> impl IntoView {
//!     let theme = ferrox_theme::use_theme();
//!     view! {
//!         <p>"Showing " {move || theme.active.get()}</p>
//!         <button on:click=move |_| theme.set_mode(ThemeMode::Dark)>"Dark"</button>
//!     }
//! }
//! ```

use leptos::prelude::*;

use super::{ThemeChoice, ThemeFamily, ThemeMode};

/// Reactive theme state. `Copy`, so it moves freely into closures.
#[derive(Debug, Clone, Copy)]
pub struct ThemeContext {
    /// The user's choice. Updates on every change.
    pub choice: ReadSignal<ThemeChoice>,
    /// The `data-theme` id shown now (`""` = `:root` default). Also updates
    /// when the device flips light/dark in `system` mode.
    pub active: ReadSignal<&'static str>,
}

impl ThemeContext {
    /// Shows and stores `choice`. See [`super::set_choice`].
    pub fn set_choice(&self, choice: ThemeChoice) {
        super::set_choice(choice);
    }

    /// Switches family, keeping the mode.
    pub fn set_family(&self, family: &str) {
        super::set_family(family);
    }

    /// Switches mode, keeping the family.
    pub fn set_mode(&self, mode: ThemeMode) {
        super::set_mode(mode);
    }

    /// Every family, default first. Not reactive: the set is fixed after `init()`.
    pub fn families(&self) -> Vec<ThemeFamily> {
        super::families()
    }
}

/// Creates the context and provides it to the component tree. Call once,
/// near the root, after `theme::init()`. Returns the context too.
pub fn provide_theme() -> ThemeContext {
    let (choice, set_choice) = signal(super::choice());
    let (active, set_active) = signal(super::active());

    let listener = super::on_change(move |new_choice, new_active| {
        // try_set: a change after this owner was disposed is simply ignored.
        set_choice.try_set(new_choice.clone());
        set_active.try_set(new_active);
    });
    on_cleanup(move || super::remove_listener(listener));

    let context = ThemeContext { choice, active };
    provide_context(context);
    context
}

/// The context from [`provide_theme`].
///
/// # Panics
/// If no ancestor called [`provide_theme`].
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>()
        .expect("use_theme(): call ferrox_theme::provide_theme() in a parent component")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signals_follow_changes_until_cleanup() {
        let owner = Owner::new();
        let context = owner.with(provide_theme);
        assert_eq!(owner.with(use_theme).choice.get_untracked(), ThemeChoice::default());

        super::super::listeners::notify(&ThemeChoice::new("jamaica", ThemeMode::Dark), "jamaica-dark");
        assert_eq!(context.choice.get_untracked(), ThemeChoice::new("jamaica", ThemeMode::Dark));
        assert_eq!(context.active.get_untracked(), "jamaica-dark");

        drop(owner);
        // Listener removed on cleanup; a later change must not panic.
        super::super::listeners::notify(&ThemeChoice::default(), "");
    }
}
