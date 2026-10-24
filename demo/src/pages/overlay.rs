//! Overlay: dialogs, the confirm dialog and sheets, each opened from a
//! button, with a log of what every way out did, so focus, Esc, Enter, the
//! footer and the scroll lock can be checked.

use std::time::Duration;

use ferrox_ui::{Button, ButtonFamily, ConfirmDialog, Dialog, DialogAction, DialogSize, Sheet, SheetSide};
use leptos::prelude::*;

use crate::section::Section;

const SAVING_MS: u64 = 800;

#[component]
pub fn OverlayPage() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
    view! {
        <Dialogs log=log />
        <Section title="Log">
            <ol class="demo-log">
                {move || log.get().into_iter().map(|entry| view! { <li>{entry}</li> }).collect_view()}
            </ol>
        </Section>
    }
}

#[component]
fn Dialogs(log: RwSignal<Vec<String>>) -> impl IntoView {
    let note = move |entry: &str| log.update(|l| l.push(entry.to_string()));
    view! {
        <Section title="Dialogs">
            <div class="demo-row">
                <FormDialog note=note />
                <WideDialog note=note />
                <NoDismissDialog note=note />
                <NestedDialogs note=note />
            </div>
        </Section>
        <Section title="Confirm dialog">
            <div class="demo-row">
                <DeleteConfirm note=note />
            </div>
        </Section>
        <Section title="Sheets">
            <div class="demo-row">
                <EdgeSheet side=SheetSide::Left label="Menu" note=note />
                <EdgeSheet side=SheetSide::Right label="Details" note=note />
                <EdgeSheet side=SheetSide::Bottom label="Actions" note=note />
            </div>
        </Section>
    }
}

/// A form: an `autofocus` field, a textarea (Enter is a newline there),
/// Save (saving, then closes) and Save as draft.
#[component]
fn FormDialog(note: impl Fn(&str) + Copy + 'static) -> impl IntoView {
    let open = RwSignal::new(false);
    let saving = RwSignal::new(false);
    let save = move || {
        saving.set(true);
        set_timeout(move || {
            saving.set(false);
            open.set(false);
            note("form: saved");
        }, Duration::from_millis(SAVING_MS));
    };
    let draft = move || {
        open.set(false);
        note("form: draft");
    };
    view! {
        <Button on_click=move || open.set(true)>"Edit account…"</Button>
        <Dialog
            open=open
            title="Edit account"
            on_close=move || { open.set(false); note("form: closed"); }
            affirm=DialogAction::new("Save", save).saving(saving)
            alternate=DialogAction::new("Save as draft", draft)
        >
            <label class="demo-field-stack">"Name" <input name="name" autofocus=true /></label>
            <label class="demo-field-stack">"Notes" <textarea name="notes"></textarea></label>
        </Dialog>
    }
}

/// Wide, for a body that is a table; no `autofocus`, so focus goes to ×.
#[component]
fn WideDialog(note: impl Fn(&str) + Copy + 'static) -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <Button family=ButtonFamily::Secondary on_click=move || open.set(true)>"Show splits…"</Button>
        <Dialog
            open=open
            title="Splits"
            size=DialogSize::Wide
            on_close=move || { open.set(false); note("wide: closed"); }
            affirm=DialogAction::new("Done", move || { open.set(false); note("wide: done"); })
        >
            <table class="demo-table">
                <thead><tr><th>"Account"</th><th>"Memo"</th><th>"Amount"</th></tr></thead>
                <tbody>
                    <tr><td>"Expenses:Groceries:Market"</td><td>"Weekly shop"</td><td>"84.20"</td></tr>
                    <tr><td>"Expenses:Household:Cleaning"</td><td>"Detergent"</td><td>"12.95"</td></tr>
                </tbody>
            </table>
        </Dialog>
    }
}

/// No dismiss button: one way to finish, though × and Esc still close.
#[component]
fn NoDismissDialog(note: impl Fn(&str) + Copy + 'static) -> impl IntoView {
    let open = RwSignal::new(false);
    view! {
        <Button family=ButtonFamily::Secondary on_click=move || open.set(true)>"Notice…"</Button>
        <Dialog
            open=open
            title="Saved"
            no_cancel=true
            on_close=move || { open.set(false); note("notice: closed"); }
            affirm=DialogAction::new("Got it", move || { open.set(false); note("notice: ok"); })
        >
            <p>"Your changes were saved."</p>
        </Dialog>
    }
}

/// A dialog opened from inside a dialog: the page stays locked until both close.
#[component]
fn NestedDialogs(note: impl Fn(&str) + Copy + 'static) -> impl IntoView {
    let outer = RwSignal::new(false);
    let inner = RwSignal::new(false);
    view! {
        <Button family=ButtonFamily::Secondary on_click=move || outer.set(true)>"Nested…"</Button>
        <Dialog
            open=outer
            title="Outer"
            on_close=move || { outer.set(false); note("outer: closed"); }
        >
            <Button on_click=move || inner.set(true)>"Open the inner dialog"</Button>
        </Dialog>
        <Dialog
            open=inner
            title="Inner"
            on_close=move || { inner.set(false); note("inner: closed"); }
            affirm=DialogAction::new("Done", move || { inner.set(false); note("inner: done"); })
        >
            <p>"A second modal on top of the first."</p>
        </Dialog>
    }
}

/// The destructive confirmation: Delete runs for a moment (saving), then the
/// dialog closes.
#[component]
fn DeleteConfirm(note: impl Fn(&str) + Copy + 'static) -> impl IntoView {
    let open = RwSignal::new(false);
    let deleting = RwSignal::new(false);
    let delete = move || {
        note("confirm: confirmed");
        deleting.set(true);
        set_timeout(move || {
            deleting.set(false);
            open.set(false);
            note("confirm: deleted");
        }, Duration::from_millis(SAVING_MS));
    };
    view! {
        <Button family=ButtonFamily::Danger on_click=move || open.set(true)>"Delete account…"</Button>
        <ConfirmDialog
            open=open
            title="Delete account?"
            confirm_label="Delete"
            saving=deleting
            on_confirm=delete
            on_cancel=move || { open.set(false); note("confirm: cancelled"); }
        >
            <p>"This deletes “Groceries” and its 214 transactions."</p>
        </ConfirmDialog>
    }
}

/// A sheet at one edge, holding a few controls; its log entries name the edge.
#[component]
fn EdgeSheet(side: SheetSide, label: &'static str, note: impl Fn(&str) + Copy + Send + Sync + 'static) -> impl IntoView {
    let open = RwSignal::new(false);
    let name = label.to_lowercase();
    let closed = format!("{name}: closed");
    let chosen = StoredValue::new(format!("{name}: chose"));
    view! {
        <Button family=ButtonFamily::Secondary on_click=move || open.set(true)>{format!("{label}…")}</Button>
        <Sheet open=open side=side title=label on_close=move || { open.set(false); note(&closed); }>
            <div class="demo-stack">
                <Button quiet=true on_click=move || { open.set(false); note(&chosen.get_value()); }>"Accounts"</Button>
                <Button quiet=true>"Budgets"</Button>
                <Button quiet=true>"Reports"</Button>
            </div>
        </Sheet>
    }
}
