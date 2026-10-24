//! Forms: fields and their controls, each with its value echoed below, so
//! labels, messages, invalid states and two-way binding can be checked.

use std::time::Duration;

use ferrox_ui::{
    Button, ButtonFamily, Checkbox, Combobox, Dialog, DialogAction, Field, Input, InputKind, PasswordInput, Select, SelectOption,
    Switch, Textarea,
};
use leptos::prelude::*;

use crate::section::Section;

#[component]
pub fn FormsPage() -> impl IntoView {
    view! {
        <Fields />
        <Passwords />
        <Choices />
        <Comboboxes />
    }
}

/// A hint, a live error, a textarea, and an input outside any field.
#[component]
fn Fields() -> impl IntoView {
    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let notes = RwSignal::new(String::new());
    let query = RwSignal::new(String::new());
    let email_error = Signal::derive(move || {
        let email = email.get();
        (!email.is_empty() && !email.contains('@')).then(|| "An email address has an @ in it.".to_string())
    });
    view! {
        <Section title="Fields">
            <div class="demo-form">
                <Field label="Name" hint="As it appears on statements">
                    <Input value=name name="name" />
                </Field>
                <Field label="Email" hint="Where receipts go" error=email_error>
                    <Input value=email kind=InputKind::Email name="email" autocomplete="email" />
                </Field>
                <Field label="Notes">
                    <Textarea value=notes name="notes" placeholder="Anything to remember" />
                </Field>
                <Input value=query kind=InputKind::Search name="query" placeholder="Search (no field)" />
            </div>
            <div class="demo-row">
                <Button family=ButtonFamily::Secondary on_click=move || { name.set("Groceries".into()); notes.set("Line one\nLine two".into()); }>
                    "Fill from code"
                </Button>
            </div>
            <dl class="demo-echo">
                <dt>"name"</dt><dd data-echo="name">{move || name.get()}</dd>
                <dt>"email"</dt><dd data-echo="email">{move || email.get()}</dd>
                <dt>"notes"</dt><dd data-echo="notes">{move || notes.get()}</dd>
                <dt>"query"</dt><dd data-echo="query">{move || query.get()}</dd>
            </dl>
        </Section>
    }
}

/// Signing in, and setting a new password with its strength meter.
#[component]
fn Passwords() -> impl IntoView {
    let password = RwSignal::new(String::new());
    let new_password = RwSignal::new(String::new());
    view! {
        <Section title="Passwords">
            <div class="demo-form">
                <Field label="Password">
                    <PasswordInput value=password name="password" />
                </Field>
                <Field label="New password" hint="At least 80 bits: long, and mixed">
                    <PasswordInput value=new_password name="new-password" autocomplete="new-password" strength=true />
                </Field>
            </div>
            <dl class="demo-echo">
                <dt>"password"</dt><dd data-echo="password">{move || password.get()}</dd>
                <dt>"new password"</dt><dd data-echo="new-password">{move || new_password.get()}</dd>
            </dl>
        </Section>
    }
}

/// Checkboxes (a hint, an error until ticked), a switch, and selects: a
/// plain one, and one that loads and refreshes its options.
#[component]
fn Choices() -> impl IntoView {
    let recurring = RwSignal::new(false);
    let terms = RwSignal::new(false);
    let archived = RwSignal::new(false);
    let currency = RwSignal::new(String::new());
    let account = RwSignal::new(String::new());
    let loading = RwSignal::new(false);
    let refreshing = RwSignal::new(false);
    let created = RwSignal::new(0_u32);
    let accounts = RwSignal::new(vec![
        SelectOption::new("checking", "Checking"),
        SelectOption::new("savings", "Savings"),
    ]);
    let currencies = Signal::stored(vec![
        SelectOption::new("eur", "Euro"),
        SelectOption::new("usd", "US dollar"),
        SelectOption::new("gbp", "Pound sterling"),
    ]);
    let terms_error = Signal::derive(move || (!terms.get()).then(|| "Tick this to continue.".to_string()));
    let refresh = move || {
        refreshing.set(true);
        set_timeout(move || {
            accounts.update(|a| a.push(SelectOption::new("brokerage", "Brokerage")));
            refreshing.set(false);
        }, Duration::from_millis(600));
    };
    // A real app would ask for the name in a dialog; a numbered one will do here.
    let create_account = move || {
        created.update(|n| *n += 1);
        let n = created.get_untracked();
        let value = format!("new-{n}");
        accounts.update(|a| a.push(SelectOption::new(value.clone(), format!("New account {n}"))));
        account.set(value);
    };
    view! {
        <Section title="Choices">
            <div class="demo-form">
                <Checkbox checked=recurring label="Repeat every month" hint="Until you stop it" name="recurring" />
                <Checkbox checked=terms label="I accept the terms" error=terms_error name="terms" />
                <div class="demo-row">
                    <Switch checked=archived label="Show archived accounts" />
                    <Switch checked=loading label="Accounts loading" />
                </div>
                <Field label="Currency">
                    <Select value=currency options=currencies placeholder="Choose a currency" name="currency" />
                </Field>
                <Field label="Account">
                    <Select value=account options=accounts loading=loading on_refresh=refresh refreshing=refreshing on_create=create_account create_label="New account…" name="account" />
                </Field>
            </div>
            <div class="demo-row">
                <Button family=ButtonFamily::Secondary on_click=move || currency.set("gbp".into())>"Choose pounds from code"</Button>
            </div>
            <dl class="demo-echo">
                <dt>"recurring"</dt><dd data-echo="recurring">{move || recurring.get().to_string()}</dd>
                <dt>"terms"</dt><dd data-echo="terms">{move || terms.get().to_string()}</dd>
                <dt>"archived"</dt><dd data-echo="archived">{move || archived.get().to_string()}</dd>
                <dt>"currency"</dt><dd data-echo="currency">{move || currency.get()}</dd>
                <dt>"account"</dt><dd data-echo="account">{move || account.get()}</dd>
                <dt>"created"</dt><dd data-echo="account-created">{move || created.get().to_string()}</dd>
            </dl>
        </Section>
    }
}

/// One account to choose: an id, and the name shown and matched.
#[derive(Clone, Debug, PartialEq)]
struct Account {
    id: String,
    name: String,
}

const ACCOUNT_NAMES: [&str; 24] = [
    "Assets:Checking", "Assets:Savings", "Assets:Brokerage", "Assets:Cash",
    "Liabilities:Visa", "Liabilities:Mortgage", "Income:Salary", "Income:Interest",
    "Expenses:Groceries", "Expenses:Dining", "Expenses:Rent", "Expenses:Utilities:Power",
    "Expenses:Utilities:Water", "Expenses:Utilities:Internet", "Expenses:Fuel", "Expenses:Insurance",
    "Expenses:Clothing", "Expenses:Gifts", "Expenses:Books", "Expenses:Travel",
    "Expenses:Health", "Expenses:Household", "Expenses:Garden", "Equity:Opening Balances",
];

fn accounts() -> Vec<Account> {
    ACCOUNT_NAMES.iter().map(|name| Account { id: name.to_lowercase().replace([':', ' '], "-"), name: name.to_string() }).collect()
}

/// A combobox of accounts (with refresh and create), the choice it reports,
/// and the same combobox inside a dialog, where its list must not be clipped.
#[component]
fn Comboboxes() -> impl IntoView {
    let items = RwSignal::new(accounts());
    let text = RwSignal::new(String::new());
    let chosen = RwSignal::new(None::<Account>);
    let reports = RwSignal::new(0_u32);
    let refreshing = RwSignal::new(false);
    let refresh = move || {
        refreshing.set(true);
        set_timeout(move || {
            items.update(|a| a.push(Account { id: "expenses-pets".into(), name: "Expenses:Pets".into() }));
            refreshing.set(false);
        }, Duration::from_millis(600));
    };
    let on_select = move |account: Option<Account>| {
        chosen.set(account);
        reports.update(|n| *n += 1);
    };
    // Creating adds the account to the list in the page, then chooses it.
    let created = RwSignal::new(Vec::<String>::new());
    let create = move |name: String| {
        let account = Account { id: name.to_lowercase().replace([':', ' '], "-"), name: name.clone() };
        items.update(|a| a.push(account.clone()));
        created.update(|c| c.push(name));
        text.set(account.name.clone());
        on_select(Some(account));
    };

    let dialog_open = RwSignal::new(false);
    let dialog_text = RwSignal::new(String::new());
    let dialog_chosen = RwSignal::new(None::<Account>);
    let saves = RwSignal::new(0_u32);
    let on_dialog_select = move |account: Option<Account>| dialog_chosen.set(account);
    view! {
        <Section title="Combobox">
            <div class="demo-form">
                <Field label="Account" hint="Type part of a name, or open the list">
                    <Combobox
                        items=items
                        key_of=|a: &Account| a.id.clone()
                        label_of=|a: &Account| a.name.clone()
                        text=text
                        on_select=on_select
                        on_refresh=refresh
                        refreshing=refreshing
                        on_create=create
                        placeholder="Choose an account"
                        name="account-combo"
                    />
                </Field>
            </div>
            <div class="demo-row">
                <Button family=ButtonFamily::Secondary on_click=move || dialog_open.set(true)>"Combobox in a dialog…"</Button>
            </div>
            <dl class="demo-echo">
                <dt>"text"</dt><dd data-echo="combo-text">{move || text.get()}</dd>
                <dt>"chosen"</dt><dd data-echo="combo-chosen">{move || chosen.get().map(|a| a.id).unwrap_or_else(|| "(none)".into())}</dd>
                <dt>"reports"</dt><dd data-echo="combo-reports">{move || reports.get().to_string()}</dd>
                <dt>"created"</dt><dd data-echo="combo-created">{move || created.get().join(", ")}</dd>
                <dt>"in dialog"</dt><dd data-echo="dialog-chosen">{move || dialog_chosen.get().map(|a| a.id).unwrap_or_else(|| "(none)".into())}</dd>
                <dt>"saves"</dt><dd data-echo="dialog-saves">{move || saves.get().to_string()}</dd>
            </dl>
            <Dialog
                open=dialog_open
                title="Move a split"
                on_close=move || dialog_open.set(false)
                affirm=DialogAction::new("Save", move || { saves.update(|n| *n += 1); dialog_open.set(false); })
            >
                <Field label="To account">
                    <Combobox
                        items=items
                        key_of=|a: &Account| a.id.clone()
                        label_of=|a: &Account| a.name.clone()
                        text=dialog_text
                        on_select=on_dialog_select
                        autofocus=true
                        name="dialog-combo"
                    />
                </Field>
            </Dialog>
        </Section>
    }
}
