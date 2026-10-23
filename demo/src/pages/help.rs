//! Help: `?` buttons opening topics (a paragraph, a list, a table, a guide
//! link), one out of the Tab order in a row of fields, `?`s at the
//! viewport's edges for placement, and a section with a fake assistant that
//! shows the request "Ask about this" sends.

use std::sync::LazyLock;

use ferrox_ui::{Button, ButtonFamily, HelpAssistant, HelpButton, HelpRequest, HelpScope, HelpTopic, Input, Sheet, SheetSide};
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
    view! {
        <Topics />
        <Asking />
        <RowOfFields />
        <Edges />
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
    let next_account = move |_| account.update(|a| *a = if a == "checking" { "savings".into() } else { "checking".into() });
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
            <Sheet open=assistant_open side=SheetSide::Right title="Assistant" on_close=move || assistant_open.set(false)>
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
        </Section>
    }
}
