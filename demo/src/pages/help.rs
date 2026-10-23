//! Help: `?` buttons opening topics (a paragraph, a list, a table, a guide
//! link), one out of the Tab order in a row of fields, and `?`s at the
//! viewport's edges, so placement can be checked.

use std::sync::LazyLock;

use ferrox_ui::{HelpButton, HelpTopic, Input};
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
