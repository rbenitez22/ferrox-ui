//! The docs' examples, compiled. Each `// region: <name>` … `// endregion`
//! is what `docs/` shows: `tests/docs_current.rs` copies it into the page's
//! `<!-- example: <name> -->` block and fails when a page's copy differs.
//! What the app supplies itself (its functions, its data) is declared just
//! outside a region, so the region shows only what an app writes. There is
//! no crate-wide `use ferrox_ui::*` here: a region names every import it
//! needs, or it doesn't compile. Examples assume `use leptos::prelude::*;`.

#![allow(dead_code, unused_imports, clippy::all)]

use leptos::prelude::*;

/// A record the examples list.
#[derive(Clone, Debug, PartialEq)]
struct Recipe {
    id: String,
    name: String,
}

fn recipes() -> Signal<Vec<Recipe>> {
    Signal::stored(Vec::new())
}

fn noop() {}

// ---------------------------------------------------------------------------
// core
// ---------------------------------------------------------------------------

mod core {
    use super::*;

    fn button() -> impl IntoView {
        let (save, close) = (noop, noop);
        let saving = RwSignal::new(false);
        // region: button
        use ferrox_ui::{Button, ButtonFamily, ButtonSize};

        view! {
            <Button on_click=move || save()>"Save"</Button>
            <Button family=ButtonFamily::Cancel on_click=move || close()>"Cancel"</Button>
            <Button family=ButtonFamily::Secondary quiet=true size=ButtonSize::Sm>"Export"</Button>
            <Button saving=saving button_type="submit">"Create"</Button>
        }
        // endregion
    }

    fn icon_button() -> impl IntoView {
        let (open_search, remove) = (noop, noop);
        let camera = RwSignal::new(false);
        // region: icon-button
        use ferrox_icons::{Icon, CAMERA, DELETE, SEARCH};
        use ferrox_ui::IconButton;

        view! {
            <IconButton label="Search" on_click=move || open_search()><Icon icon=SEARCH /></IconButton>
            <IconButton label="Camera" pressed=camera on_click=move || camera.update(|c| *c = !*c)><Icon icon=CAMERA /></IconButton>
            <IconButton label="Remove line" danger=true on_click=move || remove()><Icon icon=DELETE /></IconButton>
        }
        // endregion
    }

    fn toolbar() -> impl IntoView {
        let add = noop;
        let camera = RwSignal::new(false);
        // region: toolbar
        use ferrox_icons::{Icon, CAMERA, HELP, SEARCH};
        use ferrox_ui::{Button, IconButton, Toolbar, ToolbarGroup};

        view! {
            <Toolbar title="Accounts">
                <Button on_click=move || add()>"New account"</Button>
            </Toolbar>

            <Toolbar compact=true>
                <ToolbarGroup>
                    <IconButton label="Search"><Icon icon=SEARCH /></IconButton>
                    <IconButton label="Camera" pressed=camera><Icon icon=CAMERA /></IconButton>
                </ToolbarGroup>
                <ToolbarGroup>
                    <IconButton label="Help"><Icon icon=HELP /></IconButton>
                </ToolbarGroup>
            </Toolbar>
        }
        // endregion
    }

    fn spinner() -> impl IntoView {
        let signing_in = RwSignal::new(false);
        // region: spinner
        use ferrox_ui::{LoadingOverlay, Spinner, SpinnerKind};

        view! {
            <Spinner label="Loading accounts" />
            <Spinner kind=SpinnerKind::Inline label="Deleting the row" />
            {move || signing_in.get().then(|| view! { <LoadingOverlay label="Signing in…" /> })}
        }
        // endregion
    }

    fn alert() -> impl IntoView {
        let error = RwSignal::new(None::<String>);
        // region: alert
        use ferrox_ui::{Alert, AlertSeverity};

        view! {
            <Alert severity=AlertSeverity::Error>"The server refused the change: the period is closed."</Alert>
            <Alert severity=AlertSeverity::Info icon=false>"Changes apply to new entries only."</Alert>
            {move || error.get().map(|e| view! {
                <Alert severity=AlertSeverity::Error on_close=move || error.set(None)>{e}</Alert>
            })}
        }
        // endregion
    }

    // region: help-topic
    use std::sync::LazyLock;

    use ferrox_ui::HelpTopic;

    static SPLIT_AMOUNT: LazyLock<HelpTopic> = LazyLock::new(|| {
        HelpTopic::new("split-amount", "Amounts on a split") // id, title
            .paragraph("Each line moves money into or out of one account.")
            .list(["Positive: money comes in", "Negative: money goes out"])
            .table(["Sign", "Means"], [["+", "in"], ["-", "out"]])
            .guide("/help#split-amount") // "Full guide"
    });
    // endregion

    static STATEMENT_BALANCE: LazyLock<HelpTopic> = LazyLock::new(|| HelpTopic::new("statement-balance", "Statement balance"));
    static PAYEE: LazyLock<HelpTopic> = LazyLock::new(|| HelpTopic::new("payee", "Payee"));
    static CATEGORY: LazyLock<HelpTopic> = LazyLock::new(|| HelpTopic::new("category", "Category"));

    #[cfg(feature = "forms")]
    fn help_prop() -> impl IntoView {
        let amount = RwSignal::new(String::new());
        // region: help-prop
        use ferrox_ui::{Field, Input};

        view! {
            <Field label="Amount" help=SPLIT_AMOUNT.clone()>
                <Input value=amount />
            </Field>
        }
        // endregion
    }

    #[cfg(feature = "forms")]
    fn help_reveal() -> impl IntoView {
        let (payee, category) = (RwSignal::new(String::new()), RwSignal::new(String::new()));
        // region: help-reveal
        use ferrox_ui::{Field, HelpReveal, HelpScope, Input};

        view! {
            <HelpScope reveal=HelpReveal::WhileActive>
                <Field label="Payee" help=PAYEE.clone()>
                    <Input value=payee />
                </Field>
                <Field label="Category" help=CATEGORY.clone() help_reveal=HelpReveal::Always>
                    <Input value=category />
                </Field>
            </HelpScope>
        }
        // endregion
    }

    fn help_button() -> impl IntoView {
        // region: help-button
        use ferrox_ui::HelpButton;

        view! {
            <h2>"Statement balance" <HelpButton topic=STATEMENT_BALANCE.clone() /></h2>
        }
        // endregion
    }

    #[cfg(feature = "forms")]
    fn help_scope() -> impl IntoView {
        let account = RwSignal::new(String::from("checking"));
        let balance = RwSignal::new(String::new());
        // region: help-scope
        use ferrox_ui::{Field, HelpScope, Input};

        view! {
            <HelpScope context=[("domain", "bookkeeping")]>
                <HelpScope context=[("screen", "reconcile")]>
                    <HelpScope context=move || vec![
                        ("entity".to_string(), "account".to_string()),
                        ("account_id".to_string(), account.get()),
                    ]>
                        <Field label="Statement balance" help=STATEMENT_BALANCE.clone()>
                            <Input value=balance />
                        </Field>
                    </HelpScope>
                </HelpScope>
            </HelpScope>
        }
        // endregion
    }

    fn help_assistant() -> impl IntoView {
        let ask_assistant = |_request: ferrox_ui::HelpRequest| {};
        // region: help-assistant
        use ferrox_ui::{HelpAssistant, HelpRequest};

        view! {
            <HelpAssistant on_ask=move |request: HelpRequest| ask_assistant(request)>
                <p>"The app: every help popover inside offers \"Ask about this\"."</p>
            </HelpAssistant>
        }
        // endregion
    }

    fn help_keys() -> impl IntoView {
        // region: help-keys
        use ferrox_ui::HelpKeys;

        view! {
            <HelpKeys>
                <p>"The app: F1 opens the focused control's help, Shift+F1 asks about it."</p>
            </HelpKeys>
        }
        // endregion
    }

    fn help_signals_observer() -> impl IntoView {
        let observe = |_signal: ferrox_ui::HelpSignal| {};
        // region: help-signals
        use ferrox_ui::{HelpSignal, HelpSignals};

        view! {
            <HelpSignals on_signal=move |signal: HelpSignal| observe(signal)>
                <p>"The app: what its components report arrives in on_signal."</p>
            </HelpSignals>
        }
        // endregion
    }

    fn help_signals_report() -> impl IntoView {
        let difference = || String::from("0.50");
        // region: help-signals-report
        use ferrox_ui::{help_signals, Button};

        let signals = help_signals();
        view! {
            <Button on_click=move || signals.report("imbalance", Some(difference()))>"Check"</Button>
        }
        // endregion
    }

    fn help_signals_suggest() -> impl IntoView {
        // region: help-signals-suggest
        use ferrox_ui::{HelpSignal, HelpSignalKind, HelpSignals, HelpSuggestions};

        // A policy: the third failed fix of the same field suggests its help.
        let suggestions = HelpSuggestions::new();
        let policy = move |signal: HelpSignal| {
            if let (HelpSignalKind::ErrorPersisted { edits: 3.. }, Some(topic)) = (&signal.kind, &signal.topic_id) {
                suggestions.suggest(topic.clone());
            }
        };
        view! {
            <HelpSignals on_signal=policy suggestions=suggestions>
                <p>"The app."</p>
            </HelpSignals>
        }
        // endregion
    }

    fn help_keys_bare_button() -> impl IntoView {
        // region: help-keys-bare
        use ferrox_ui::HelpButton;

        view! {
            <div data-help="balance-help">
                <input aria-label="Statement balance" />
                <HelpButton topic=STATEMENT_BALANCE.clone() id="balance-help" />
            </div>
        }
        // endregion
    }
}

// ---------------------------------------------------------------------------
// overlay
// ---------------------------------------------------------------------------

#[cfg(feature = "overlay")]
mod overlay {
    use super::*;

    fn dialog() -> impl IntoView {
        let (save, save_draft) = (noop, noop);
        // region: dialog
        use ferrox_ui::{Button, Dialog, DialogAction};

        let editing = RwSignal::new(false);
        let saving = RwSignal::new(false);

        view! {
            <Button on_click=move || editing.set(true)>"Edit account…"</Button>
            <Dialog
                open=editing
                title="Edit account"
                affirm=DialogAction::new("Save", move || save()).saving(saving)
                alternate=DialogAction::new("Save as draft", move || save_draft())
            >
                <label>"Name" <input autofocus=true /></label>
            </Dialog>
        }
        // endregion
    }

    fn confirm_dialog() -> impl IntoView {
        let delete_account = noop;
        let confirming = RwSignal::new(false);
        let deleting = RwSignal::new(false);
        // region: confirm-dialog
        use ferrox_ui::ConfirmDialog;

        view! {
            <ConfirmDialog
                open=confirming
                title="Delete account?"
                confirm_label="Delete"
                saving=deleting
                on_confirm=move || delete_account() // set `deleting`, then `confirming` false when done
            >
                <p>"This deletes “Groceries” and its 214 transactions."</p>
            </ConfirmDialog>
        }
        // endregion
    }

    fn sheet() -> impl IntoView {
        let menu = RwSignal::new(false);
        // region: sheet
        use ferrox_icons::{Icon, MENU};
        use ferrox_ui::{IconButton, Sheet};

        view! {
            <IconButton label="Menu" on_click=move || menu.set(true)><Icon icon=MENU /></IconButton>
            <Sheet open=menu title="Menu">
                <nav>"The app's links"</nav>
            </Sheet>
        }
        // endregion
    }
}

// ---------------------------------------------------------------------------
// forms
// ---------------------------------------------------------------------------

#[cfg(feature = "forms")]
mod forms {
    use std::sync::LazyLock;

    use ferrox_ui::HelpTopic;

    use super::*;

    static NAME_HELP: LazyLock<HelpTopic> = LazyLock::new(|| HelpTopic::new("name", "Name"));

    fn forms_intro() -> impl IntoView {
        let refused = RwSignal::new(None::<String>);
        // region: forms
        use ferrox_ui::{Alert, AlertSeverity, Field, Input, InputKind};

        let email = RwSignal::new(String::new());
        let email_error = Signal::derive(move || {
            let e = email.get();
            (!e.is_empty() && !e.contains('@')).then(|| "Enter an email address, like name@example.com".to_string())
        });

        view! {
            {move || refused.get().map(|why| view! { <Alert severity=AlertSeverity::Error>{why}</Alert> })}
            <Field label="Email" hint="Where receipts go" error=email_error>
                <Input value=email kind=InputKind::Email autocomplete="email" />
            </Field>
        }
        // endregion
    }

    fn field() -> impl IntoView {
        let name = RwSignal::new(String::new());
        let name_error = Signal::derive(move || name.get().is_empty().then(|| "Enter a name".to_string()));
        // region: field
        use ferrox_ui::{Field, Input};

        view! {
            <Field label="Name" hint="As it appears on statements" error=name_error help=NAME_HELP.clone()>
                <Input value=name autofocus=true />
            </Field>
        }
        // endregion
    }

    fn input() -> impl IntoView {
        let (name, query) = (RwSignal::new(String::new()), RwSignal::new(String::new()));
        // region: input
        use ferrox_ui::{Field, Input, InputKind};

        view! {
            <Field label="Name"><Input value=name autofocus=true /></Field>
            <Input value=query kind=InputKind::Search placeholder="Search accounts" />
        }
        // endregion
    }

    fn textarea() -> impl IntoView {
        let notes = RwSignal::new(String::new());
        // region: textarea
        use ferrox_ui::{Field, Textarea};

        view! { <Field label="Notes"><Textarea value=notes rows=6 /></Field> }
        // endregion
    }

    fn password_input() -> impl IntoView {
        let (password, new_password) = (RwSignal::new(String::new()), RwSignal::new(String::new()));
        // region: password-input
        use ferrox_ui::{Field, PasswordInput};

        view! {
            <Field label="Password"><PasswordInput value=password /></Field>

            <Field label="New password" hint="At least four words, or a long mix">
                <PasswordInput value=new_password autocomplete="new-password" strength=true min_bits=70.0 />
            </Field>
        }
        // endregion
    }

    fn password_strength_check() {
        let new_password = RwSignal::new(String::new());
        // region: password-strength
        use ferrox_ui::{password_strength, StrengthLevel};

        let strength = password_strength(&new_password.get(), 80.0); // PasswordStrength { bits, level }
        if strength.level != StrengthLevel::Strong { /* say so under the field */ }
        // endregion
    }

    fn checkbox() -> impl IntoView {
        let (recurring, agreed) = (RwSignal::new(false), RwSignal::new(false));
        let terms_error = Signal::derive(move || (!agreed.get()).then(|| "Tick this to continue.".to_string()));
        // region: checkbox
        use ferrox_ui::Checkbox;

        view! {
            <Checkbox checked=recurring label="Repeat every month" hint="On the same day each month" />
            <Checkbox checked=agreed label="I accept the terms" error=terms_error required=true />
        }
        // endregion
    }

    fn switch() -> impl IntoView {
        let show_archived = RwSignal::new(false);
        // region: switch
        use ferrox_ui::Switch;

        view! { <Switch checked=show_archived label="Show archived accounts" /> }
        // endregion
    }

    fn select() -> impl IntoView {
        let (currency, category) = (RwSignal::new(String::new()), RwSignal::new(String::new()));
        let categories = Signal::stored(Vec::<ferrox_ui::SelectOption>::new());
        let (categories_loading, categories_refreshing) = (RwSignal::new(false), RwSignal::new(false));
        let new_category_dialog = RwSignal::new(false);
        let reload_categories = noop;
        // region: select
        use ferrox_ui::{Field, Select, SelectOption};

        let currencies = Signal::derive(|| vec![
            SelectOption::new("EUR", "Euro"),
            SelectOption::new("USD", "US dollar"),
        ]);

        view! {
            <Field label="Currency">
                <Select value=currency options=currencies placeholder="Choose a currency" />
            </Field>

            <Field label="Category">
                <Select
                    value=category
                    options=categories
                    loading=categories_loading
                    on_refresh=move || reload_categories()
                    refreshing=categories_refreshing
                    on_create=move || new_category_dialog.set(true)
                    create_label="New category…"
                />
            </Field>
        }
        // endregion
    }

    #[derive(Clone)]
    struct Account {
        id: String,
        name: String,
    }

    fn combobox() -> impl IntoView {
        let accounts = Signal::stored(Vec::<Account>::new());
        let account_error = Signal::stored(None::<String>);
        let accounts_refreshing = RwSignal::new(false);
        let reload_accounts = noop;
        let create_account = |_name: String| {};
        // region: combobox
        use ferrox_ui::{Combobox, Field};

        // `accounts: Signal<Vec<Account>>` is loaded by the app.
        let account_text = RwSignal::new(String::new());
        let chosen = RwSignal::new(None::<Account>);

        view! {
            <Field label="Account" error=account_error>
                <Combobox
                    items=accounts
                    key_of=|a: &Account| a.id.clone()
                    label_of=|a: &Account| a.name.clone()
                    text=account_text
                    on_select=move |a| chosen.set(a)
                    placeholder="Type to search"
                    on_refresh=move || reload_accounts()
                    refreshing=accounts_refreshing
                    on_create=move |name: String| create_account(name)
                />
            </Field>
        }
        // endregion
    }
}

// ---------------------------------------------------------------------------
// navigation
// ---------------------------------------------------------------------------

#[cfg(feature = "navigation")]
mod navigation {
    use std::sync::LazyLock;

    use ferrox_ui::HelpTopic;

    use super::*;

    static MENU_HELP: LazyLock<HelpTopic> = LazyLock::new(|| HelpTopic::new("menu", "The menu"));

    fn navigation_intro() -> impl IntoView {
        // region: navigation
        use ferrox_ui::{AppShell, NavGroup, NavLink};
        use leptos_router::components::{Route, Router, Routes};
        use leptos_router::path;

        view! {
            <Router>
                <AppShell
                    title="Groceries"
                    href="/"
                    nav=|| view! {
                        <NavLink href="/" exact=true>"Lists"</NavLink>
                        <NavLink href="/load">"Load Data"</NavLink>
                        <NavGroup label="Product database" collapsible=true>
                            <NavLink href="/units">"Units"</NavLink>
                            <NavLink href="/categories">"Categories"</NavLink>
                            <NavLink href="/products">"Products"</NavLink>
                        </NavGroup>
                        <NavGroup label="Account">
                            <NavLink href="/account">"Account"</NavLink>
                            <NavLink href="/logout">"Logout"</NavLink>
                        </NavGroup>
                    }
                >
                    <Routes fallback=|| "Not found">
                        <Route path=path!("/") view=|| "Lists" />
                    </Routes>
                </AppShell>
            </Router>
        }
        // endregion
    }

    fn app_shell() -> impl IntoView {
        // region: app-shell
        use ferrox_ui::{AppShell, Menu, MenuItem, MenuTrigger, NavLink};
        use leptos_router::components::Outlet;

        view! {
            <AppShell
                title="Cookbook"
                href="/"
                nav=|| view! {
                    <NavLink href="/" exact=true>"Recipes"</NavLink>
                    <NavLink href="/settings">"Settings"</NavLink>
                }
                bar_end=|| view! {
                    <Menu label="Account" trigger=MenuTrigger::custom(|| view! { <span>"Ada Lovelace"</span> }, "Account")>
                        <MenuItem on_select=|| ()>"Sign out"</MenuItem>
                    </Menu>
                }
                menu_footer=|| view! { <span>"Version 1.4"</span> }
            >
                <Outlet />
            </AppShell>
        }
        // endregion
    }

    fn nav_list() -> impl IntoView {
        // region: nav-list
        use ferrox_icons::SEARCH;
        use ferrox_ui::{NavLink, NavList};

        view! {
            <NavList label="Settings">
                <NavLink href="/settings" exact=true>"General"</NavLink>
                <NavLink href="/settings/search" icon=SEARCH>"Search"</NavLink>
            </NavList>
        }
        // endregion
    }

    fn nav_group() -> impl IntoView {
        // region: nav-group
        use ferrox_ui::{NavGroup, NavLink};

        let remember = RwSignal::new(false); // store it wherever the app keeps preferences

        view! {
            <NavGroup label="Account"><NavLink href="/account">"Account"</NavLink></NavGroup> // plain
            <NavGroup label="Product database" collapsible=true> // starts collapsed
                <NavLink href="/units">"Units"</NavLink>
            </NavGroup>
            <NavGroup label="Reports" collapsible=true start_expanded=true>
                <NavLink href="/reports">"Reports"</NavLink>
            </NavGroup>
            <NavGroup label="Admin" collapsible=true expanded=remember> // the app owns it
                <NavLink href="/admin">"Admin"</NavLink>
            </NavGroup>
        }
        // endregion
    }

    fn side_menu() -> impl IntoView {
        let menu = RwSignal::new(false);
        // region: side-menu
        use ferrox_ui::{NavLink, SideMenu, SideMenuButton};

        view! {
            <SideMenuButton open=menu />
            <SideMenu
                open=menu
                title="Groceries"
                help=MENU_HELP.clone()
                footer=|| view! { <span>"Version 1.4"</span> }
            >
                <NavLink href="/" exact=true>"Lists"</NavLink>
            </SideMenu>
        }
        // endregion
    }
}

// ---------------------------------------------------------------------------
// layout
// ---------------------------------------------------------------------------

#[cfg(feature = "layout")]
mod layout {
    use super::*;

    #[cfg(all(feature = "navigation", feature = "menus"))]
    fn app_bar() -> impl IntoView {
        let sign_out = noop;
        // region: app-bar
        use ferrox_ui::{AppBar, Menu, MenuItem, MenuTrigger, SideMenuButton};

        let menu = RwSignal::new(false); // the SideMenu's, declared beside it

        view! {
            <AppBar title="Cookbook" href="/" start=move || view! { <SideMenuButton open=menu /> }>
                <Menu label="Account" trigger=MenuTrigger::custom(|| view! { <span>"Ada Lovelace"</span> }, "Account")>
                    <MenuItem on_select=move || sign_out()>"Sign out"</MenuItem>
                </Menu>
            </AppBar>
        }
        // endregion
    }

    fn facts() -> impl IntoView {
        // region: facts
        use ferrox_ui::{Fact, Facts};

        view! {
            <Facts>
                <Fact label="Time">"30 minutes"</Fact>
                <Fact label="Serves">"4"</Fact>
            </Facts>
        }
        // endregion
    }

    fn page_grid() -> impl IntoView {
        // region: page-grid
        use ferrox_ui::{HeadingLevel, PageGrid, PageWidth, Panel};

        view! {
            <PageGrid width=PageWidth::Wide side=move || view! { <Panel title="Filters">"…"</Panel> }>
                <Panel title="Recipes" heading_level=HeadingLevel::H1>"…"</Panel>
                <Panel title="Recently made">"…"</Panel>
            </PageGrid>
        }
        // endregion
    }

    fn page() -> impl IntoView {
        let add = noop;
        // region: page
        use ferrox_icons::{Icon, PLUS};
        use ferrox_ui::{IconButton, Page, PageWidth, Panel};

        view! {
            <Page
                title="Recipes"
                width=PageWidth::Wide
                actions=move || view! { <IconButton label="Add recipe" on_click=move || add()><Icon icon=PLUS /></IconButton> }
                side=move || view! { <Panel title="Filters">"…"</Panel> }
                more=move || view! { <Panel title="Recently made">"…"</Panel> }
            >
                "The page's own panel's body: the recipes."
            </Page>
        }
        // endregion
    }

    #[cfg(feature = "forms")]
    fn stack_cluster() -> impl IntoView {
        let (cancel, save) = (noop, noop);
        let (name, minutes) = (RwSignal::new(String::new()), RwSignal::new(String::new()));
        // region: stack-cluster
        use ferrox_ui::{Button, ButtonFamily, Cluster, Field, Gap, Input, InputKind, Justify, Stack};

        view! {
            <Stack gap=Gap::Lg>
                <Field label="Name"><Input value=name /></Field>
                <Field label="Minutes"><Input value=minutes kind=InputKind::Number /></Field>
                <Cluster justify=Justify::End>
                    <Button family=ButtonFamily::Cancel on_click=move || cancel()>"Cancel"</Button>
                    <Button on_click=move || save()>"Save"</Button>
                </Cluster>
            </Stack>
        }
        // endregion
    }

    fn panel() -> impl IntoView {
        let (search, new_item) = (noop, noop);
        let can_add_items = RwSignal::new(true);
        // region: panel
        use ferrox_icons::{Icon, PLUS, SEARCH};
        use ferrox_ui::{HeadingLevel, IconButton, Panel};

        view! {
            <Panel title="Shopping Lists" heading_level=HeadingLevel::H1>
                "…"
            </Panel>

            <Panel
                title="Items"
                actions=move || view! {
                    <IconButton label="Search items" on_click=move || search()><Icon icon=SEARCH /></IconButton>
                    // The app's own permission decides whether "add" shows.
                    <Show when=move || can_add_items.get()>
                        <IconButton label="Add item" on_click=move || new_item()><Icon icon=PLUS /></IconButton>
                    </Show>
                }
            >
                "…"
            </Panel>

            <Panel title="Load Data" heading_level=HeadingLevel::H1>"…"</Panel> // a bar with no buttons
            <Panel title="Transactions" flush=true>"…"</Panel> // a table runs to the edges
        }
        // endregion
    }

    fn panel_collapsible() -> impl IntoView {
        let notes_open = RwSignal::new(false);
        let count = RwSignal::new(3);
        // region: panel-collapsible
        use ferrox_ui::{Panel, PanelTone};

        view! {
            <Panel title="Danger zone" tone=PanelTone::Danger collapsible=true summary="Delete this book">
                "…"
            </Panel>
            <Panel title="Notes" collapsible=true expanded=notes_open summary=Signal::derive(move || format!("{} notes", count.get()))>
                "…"
            </Panel>
        }
        // endregion
    }

    #[derive(Clone)]
    struct Employee {
        id: u32,
        name: String,
        role: String,
    }

    fn card() -> impl IntoView {
        let (remove, open_list) = (noop, noop);
        let employee = Employee { id: 7, name: "Ada Lovelace".into(), role: "Analyst".into() };
        // region: card
        use ferrox_icons::{Icon, DELETE};
        use ferrox_ui::{Card, IconButton};

        view! {
            <Card
                title=employee.name.clone()
                href=format!("/employees/{}", employee.id)
                actions=move || view! {
                    <IconButton label="Remove" danger=true on_click=move || remove()><Icon icon=DELETE /></IconButton>
                }
            >
                <p>{employee.role.clone()}</p>
            </Card>

            <Card title="Groceries" on_click=move || open_list()>"…"</Card> // the whole card is a button
            <Card title="Note" compact=true>"…"</Card> // just a surface
        }
        // endregion
    }
}

// ---------------------------------------------------------------------------
// data
// ---------------------------------------------------------------------------

#[cfg(all(feature = "data", feature = "layout"))]
mod data {
    use super::*;

    fn data_intro() -> impl IntoView {
        let (loading, error) = (RwSignal::new(false), RwSignal::new(None::<String>));
        let reload = noop;
        let recipes_on = |_page: usize| Vec::<Recipe>::new();
        let page_count = Signal::stored(5_usize);
        // region: data
        use ferrox_ui::{Card, DataLayout, DataLayoutSwitch, DataView, Pagination, Panel};

        let layout = RwSignal::new(DataLayout::default());
        let page = RwSignal::new(1);

        view! {
            <Panel title="Recipes" actions=move || view! { <DataLayoutSwitch layout=layout /> }>
                <DataView
                    items=Signal::derive(move || recipes_on(page.get()))
                    key_of=|r: &Recipe| r.id.clone()
                    item=|r: Recipe| view! { <Card title=r.name.clone() href=format!("/recipes/{}", r.id)>"…"</Card> }
                    label="Recipes"
                    layout=layout
                    loading=loading
                    error=error
                    on_retry=move || reload()
                />
                <Pagination page=page page_count=page_count />
            </Panel>
        }
        // endregion
    }

    fn data_view() -> impl IntoView {
        let recipes = recipes();
        let (loading, error) = (RwSignal::new(false), RwSignal::new(None::<String>));
        let reload = noop;
        // region: data-view
        use ferrox_ui::{Card, DataLayout, DataView};

        view! {
            <DataView
                items=recipes
                key_of=|r: &Recipe| r.id.clone()
                item=|r: Recipe| view! { <Card title=r.name.clone() href=format!("/recipes/{}", r.id)>"…"</Card> }
                label="Recipes"
                layout=DataLayout::default()
                loading=loading
                error=error
                on_retry=move || reload()
                empty_label="No recipes yet."
            />
        }
        // endregion
    }

    #[derive(Clone, PartialEq)]
    struct Entry {
        id: String,
        text: String,
    }

    fn data_view_more() -> impl IntoView {
        let entries = Signal::stored(Vec::<Entry>::new());
        let entry_row = |e: Entry| view! { <span>{e.text}</span> };
        let (has_more, loading_more) = (RwSignal::new(true), RwSignal::new(false));
        let load_next_page = noop;
        // region: data-view-more
        use ferrox_ui::{DataLayout, DataView};

        view! {
            <DataView
                items=entries
                key_of=|e: &Entry| e.id.clone()
                item=entry_row
                label="Activity"
                layout=DataLayout::List
                on_more=move || load_next_page()
                has_more=has_more
                loading_more=loading_more
            />
        }
        // endregion
    }

    fn data_layout_switch() -> impl IntoView {
        let recipes = recipes();
        // region: data-layout-switch
        use ferrox_ui::{Card, DataLayout, DataLayoutSwitch, DataView, Panel};

        let layout = RwSignal::new(DataLayout::default());

        view! {
            <Panel title="Recipes" actions=move || view! { <DataLayoutSwitch layout=layout /> }>
                <DataView
                    items=recipes
                    key_of=|r: &Recipe| r.id.clone()
                    item=|r: Recipe| view! { <Card title=r.name>"…"</Card> }
                    label="Recipes"
                    layout=layout
                />
            </Panel>
        }
        // endregion
    }

    fn pagination() -> impl IntoView {
        let total = RwSignal::new(240_usize);
        let slice = |_page: usize| Vec::<Recipe>::new();
        // region: pagination
        use ferrox_ui::{Card, DataView, Pagination};

        let page = RwSignal::new(1);
        let page_count = Signal::derive(move || total.get().div_ceil(20));

        view! {
            <DataView
                items=Signal::derive(move || slice(page.get()))
                key_of=|r: &Recipe| r.id.clone()
                item=|r: Recipe| view! { <Card title=r.name>"…"</Card> }
                label="Recipes"
            />
            <Pagination page=page page_count=page_count />
        }
        // endregion
    }
}

// ---------------------------------------------------------------------------
// menus
// ---------------------------------------------------------------------------

#[cfg(feature = "menus")]
mod menus {
    use super::*;

    fn menu() -> impl IntoView {
        let (edit, duplicate, ask_delete) = (noop, noop, noop);
        // region: menu
        use ferrox_icons::MORE_VERTICAL;
        use ferrox_ui::{Menu, MenuDivider, MenuItem, MenuTrigger};

        view! {
            <Menu label="Recipe actions" trigger=MenuTrigger::icon(MORE_VERTICAL, "More actions")>
                <MenuItem on_select=move || edit()>"Edit"</MenuItem>
                <MenuItem on_select=move || duplicate()>"Duplicate"</MenuItem>
                <MenuDivider />
                <MenuItem on_select=move || ask_delete()>"Delete"</MenuItem>
            </Menu>
        }
        // endregion
    }

    fn menu_items() -> impl IntoView {
        let (edit, archive, ask_delete) = (noop, noop, noop);
        let can_archive = RwSignal::new(false);
        let book = RwSignal::new("Household");
        // region: menu-items
        use ferrox_icons::{DELETE, EDIT, MORE_VERTICAL};
        use ferrox_ui::{Menu, MenuDivider, MenuItem, MenuLink, MenuTrigger};

        view! {
            <Menu label="Actions" trigger=MenuTrigger::icon(MORE_VERTICAL, "More actions")>
                <MenuItem icon=EDIT on_select=move || edit()>"Edit"</MenuItem>
                <MenuItem on_select=move || archive() disabled=Signal::derive(move || !can_archive.get())>"Archive"</MenuItem>
                <MenuDivider />
                <MenuItem icon=DELETE danger=true on_select=move || ask_delete()>"Delete"</MenuItem>
            </Menu>

            <Menu label="Books" trigger=MenuTrigger::button("Book")>
                <MenuItem current=Signal::derive(move || book.get() == "Household") on_select=move || book.set("Household")>
                    "Household"
                </MenuItem>
                <MenuLink href="/settings">"Settings"</MenuLink>
            </Menu>
        }
        // endregion
    }

    fn split_button() -> impl IntoView {
        let (save, save_as_draft, save_and_close) = (noop, noop, noop);
        let saving = RwSignal::new(false);
        // region: split-button
        use ferrox_ui::{MenuItem, SplitButton};

        view! {
            <SplitButton text="Save" on_click=move || save() menu_label="More save options" saving=saving>
                <MenuItem on_select=move || save_as_draft()>"Save as draft"</MenuItem>
                <MenuItem on_select=move || save_and_close()>"Save and close"</MenuItem>
            </SplitButton>
        }
        // endregion
    }

    #[cfg(feature = "layout")]
    fn context_menu() -> impl IntoView {
        let (edit, ask_delete) = (noop, noop);
        let recipe = Recipe { id: "7".into(), name: "Shakshuka".into() };
        // region: context-menu
        use ferrox_icons::{DELETE, EDIT};
        use ferrox_ui::{Card, ContextMenu, MenuItem};

        view! {
            <ContextMenu label="Recipe actions" menu=move || view! {
                <MenuItem icon=EDIT on_select=move || edit()>"Edit"</MenuItem>
                <MenuItem icon=DELETE danger=true on_select=move || ask_delete()>"Delete"</MenuItem>
            }>
                <Card title=recipe.name.clone() href=format!("/recipes/{}", recipe.id)>"…"</Card>
            </ContextMenu>
        }
        // endregion
    }
}
