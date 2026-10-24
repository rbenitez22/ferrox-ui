//! Menus: each kind of trigger (an icon at a row's end, a button, a custom
//! view), a log of what was chosen, a disabled menu,
//! a menu inside a dialog, and one low on the page to watch it flip above.

use ferrox_icons::MORE_VERTICAL;
use ferrox_ui::{Button, ButtonFamily, Dialog, Menu, MenuAlign, MenuDivider, MenuItem, MenuTrigger};
use leptos::prelude::*;

use crate::section::Section;

#[component]
pub fn MenusPage() -> impl IntoView {
    let chosen = RwSignal::new(Vec::<String>::new());
    let choose = move |what: &'static str| move || chosen.update(|c| c.push(what.to_string()));
    let dialog_open = RwSignal::new(false);

    view! {
        <Section title="Menu">
            <div class="demo-menu-row">
                <span>"Shakshuka"</span>
                <Menu label="Recipe actions" trigger=MenuTrigger::icon(MORE_VERTICAL, "More actions")>
                    <MenuItem on_select=choose("edit")>"Edit"</MenuItem>
                    <MenuItem on_select=choose("duplicate")>"Duplicate"</MenuItem>
                    <MenuItem on_select=choose("archive")>"Archive"</MenuItem>
                    <MenuDivider />
                    <MenuItem on_select=choose("delete")>"Delete"</MenuItem>
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
                >
                    <MenuItem on_select=choose("settings")>"Settings"</MenuItem>
                    <MenuItem on_select=choose("help")>"Help"</MenuItem>
                    <MenuItem on_select=choose("sign out")>"Sign out"</MenuItem>
                </Menu>
                <Menu label="Locked actions" trigger=MenuTrigger::button("Locked") disabled=true>
                    <MenuItem on_select=choose("never")>"Never"</MenuItem>
                </Menu>
                <Button family=ButtonFamily::Secondary on_click=move |_| dialog_open.set(true)>"Menu in a dialog…"</Button>
            </div>
            <dl class="demo-echo">
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
