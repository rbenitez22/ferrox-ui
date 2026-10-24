//! One page, built only from ferrox-ui: a list of items in a panel, each a
//! card with its actions in a menu, an add dialog, a delete confirmation, and
//! the loading and empty states. Replace `Item` and the in-memory store with
//! your own data; the shape of the page stays.

use std::time::Duration;

use ferrox_icons::{Icon, DELETE, EDIT, MORE_VERTICAL, PLUS};
use ferrox_ui::{
    Button, Card, ConfirmDialog, DataView, Dialog, DialogAction, Field, HeadingLevel, IconButton, Input, Menu, MenuItem,
    MenuTrigger, Panel,
};
use leptos::prelude::*;

/// One record.
#[derive(Clone, Debug, PartialEq)]
struct Item {
    id: u32,
    name: String,
}

#[component]
pub fn App() -> impl IntoView {
    let items = RwSignal::new(Vec::<Item>::new());
    let loading = RwSignal::new(true);
    let next_id = StoredValue::new(1_u32);

    // The first load: a stand-in for a request, 300 ms later.
    set_timeout(
        move || {
            items.set(["First item", "Second item"].iter().map(|name| new_item(next_id, name)).collect());
            loading.set(false);
        },
        Duration::from_millis(300),
    );

    // Adding: a dialog with one field. `editing` holds the item being renamed,
    // or `None` while adding a new one.
    let dialog_open = RwSignal::new(false);
    let editing = RwSignal::new(None::<u32>);
    let name = RwSignal::new(String::new());
    let open_add = move || {
        editing.set(None);
        name.set(String::new());
        dialog_open.set(true);
    };
    let open_rename = move |item: Item| {
        editing.set(Some(item.id));
        name.set(item.name);
        dialog_open.set(true);
    };
    let save = move || {
        let text = name.get_untracked().trim().to_string();
        if text.is_empty() {
            return;
        }
        match editing.get_untracked() {
            Some(id) => items.update(|all| {
                if let Some(item) = all.iter_mut().find(|i| i.id == id) {
                    item.name = text;
                }
            }),
            None => items.update(|all| all.push(new_item(next_id, &text))),
        }
        dialog_open.set(false);
    };

    // Deleting asks first, naming what goes. Cancel, × and Esc close the
    // confirmation themselves; confirming closes it when the delete is done.
    let confirming = RwSignal::new(false);
    let to_delete = RwSignal::new(None::<Item>);
    let ask_delete = move |item: Item| {
        to_delete.set(Some(item));
        confirming.set(true);
    };
    let delete = move || {
        if let Some(gone) = to_delete.get_untracked() {
            items.update(|all| all.retain(|i| i.id != gone.id));
        }
        confirming.set(false);
    };

    // A card per item, its actions in a menu. The item goes in a
    // `StoredValue` (which is `Copy`), because the menu may build its items
    // more than once and every closure must be able to run again.
    let card = move |item: Item| {
        let (title, id) = (item.name.clone(), item.id);
        let this = StoredValue::new(item);
        view! {
            <Card title=title actions=move || view! {
                <Menu label=format!("{} actions", this.read_value().name) trigger=MenuTrigger::icon(MORE_VERTICAL, "More actions")>
                    <MenuItem icon=EDIT on_select=move || open_rename(this.get_value())>"Rename"</MenuItem>
                    <MenuItem icon=DELETE danger=true on_select=move || ask_delete(this.get_value())>"Delete"</MenuItem>
                </Menu>
            }>
                <p>{format!("Item #{id}")}</p>
            </Card>
        }
    };

    view! {
        <Panel
            title="Items"
            heading_level=HeadingLevel::H1
            actions=move || view! {
                <IconButton label="Add item" on_click=move || open_add()><Icon icon=PLUS /></IconButton>
            }
        >
            <DataView
                items=items
                // DataView redraws an item only when its key changes: a key
                // that includes the name redraws a renamed card.
                key_of=|item: &Item| format!("{}:{}", item.id, item.name)
                item=card
                label="Items"
                loading=loading
                loading_label="Loading items…"
                empty_view=move || view! {
                    <p>"No items yet."</p>
                    <Button on_click=move || open_add()>"Add an item"</Button>
                }
            />
        </Panel>
        <Dialog
            open=dialog_open
            title=Signal::derive(move || if editing.get().is_some() { "Rename item".to_string() } else { "Add item".to_string() })
            affirm=DialogAction::new("Save", save)
        >
            <Field label="Name">
                <Input value=name autofocus=true required=true />
            </Field>
        </Dialog>
        <ConfirmDialog
            open=confirming
            title="Delete item?"
            confirm_label="Delete"
            on_confirm=delete
        >
            <p>{move || format!("This deletes “{}”.", to_delete.get().map(|i| i.name).unwrap_or_default())}</p>
        </ConfirmDialog>
    }
}

/// A new item with the next id.
fn new_item(next_id: StoredValue<u32>, name: &str) -> Item {
    let id = next_id.get_value();
    next_id.set_value(id + 1);
    Item { id, name: name.to_string() }
}
