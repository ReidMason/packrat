use dioxus::prelude::*;

use crate::api_client::{self, ItemDto};

const DEFAULT_API_BASE: &str = "http://127.0.0.1:3000";

fn spawn_status_check(
    base: String,
    mut status_line: Signal<Option<String>>,
    mut status_busy: Signal<bool>,
) {
    status_busy.set(true);
    spawn(async move {
        let h = api_client::fetch_health(&base).await;
        let r = api_client::fetch_ready(&base).await;
        let msg = match (h, r) {
            (Ok(he), Ok(re)) => format!(
                "Liveness: {} — Readiness: {} (database: {})",
                he.status, re.status, re.database
            ),
            (Ok(he), Err(e)) => format!("Liveness: {} — Readiness error: {e}", he.status),
            (Err(e), _) => format!("Liveness error: {e}"),
        };
        status_line.set(Some(msg));
        status_busy.set(false);
    });
}

#[component]
pub fn Inventory() -> Element {
    let mut api_base = use_signal(|| DEFAULT_API_BASE.to_string());
    let status_line = use_signal(|| Option::<String>::None);
    let status_busy = use_signal(|| false);

    let mut create_name = use_signal(|| String::new());
    let mut create_parent = use_signal(|| String::new());
    let mut create_flash = use_signal(|| Option::<String>::None);

    let mut lookup_id = use_signal(|| String::new());
    let mut looked_up = use_signal(|| Option::<Result<ItemDto, String>>::None);
    let mut lookup_busy = use_signal(|| false);

    let mut delete_id = use_signal(|| String::new());
    let mut delete_flash = use_signal(|| Option::<String>::None);

    use_hook(|| {
        spawn_status_check(api_base(), status_line, status_busy);
    });

    rsx! {
        div {
            class: "max-w-3xl mx-auto px-4 py-8 space-y-10",

            header {
                class: "space-y-2",
                h1 {
                    class: "text-2xl font-semibold text-ui-text tracking-tight",
                    "Packrat"
                }
                p {
                    class: "text-ui-text-muted text-sm leading-relaxed",
                    "Inventory backed by the Packrat API. Start ",
                    code { class: "text-ui-secondary px-1", "packrat_api" },
                    " (default ",
                    code { class: "text-ui-secondary px-1", "{DEFAULT_API_BASE}" },
                    "), then use this UI to create and inspect nested items."
                }
            }

            section {
                class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-5 space-y-4",
                h2 { class: "text-lg font-medium text-ui-text", "API connection" }
                div {
                    class: "flex flex-col sm:flex-row gap-3 sm:items-end",
                    label {
                        class: "flex-1 flex flex-col gap-1 text-sm text-ui-text-muted",
                        span { "Base URL" }
                        input {
                            class: "bg-ui-bg-dim border border-ui-bg-dim rounded-lg px-3 py-2 text-ui-text placeholder:text-ui-text-dim focus:outline-none focus:ring-2 focus:ring-ui-secondary",
                            r#type: "url",
                            placeholder: "{DEFAULT_API_BASE}",
                            value: "{api_base}",
                            oninput: move |e| *api_base.write() = e.value(),
                        }
                    }
                    button {
                        class: "shrink-0 rounded-lg bg-ui-primary text-ui-bg px-4 py-2 text-sm font-medium hover:opacity-90 disabled:opacity-50",
                        disabled: status_busy(),
                        onclick: move |_| {
                            spawn_status_check(api_base(), status_line, status_busy);
                        },
                        if status_busy() { "Checking…" } else { "Refresh status" }
                    }
                }
                if let Some(line) = status_line() {
                    p {
                        class: "text-sm text-ui-success font-mono",
                        "{line}"
                    }
                }
            }

            section {
                class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-5 space-y-4",
                h2 { class: "text-lg font-medium text-ui-text", "Create item" }
                p {
                    class: "text-sm text-ui-text-muted",
                    "Optional parent ID links this row into the tree (e.g. a lens under a camera body)."
                }
                div {
                    class: "grid sm:grid-cols-2 gap-3",
                    label {
                        class: "flex flex-col gap-1 text-sm text-ui-text-muted",
                        span { "Name" }
                        input {
                            class: "bg-ui-bg-dim border border-ui-bg-dim rounded-lg px-3 py-2 text-ui-text focus:outline-none focus:ring-2 focus:ring-ui-secondary",
                            placeholder: "e.g. Canon R5",
                            value: "{create_name}",
                            oninput: move |e| *create_name.write() = e.value(),
                        }
                    }
                    label {
                        class: "flex flex-col gap-1 text-sm text-ui-text-muted",
                        span { "Parent ID (optional)" }
                        input {
                            class: "bg-ui-bg-dim border border-ui-bg-dim rounded-lg px-3 py-2 text-ui-text focus:outline-none focus:ring-2 focus:ring-ui-secondary",
                            placeholder: "empty for root",
                            value: "{create_parent}",
                            oninput: move |e| *create_parent.write() = e.value(),
                        }
                    }
                }
                button {
                    class: "rounded-lg bg-ui-primary text-ui-bg px-4 py-2 text-sm font-medium hover:opacity-90",
                    onclick: move |_| {
                        let base = api_base();
                        let name = create_name().trim().to_string();
                        let parent_raw = create_parent().trim().to_string();
                        let parent_id = if parent_raw.is_empty() {
                            None
                        } else {
                            match parent_raw.parse::<i64>() {
                                Ok(id) => Some(id),
                                Err(_) => {
                                    create_flash.set(Some("Parent ID must be a number.".into()));
                                    return;
                                }
                            }
                        };
                        if name.is_empty() {
                            create_flash.set(Some("Name is required.".into()));
                            return;
                        }
                        spawn(async move {
                            match api_client::create_item(&base, name, parent_id).await {
                                Ok(item) => {
                                    create_flash.set(Some(format!(
                                        "Created item #{} — {}",
                                        item.id, item.name
                                    )));
                                    create_name.write().clear();
                                    create_parent.write().clear();
                                }
                                Err(e) => create_flash.set(Some(format!("Error: {e}"))),
                            }
                        });
                    },
                    "Create"
                }
                if let Some(msg) = create_flash() {
                    p { class: "text-sm text-ui-info", "{msg}" }
                }
            }

            section {
                class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-5 space-y-4",
                h2 { class: "text-lg font-medium text-ui-text", "Look up by ID" }
                div {
                    class: "flex flex-col sm:flex-row gap-3 sm:items-end",
                    label {
                        class: "flex-1 flex flex-col gap-1 text-sm text-ui-text-muted",
                        span { "Item ID" }
                        input {
                            class: "bg-ui-bg-dim border border-ui-bg-dim rounded-lg px-3 py-2 text-ui-text focus:outline-none focus:ring-2 focus:ring-ui-secondary",
                            r#type: "text",
                            inputmode: "numeric",
                            placeholder: "e.g. 1",
                            value: "{lookup_id}",
                            oninput: move |e| *lookup_id.write() = e.value(),
                        }
                    }
                    button {
                        class: "shrink-0 rounded-lg bg-ui-secondary text-ui-bg px-4 py-2 text-sm font-medium hover:opacity-90 disabled:opacity-50",
                        disabled: lookup_busy(),
                        onclick: move |_| {
                            let base = api_base();
                            let raw = lookup_id().trim().to_string();
                            lookup_busy.set(true);
                            spawn(async move {
                                let res = match raw.parse::<i64>() {
                                    Ok(id) => api_client::get_item(&base, id).await,
                                    Err(_) => Err("ID must be a number.".into()),
                                };
                                looked_up.set(Some(res));
                                lookup_busy.set(false);
                            });
                        },
                        if lookup_busy() { "Loading…" } else { "Load" }
                    }
                }
                if let Some(res) = looked_up() {
                    match res {
                        Ok(item) => rsx! { ItemCard { item } },
                        Err(e) => rsx! {
                            p { class: "text-sm text-ui-error", "{e}" }
                        },
                    }
                }
            }

            section {
                class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-5 space-y-4",
                h2 { class: "text-lg font-medium text-ui-text", "Delete item" }
                p {
                    class: "text-sm text-ui-text-muted",
                    "Deletes are soft on the server. Items with active children return 409."
                }
                div {
                    class: "flex flex-col sm:flex-row gap-3 sm:items-end",
                    label {
                        class: "flex-1 flex flex-col gap-1 text-sm text-ui-text-muted",
                        span { "Item ID" }
                        input {
                            class: "bg-ui-bg-dim border border-ui-bg-dim rounded-lg px-3 py-2 text-ui-text focus:outline-none focus:ring-2 focus:ring-ui-secondary",
                            r#type: "text",
                            inputmode: "numeric",
                            value: "{delete_id}",
                            oninput: move |e| *delete_id.write() = e.value(),
                        }
                    }
                    button {
                        class: "shrink-0 rounded-lg bg-ui-error text-ui-bg px-4 py-2 text-sm font-medium hover:opacity-90",
                        onclick: move |_| {
                            let base = api_base();
                            let raw = delete_id().trim().to_string();
                            spawn(async move {
                                let out = match raw.parse::<i64>() {
                                    Ok(id) => match api_client::delete_item(&base, id).await {
                                        Ok(()) => format!("Item #{id} deleted (or marked deleted)."),
                                        Err(e) => format!("Error: {e}"),
                                    },
                                    Err(_) => "ID must be a number.".into(),
                                };
                                delete_flash.set(Some(out));
                            });
                        },
                        "Delete"
                    }
                }
                if let Some(msg) = delete_flash() {
                    p { class: "text-sm text-ui-warning", "{msg}" }
                }
            }
        }
    }
}

#[component]
fn ItemCard(item: ItemDto) -> Element {
    rsx! {
        div {
            class: "rounded-lg border border-ui-bg-dim bg-ui-bg-dim/40 p-4 space-y-2 text-sm",
            div {
                class: "flex flex-wrap gap-x-4 gap-y-1 text-ui-text",
                span { class: "font-medium", "ID {item.id}" }
                span { class: "text-ui-text-muted", "—" }
                span { "{item.name}" }
            }
            dl {
                class: "grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 text-ui-text-muted font-mono text-xs",
                dt { "parent_id" }
                dd { "{item.parent_id:?}" }
                dt { "created" }
                dd { "{item.created}" }
                dt { "deleted" }
                dd { "{item.deleted:?}" }
            }
        }
    }
}
