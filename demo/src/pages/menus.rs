//! Menus: each kind of trigger (an icon at a row's end, a button, a custom
//! view with a header), item icons, a danger item, a disabled one, a
//! current one, links, a split button (saving, locked), context menus on
//! rows (one disabled), a log of what was chosen, a disabled menu,
//! a menu inside a dialog, and one low on the page to watch it flip above.

use ferrox_icons::{DELETE, EDIT, MORE_VERTICAL};
use std::time::Duration;

use ferrox_ui::{Button, ButtonFamily, ContextMenu, Dialog, Menu, MenuAlign, MenuDivider, MenuItem, MenuLink, MenuTrigger, SplitButton, Switch};
use leptos::prelude::*;

use crate::section::Section;

#[component]
pub fn MenusPage() -> impl IntoView {
    let chosen = RwSignal::new(Vec::<String>::new());
    let choose = move |what: &'static str| move || chosen.update(|c| c.push(what.to_string()));
    let dialog_open = RwSignal::new(false);
    let archiving = RwSignal::new(false);
    const BOOKS: [&str; 3] = ["Household", "Business", "Allotment"];
    let book = RwSignal::new(BOOKS[0]);

    view! {
        <Section title="Menu">
            <div class="demo-menu-row">
                <span>"Shakshuka"</span>
                <Menu label="Recipe actions" trigger=MenuTrigger::icon(MORE_VERTICAL, "More actions")>
                    <MenuItem icon=EDIT on_select=choose("edit")>"Edit"</MenuItem>
                    <MenuItem on_select=choose("duplicate")>"Duplicate"</MenuItem>
                    <MenuItem on_select=choose("archive") disabled=Signal::derive(move || !archiving.get())>"Archive"</MenuItem>
                    <MenuDivider />
                    <MenuItem icon=DELETE danger=true on_select=choose("delete")>"Delete"</MenuItem>
                </Menu>
            </div>
            <div class="demo-row">
                <Menu label="Export" trigger=MenuTrigger::button("Export") align=MenuAlign::Start>
                    <MenuItem on_select=choose("pdf")>"As PDF"</MenuItem>
                    <MenuItem on_select=choose("csv")>"As CSV"</MenuItem>
                    <MenuItem on_select=choose("print")>"Print"</MenuItem>
                </Menu>
                <Menu
                    label="Account"
                    trigger=MenuTrigger::custom(|| view! { <span class="demo-avatar">"AL"</span><span>"Ada Lovelace"</span> }, "Account")
                    header=|| view! { "Signed in as ada@example.org" }
                >
                    <MenuItem on_select=choose("settings")>"Settings"</MenuItem>
                    <MenuItem on_select=choose("help")>"Help"</MenuItem>
                    <MenuItem on_select=choose("sign out")>"Sign out"</MenuItem>
                </Menu>
                <Menu label="Books" trigger=MenuTrigger::Button { text: "Book".into(), family: ButtonFamily::Primary } align=MenuAlign::Start>
                    {BOOKS.into_iter().map(|name| view! {
                        <MenuItem current=Signal::derive(move || book.get() == name) on_select=move || {
                            book.set(name);
                            chosen.update(|c| c.push(format!("book {name}")));
                        }>{name}</MenuItem>
                    }).collect_view()}
                </Menu>
                <Menu label="Go to" trigger=MenuTrigger::button("Go to") align=MenuAlign::Start>
                    <MenuLink href="/layout">"Layout"</MenuLink>
                    <MenuLink href="/data" icon=EDIT>"Data"</MenuLink>
                    <MenuLink href="/navigation">"Navigation, a page whose name is far too long for any menu"</MenuLink>
                </Menu>
                <Menu label="Go to, wider" trigger=MenuTrigger::button("Go to, wider") align=MenuAlign::Start class="demo-wide-menu">
                    <MenuLink href="/layout">"Layout"</MenuLink>
                    <MenuLink href="/data" icon=EDIT>"Data"</MenuLink>
                    <MenuLink href="/navigation">"Navigation, a page whose name is far too long for any menu"</MenuLink>
                </Menu>
                <Menu label="Locked actions" trigger=MenuTrigger::button("Locked") disabled=true>
                    <MenuItem on_select=choose("never")>"Never"</MenuItem>
                </Menu>
                <Button family=ButtonFamily::Secondary on_click=move |_| dialog_open.set(true)>"Menu in a dialog…"</Button>
            </div>
            <Switch checked=archiving label="Archiving allowed" />
            <dl class="demo-echo">
                <dt>"book"</dt><dd data-echo="book">{move || book.get()}</dd>
                <dt>"chosen"</dt><dd data-echo="chosen">{move || chosen.get().join(", ")}</dd>
            </dl>
            <Dialog open=dialog_open title="Move a recipe" on_close=move || dialog_open.set(false)>
                <p>"Where to?"</p>
                <Menu label="Destinations" trigger=MenuTrigger::button("Destination")>
                    <MenuItem on_select=choose("to breakfast")>"Breakfast"</MenuItem>
                    <MenuItem on_select=choose("to dinner")>"Dinner"</MenuItem>
                </Menu>
            </Dialog>
        </Section>
        <SplitButtons chosen=chosen />
        <ContextRows chosen=chosen />
        <Section title="Low on the page">
            <p>"Make the window short enough and this menu opens above its button."</p>
            <div>
                <Menu label="Low actions" trigger=MenuTrigger::button("Low menu")>
                    <MenuItem on_select=choose("low one")>"One"</MenuItem>
                    <MenuItem on_select=choose("low two")>"Two"</MenuItem>
                    <MenuItem on_select=choose("low three")>"Three"</MenuItem>
                    <MenuItem on_select=choose("low four")>"Four"</MenuItem>
                </Menu>
            </div>
        </Section>
    }
}

/// A split button: Save (a second of saving, then counted) with Save as
/// draft and Save and close behind its chevron; a switch locks it.
#[component]
fn SplitButtons(chosen: RwSignal<Vec<String>>) -> impl IntoView {
    let saving = RwSignal::new(false);
    let locked = RwSignal::new(false);
    let saves = RwSignal::new(0_u32);
    let save = move |_| {
        saving.set(true);
        set_timeout(move || {
            saves.update(|n| *n += 1);
            saving.set(false);
        }, Duration::from_millis(1000));
    };
    view! {
        <Section title="Split button">
            <div class="demo-row">
                <SplitButton text="Save" on_click=save menu_label="More save options" saving=saving disabled=locked>
                    <MenuItem on_select=move || chosen.update(|c| c.push("save as draft".into()))>"Save as draft"</MenuItem>
                    <MenuItem on_select=move || chosen.update(|c| c.push("save and close".into()))>"Save and close"</MenuItem>
                </SplitButton>
                <SplitButton text="Publish" family=ButtonFamily::Secondary on_click=move |_| chosen.update(|c| c.push("publish".into())) menu_label="More publish options">
                    <MenuItem on_select=move || chosen.update(|c| c.push("schedule".into()))>"Schedule…"</MenuItem>
                </SplitButton>
                <Switch checked=locked label="Saving locked" />
            </div>
            <dl class="demo-echo">
                <dt>"saves"</dt><dd data-echo="saves">{move || saves.get().to_string()}</dd>
            </dl>
        </Section>
    }
}

/// Rows with a context menu each (right-click, Shift+F10, the Menu key, a
/// long press), the same actions as a visible ⋮ would offer; the last row's
/// is disabled, so the browser's own menu stays.
#[component]
fn ContextRows(chosen: RwSignal<Vec<String>>) -> impl IntoView {
    let opened = RwSignal::new(Vec::<String>::new());
    let row = move |name: &'static str, disabled: bool| {
        let log = move |what: &'static str| move || chosen.update(|c| c.push(format!("{what} {name}")));
        view! {
            <ContextMenu
                label=format!("{name} actions")
                disabled=disabled
                items=move || view! {
                    <MenuItem icon=EDIT on_select=log("edit")>"Edit"</MenuItem>
                    <MenuItem on_select=log("duplicate")>"Duplicate"</MenuItem>
                    <MenuDivider />
                    <MenuItem icon=DELETE danger=true on_select=log("delete")>"Delete"</MenuItem>
                }
            >
                <div class="demo-context-row" data-row=name>
                    <span>{name}</span>
                    <Button family=ButtonFamily::Secondary on_click=move |_| opened.update(|o| o.push(name.to_string()))>
                        {format!("Open {name}")}
                    </Button>
                </div>
            </ContextMenu>
        }
    };
    view! {
        <Section title="Context menu">
            <p>"Right-click a row, press Shift+F10 or the Menu key on its button, or press and hold on a touch screen."</p>
            {row("Shakshuka", false)}
            {row("Ribollita", false)}
            {row("Pierogi", true)}
            <p class="demo-outside" data-outside="true">"Outside every row: the browser's own menu."</p>
            <dl class="demo-echo">
                <dt>"opened"</dt><dd data-echo="opened">{move || opened.get().join(", ")}</dd>
            </dl>
        </Section>
    }
}
