//! Callbacks run after every theme change: a `set_choice`, or the device
//! flipping light/dark while in `system` mode.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use super::choice::ThemeChoice;

type Listener = Rc<dyn Fn(&ThemeChoice, &'static str)>;

/// Returned by [`super::on_change`]; pass to [`super::remove_listener`] to stop.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListenerId(u64);

thread_local! {
    static LISTENERS: RefCell<Vec<(ListenerId, Listener)>> = const { RefCell::new(Vec::new()) };
    static NEXT_ID: Cell<u64> = const { Cell::new(0) };
}

pub(crate) fn add(listener: impl Fn(&ThemeChoice, &'static str) + 'static) -> ListenerId {
    let id = ListenerId(NEXT_ID.replace(NEXT_ID.get() + 1));
    LISTENERS.with_borrow_mut(|listeners| listeners.push((id, Rc::new(listener))));
    id
}

pub(crate) fn remove(id: ListenerId) {
    LISTENERS.with_borrow_mut(|listeners| listeners.retain(|(each, _)| *each != id));
}

/// Calls every listener. The list is copied first, so a listener may itself
/// add or remove listeners, or change the theme.
pub(crate) fn notify(choice: &ThemeChoice, active: &'static str) {
    let snapshot: Vec<Listener> = LISTENERS.with_borrow(|listeners| listeners.iter().map(|(_, l)| l.clone()).collect());
    for listener in snapshot {
        listener(choice, active);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ThemeMode;

    #[test]
    fn add_notify_remove() {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        let id = add(move |choice, active| sink.borrow_mut().push((choice.format(), active)));

        notify(&ThemeChoice::new("jamaica", ThemeMode::Dark), "jamaica-dark");
        remove(id);
        notify(&ThemeChoice::default(), "");

        assert_eq!(*seen.borrow(), [("jamaica:dark".to_string(), "jamaica-dark")]);
    }

    #[test]
    fn listener_may_remove_itself() {
        let id = Rc::new(Cell::new(None));
        let own = id.clone();
        id.set(Some(add(move |_, _| remove(own.get().unwrap()))));
        notify(&ThemeChoice::default(), "");
        notify(&ThemeChoice::default(), "");
        assert!(LISTENERS.with_borrow(|l| l.iter().all(|(each, _)| Some(*each) != id.get())));
    }
}
