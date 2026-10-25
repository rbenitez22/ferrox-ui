//! Help: `?` buttons opening topics (a paragraph, a list, a table, a guide
//! link), one out of the Tab order in a row of fields, a form whose `?`s
//! show only while their control is in use (F1 opens the focused control's
//! help, Shift+F1 asks: the page is inside `HelpKeys`), a log of the help
//! signals the page reports (it is inside `HelpSignals`), `?`s at the
//! viewport's edges for placement, and a section with a fake assistant that
//! shows the request "Ask about this" sends.

use std::sync::LazyLock;

use ferrox_ui::{help_signals, 
    Button, ButtonFamily, Checkbox, Combobox, ConfirmDialog, Dialog, DialogAction, Field, HelpAssistant, HelpButton, HelpKeys, HelpRequest, HelpReveal,
    HelpScope, HelpSignal, HelpSignals, HelpTopic, Input, PasswordInput, Sheet, SheetSide, Switch, Toolbar,
};
use leptos::prelude::*;

use crate::section::Section;

static STATEMENT_BALANCE: LazyLock<HelpTopic> = LazyLock::new(|| {
    HelpTopic::new("statement-balance", "Statement balance")
        .paragraph("The closing balance printed on the bank's statement for this period.")
        .paragraph("Reconciling ticks off entries until the account agrees with it.")
});

static SPLIT_AMOUNT: LazyLock<HelpTopic> = LazyLock::new(|| {
    HelpTopic::new("split-amount", "Amounts on a split")
        .paragraph("Each line moves money into or out of one account.")
        .list(["Positive: money comes in", "Negative: money goes out", "The lines of an entry add up to zero"])
});

static ACCOUNT_KINDS: LazyLock<HelpTopic> = LazyLock::new(|| {
    HelpTopic::new("account-kinds", "The five kinds of account")
        .table(
            ["Kind", "Holds", "Grows with"],
            [
                ["Asset", "What you own", "Debits"],
                ["Liability", "What you owe", "Credits"],
                ["Equity", "What is left", "Credits"],
                ["Income", "Money earned", "Credits"],
                ["Expense", "Money spent", "Debits"],
            ],
        )
        .guide("#account-kinds")
});

#[component]
pub fn HelpPage() -> impl IntoView {
    let log = RwSignal::new(Vec::<HelpSignal>::new());
    view! {
        <HelpSignals on_signal=move |signal: HelpSignal| log.update(|l| l.push(signal))>
            <HelpKeys>
                <Topics />
                <Asking />
                <OnComponents />
                <RowOfFields />
                <WhileActive />
                <Signals log=log />
                <Edges />
            </HelpKeys>
        </HelpSignals>
    }
}

#[component]
fn Topics() -> impl IntoView {
    view! {
        <Section title="Help topics">
            <div class="demo-row">
                <span class="demo-help-label">"Statement balance"</span>
                <HelpButton topic=STATEMENT_BALANCE.clone() />
            </div>
            <div class="demo-row">
                <span class="demo-help-label">"Amount"</span>
                <HelpButton topic=SPLIT_AMOUNT.clone() />
            </div>
            <div class="demo-row">
                <span class="demo-help-label">"Kind"</span>
                <HelpButton topic=ACCOUNT_KINDS.clone() />
            </div>
        </Section>
    }
}

/// A `?` in a row of fields stays out of the Tab order (`skip_tab`), so Tab
/// goes field to field.
#[component]
fn RowOfFields() -> impl IntoView {
    let (memo, amount) = (RwSignal::new(String::new()), RwSignal::new(String::new()));
    view! {
        <Section title="Row of fields">
            <div class="demo-row">
                <Input value=memo name="memo" placeholder="Memo" />
                <Input value=amount name="amount" placeholder="Amount" />
                <HelpButton topic=SPLIT_AMOUNT.clone() skip_tab=true />
            </div>
        </Section>
    }
}

/// A form whose `?`s show only while their control is in use: the scope
/// sets it for everything inside; one field keeps its `?` always shown.
#[component]
fn WhileActive() -> impl IntoView {
    let (reference, amount, category) = (RwSignal::new(String::new()), RwSignal::new(String::new()), RwSignal::new(String::new()));
    let (remind, reviewed) = (RwSignal::new(false), RwSignal::new(false));
    view! {
        <Section title="Only while in use">
            <HelpScope reveal=HelpReveal::WhileActive>
                <Field label="Reference" help=about("reference", "Reference numbers")>
                    <Input value=reference />
                </Field>
                <Field label="Amount" help=SPLIT_AMOUNT.clone()>
                    <Input value=amount />
                </Field>
                <Field label="Category" help=about("category", "Categories") help_reveal=HelpReveal::Always>
                    <Input value=category />
                </Field>
                <Checkbox checked=remind label="Send a reminder" help=about("reminders", "Reminders") />
                <Switch checked=reviewed label="Reviewed" help=about("reviewed", "Reviewed entries") />
            </HelpScope>
        </Section>
    }
}

/// What `HelpSignals` received, newest last, and a button that reports the
/// app's own signal from inside a scope.
#[component]
fn Signals(log: RwSignal<Vec<HelpSignal>>) -> impl IntoView {
    view! {
        <Section title="Help signals">
            <HelpScope context=[("screen", "signals")]>
                <ReportImbalance />
                <Suggest />
                <SignalSources />
            </HelpScope>
            <Button family=ButtonFamily::Secondary on_click=move || log.set(Vec::new())>"Clear the log"</Button>
            <ol class="demo-signals">
                {move || log.get().into_iter().map(|s| {
                    let context = s.context.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join(" ");
                    view! {
                        <li
                            data-signal=s.kind.name().to_string()
                            data-kind=format!("{:?}", s.kind)
                            data-component=s.component.clone()
                            data-about=s.about.clone()
                            data-topic=s.topic_id.clone().unwrap_or_default()
                            data-detail=s.detail.clone().unwrap_or_default()
                            data-context=context.clone()
                        >
                            {format!("{} · {} {} {}", s.kind.name(), s.component, s.about, context)}
                        </li>
                    }
                }).collect_view()}
            </ol>
        </Section>
    }
}

/// Components that report: a field whose error persists, a combobox left
/// with no match, a dialog whose Pay is pressed again while paying.
#[component]
fn SignalSources() -> impl IntoView {
    let total = RwSignal::new(String::new());
    let total_error = Signal::derive(move || {
        let text = total.get();
        (!text.is_empty() && text.parse::<f64>().is_err()).then(|| "A number, like 12.50".to_string())
    });
    let category = RwSignal::new(String::new());
    let categories = Signal::stored(vec!["Groceries".to_string(), "Rent".to_string()]);
    let (paying_open, paying) = (RwSignal::new(false), RwSignal::new(false));
    let memo = RwSignal::new(String::new());
    let ignore_choice = |_: Option<String>| ();
    view! {
        <div class="demo-form">
            <Field label="Total" error=total_error help=about("total", "Totals")>
                <Input value=total name="signal-total" />
            </Field>
            <Field label="Category" help=about("category-signal", "Choosing a category")>
                <Combobox
                    items=categories
                    key_of=|c: &String| c.clone()
                    label_of=|c: &String| c.clone()
                    text=category
                    on_select=ignore_choice
                />
            </Field>
        </div>
        <Button family=ButtonFamily::Secondary on_click=move || { paying.set(false); paying_open.set(true) }>"Pay…"</Button>
        <Dialog
            open=paying_open
            title="Pay the bill"
            help=about("paying", "Paying a bill")
            affirm=DialogAction::new("Pay", move || paying.set(true)).saving(paying)
        >
            <Field label="Note">
                <Input value=memo name="signal-note" />
            </Field>
        </Dialog>
    }
}

/// What a policy does when it decides to offer help: suggest a topic (its
/// `?`s wear a ring until opened), or take the suggestion back.
#[component]
fn Suggest() -> impl IntoView {
    let signals = help_signals();
    view! {
        <div class="demo-row">
            <Button family=ButtonFamily::Secondary on_click=move || signals.suggest("total")>"Suggest help on Total"</Button>
            <Button family=ButtonFamily::Secondary on_click=move || signals.suggest("reference")>"Suggest help on Reference"</Button>
            <Button family=ButtonFamily::Secondary on_click=move || signals.clear("total")>"Take it back"</Button>
        </div>
    }
}

/// The app's own signal: a check the app knows failed.
#[component]
fn ReportImbalance() -> impl IntoView {
    let signals = help_signals();
    view! {
        <Button family=ButtonFamily::Secondary on_click=move || signals.report("imbalance", Some("0.50".into()))>
            "Report an imbalance"
        </Button>
    }
}

/// `?`s against the right edge and at the bottom of a tall section, for
/// placement: kept inside the viewport, above when there is no room below.
#[component]
fn Edges() -> impl IntoView {
    view! {
        <Section title="Edges">
            <div class="demo-row demo-row--end">
                <span class="demo-help-label">"At the right edge"</span>
                <HelpButton topic=ACCOUNT_KINDS.clone() class="demo-help-right" />
            </div>
            <div class="demo-tall"></div>
            <div class="demo-row">
                <span class="demo-help-label">"At the bottom"</span>
                <HelpButton topic=SPLIT_AMOUNT.clone() class="demo-help-bottom" />
            </div>
        </Section>
    }
}

/// Inside a `HelpAssistant`, popovers offer "Ask about this". Scopes add
/// the screen and the (changing) account; one `?` sends its field's value.
/// The fake assistant opens a sheet showing the request it got.
#[component]
fn Asking() -> impl IntoView {
    let asked = RwSignal::new(None::<HelpRequest>);
    let assistant_open = RwSignal::new(false);
    let account = RwSignal::new("checking".to_string());
    let amount = RwSignal::new(String::new());
    let on_ask = move |request: HelpRequest| {
        asked.set(Some(request));
        assistant_open.set(true);
    };
    let next_account = move || account.update(|a| *a = if a == "checking" { "savings".into() } else { "checking".into() });
    view! {
        <Section title="Asking an assistant">
            <HelpAssistant on_ask=on_ask>
                <HelpScope context=[("screen", "reconcile")]>
                    <HelpScope context=move || vec![("account".to_string(), account.get())]>
                        <div class="demo-row">
                            <span class="demo-help-label">"Statement balance"</span>
                            <HelpButton topic=STATEMENT_BALANCE.clone() kind="field" about="Statement balance" />
                        </div>
                        <div class="demo-row">
                            <Input value=amount name="shared-amount" placeholder="Amount (sent with the request)" />
                            <HelpButton topic=SPLIT_AMOUNT.clone() kind="field" about="Amount" value=Signal::derive(move || amount.get()) />
                        </div>
                    </HelpScope>
                    <HelpScope context=[("section", "sibling")]>
                        <p class="demo-note">"A sibling scope: its context stays inside it."</p>
                    </HelpScope>
                    <div class="demo-row">
                        <span class="demo-help-label">"Kinds, outside the account scope"</span>
                        <HelpButton topic=ACCOUNT_KINDS.clone() />
                    </div>
                </HelpScope>
            </HelpAssistant>
            <div class="demo-row">
                <Button family=ButtonFamily::Secondary on_click=next_account>"Next account"</Button>
                <span class="demo-note" data-echo="account">{move || account.get()}</span>
            </div>
            <FakeAssistant asked=asked open=assistant_open />
        </Section>
    }
}

/// The fake assistant: a sheet showing the request it was handed.
#[component]
fn FakeAssistant(asked: RwSignal<Option<HelpRequest>>, open: RwSignal<bool>) -> impl IntoView {
    view! {
        <Sheet open=open side=SheetSide::Right title="Assistant">
            {move || asked.get().map(|request| view! {
                <dl class="demo-echo">
                    <dt>"topic"</dt><dd data-request="topic">{request.topic_id.clone()}</dd>
                    <dt>"component"</dt><dd data-request="component">{request.component}</dd>
                    <dt>"label"</dt><dd data-request="label">{request.label.clone()}</dd>
                    <dt>"value"</dt><dd data-request="value">{request.value.clone().unwrap_or_else(|| "(none)".into())}</dd>
                </dl>
                <pre class="demo-prompt" data-request="prompt">{request.to_prompt_context()}</pre>
            })}
        </Sheet>
    }
}

/// A one-paragraph topic, for the components below.
fn about(id: &str, title: &str) -> HelpTopic {
    HelpTopic::new(id, title).paragraph(format!("What {title} means here."))
}

/// Every component's `help` prop, inside an assistant: where each `?` sits,
/// and what its request says.
#[component]
fn OnComponents() -> impl IntoView {
    let asked = RwSignal::new(None::<HelpRequest>);
    let assistant_open = RwSignal::new(false);
    let on_ask = move |request: HelpRequest| {
        asked.set(Some(request));
        assistant_open.set(true);
    };
    let (memo, payee, password) = (RwSignal::new(String::new()), RwSignal::new(String::new()), RwSignal::new(String::new()));
    let split = RwSignal::new(String::new());
    let (repeat, archived) = (RwSignal::new(false), RwSignal::new(false));
    let (editing, deleting, menu) = (RwSignal::new(false), RwSignal::new(false), RwSignal::new(false));
    view! {
        <Section title="Help on components">
            <HelpAssistant on_ask=on_ask>
                <HelpScope context=[("screen", "components")]>
                    <Toolbar title="Accounts" help=about("accounts", "Accounts")>
                        <Button>"New account"</Button>
                    </Toolbar>
                    <div class="demo-form">
                        <Field label="Memo" help=about("memo", "Memo") help_shares_value=true>
                            <Input value=memo name="help-memo" />
                        </Field>
                        <Field label="Payee" help=about("payee", "Payee")>
                            <Input value=payee name="help-payee" />
                        </Field>
                        <Field label="Password" help=about("password", "Password") help_shares_value=true>
                            <PasswordInput value=password name="help-password" />
                        </Field>
                        <Checkbox checked=repeat label="Repeat every month" help=about("repeat", "Repeating") help_shares_value=true />
                        <div class="demo-row">
                            <Switch checked=archived label="Show archived" help=about("archived", "Archived accounts") help_skip_tab=true />
                        </div>
                    </div>
                    <div class="demo-row">
                        <Button family=ButtonFamily::Secondary on_click=move || editing.set(true)>"Dialog with help…"</Button>
                        <Button family=ButtonFamily::Danger on_click=move || deleting.set(true)>"Confirm with help…"</Button>
                        <Button family=ButtonFamily::Secondary on_click=move || menu.set(true)>"Sheet with help…"</Button>
                    </div>
                    <Dialog open=editing title="Edit split" help=about("edit-split", "Editing a split")>
                        <p>"A dialog's ? sits in its title bar, before the ×."</p>
                        <Field label="Split amount" help=about("split-line", "Split lines")>
                            <Input value=split name="help-split" />
                        </Field>
                    </Dialog>
                    <ConfirmDialog
                        open=deleting
                        title="Delete split?"
                        confirm_label="Delete"
                        help=about("delete-split", "Deleting a split")
                        on_confirm=move || deleting.set(false)
                    >
                        <p>"This deletes the split."</p>
                    </ConfirmDialog>
                    <Sheet open=menu title="Menu" help=about("menu", "The menu")>
                        <p>"A sheet's ? sits in its title bar too."</p>
                    </Sheet>
                </HelpScope>
            </HelpAssistant>
            <FakeAssistant asked=asked open=assistant_open />
        </Section>
    }
}
