//! Recipe: a settings page. A narrow page of panels: everyday settings at
//! the top (switches and a select, applied at once), rarer ones in a
//! collapsible panel, and a danger zone that asks before it destroys.

use ferrox_ui::{
    Button, ButtonFamily, ConfirmDialog, Field, Gap, Page, PageWidth, Panel, PanelTone, Select, SelectOption, Stack, Switch,
};
use leptos::prelude::*;

use super::store::use_store;

#[component]
pub fn SettingsPage() -> impl IntoView {
    let store = use_store();
    let settings = store.settings;
    let units = Signal::stored(vec![SelectOption::new("metric", "Metric (grams, litres)"), SelectOption::new("us", "US (cups, ounces)")]);

    let confirming = RwSignal::new(false);
    let deleting = RwSignal::new(false);
    let delete_all = move || {
        deleting.set(true);
        store.remove_all(move || {
            deleting.set(false);
            confirming.set(false);
        });
    };

    view! {
        <Page
            title="Settings"
            width=PageWidth::Narrow
            more=move || view! {
                <Panel title="Advanced" collapsible=true summary="Deleting">
                    <Switch checked=settings.confirm_delete label="Ask before deleting a recipe" />
                </Panel>
                <Panel title="Danger zone" tone=PanelTone::Danger collapsible=true summary="Delete every recipe">
                    <Stack gap=Gap::Sm>
                        <p>"Deletes every recipe. There's no undo."</p>
                        <div>
                            <Button family=ButtonFamily::Danger on_click=move || confirming.set(true)>"Delete all recipes…"</Button>
                        </div>
                    </Stack>
                </Panel>
            }
        >
            <Stack gap=Gap::Lg>
                <Switch checked=settings.show_minutes label="Show cooking times on cards" />
                <Field label="Units" hint="Used on each recipe's page">
                    <Select value=settings.units options=units />
                </Field>
            </Stack>
        </Page>
        <ConfirmDialog open=confirming title="Delete every recipe?" confirm_label="Delete all" saving=deleting on_confirm=delete_all>
            <p>{move || format!("This deletes all {} recipes.", store.recipes.with(Vec::len))}</p>
        </ConfirmDialog>
    }
}
