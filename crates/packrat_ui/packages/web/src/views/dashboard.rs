use dioxus::prelude::*;

use crate::api_client::{self, ItemDto};
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
const RECENT_KEY: &str = "packrat_recent_v1";
const MAX_RECENT: usize = 10;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct RecentBrief {
    id: i64,
    name: String,
}

#[cfg(target_arch = "wasm32")]
fn load_recent_disk() -> Vec<RecentBrief> {
    try_load_recent().unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
fn try_load_recent() -> Option<Vec<RecentBrief>> {
    let window = web_sys::window()?;
    let storage = window.local_storage().ok().flatten()?;
    let json = storage.get_item(RECENT_KEY).ok().flatten()?;
    serde_json::from_str(&json).ok()
}

#[cfg(not(target_arch = "wasm32"))]
fn load_recent_disk() -> Vec<RecentBrief> {
    Vec::new()
}

#[cfg(target_arch = "wasm32")]
fn save_recent_disk(entries: &[RecentBrief]) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(json) = serde_json::to_string(entries) {
                let _ = storage.set_item(RECENT_KEY, &json);
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_recent_disk(_entries: &[RecentBrief]) {}

fn remember_recent(mut recent: Signal<Vec<RecentBrief>>, id: i64, name: String) {
    let mut v = recent();
    v.retain(|e| e.id != id);
    v.insert(0, RecentBrief { id, name });
    v.truncate(MAX_RECENT);
    save_recent_disk(&v);
    recent.set(v);
}

fn spawn_lookup(
    base: String,
    raw: String,
    mut lookup_busy: Signal<bool>,
    mut looked_up: Signal<Option<Result<ItemDto, String>>>,
    recent: Signal<Vec<RecentBrief>>,
) {
    lookup_busy.set(true);
    spawn(async move {
        let res = match raw.parse::<i64>() {
            Ok(id) => api_client::get_item(&base, id).await,
            Err(_) => Err("ID must be a number.".into()),
        };
        if let Ok(ref item) = res {
            remember_recent(recent, item.id, item.name.clone());
        }
        looked_up.set(Some(res));
        lookup_busy.set(false);
    });
}

#[component]
pub fn Dashboard() -> Element {
    let api_base = use_context::<Signal<String>>();

    let mut create_name = use_signal(|| String::new());
    let mut create_parent = use_signal(|| String::new());
    let mut create_flash = use_signal(|| Option::<String>::None);

    let mut lookup_id = use_signal(|| String::new());
    let looked_up = use_signal(|| Option::<Result<ItemDto, String>>::None);
    let lookup_busy = use_signal(|| false);

    let mut delete_id = use_signal(|| String::new());
    let mut delete_flash = use_signal(|| Option::<String>::None);

    let mut recent = use_signal(Vec::<RecentBrief>::new);

    use_hook(move || {
        recent.set(load_recent_disk());
    });

    rsx! {
        div {
            class: "max-w-6xl mx-auto px-4 py-8 space-y-8",

            div {
                class: "space-y-1",
                h1 { class: "text-2xl font-semibold text-ui-text tracking-tight", "Dashboard" }
                p { class: "text-sm text-ui-text-muted max-w-2xl leading-relaxed",
                    "Add inventory rows and open items by ID. The API does not list everything yet — ",
                    "recent opens are kept in this browser only. API URL and health checks live under ",
                    "Debug in the sidebar."
                }
            }

            div {
                class: "grid gap-6 lg:grid-cols-2",
                section {
                    class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-5 space-y-4 h-full",
                    h2 { class: "text-lg font-medium text-ui-text", "Quick add" }
                    p { class: "text-sm text-ui-text-muted",
                        "Create a row in the tree. Parent ID is optional (e.g. lens under a camera body)."
                    }
                    div {
                        class: "space-y-3",
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
                        class: "w-full sm:w-auto rounded-lg bg-ui-primary text-ui-bg px-4 py-2 text-sm font-medium hover:opacity-90",
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
                            let recent_sig = recent;
                            spawn(async move {
                                match api_client::create_item(&base, name, parent_id).await {
                                    Ok(item) => {
                                        create_flash.set(Some(format!(
                                            "Created #{} — {}",
                                            item.id, item.name
                                        )));
                                        create_name.write().clear();
                                        create_parent.write().clear();
                                        remember_recent(recent_sig, item.id, item.name);
                                    }
                                    Err(e) => create_flash.set(Some(format!("Error: {e}"))),
                                }
                            });
                        },
                        "Create item"
                    }
                    if let Some(msg) = create_flash() {
                        p { class: "text-sm text-ui-info", "{msg}" }
                    }
                }

                section {
                    class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-5 space-y-4 h-full",
                    h2 { class: "text-lg font-medium text-ui-text", "Open by ID" }
                    p { class: "text-sm text-ui-text-muted", "Load one item from the API." }
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
                                spawn_lookup(
                                    base,
                                    raw,
                                    lookup_busy,
                                    looked_up,
                                    recent,
                                );
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
            }

            if !recent().is_empty() {
                section {
                    class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-5 space-y-3",
                    div {
                        class: "flex items-center justify-between gap-4",
                        h2 { class: "text-lg font-medium text-ui-text", "Recent in this browser" }
                        button {
                            class: "text-xs text-ui-text-muted hover:text-ui-error",
                            onclick: move |_| {
                                recent.set(Vec::new());
                                save_recent_disk(&[]);
                            },
                            "Clear"
                        }
                    }
                    ul {
                        class: "divide-y divide-ui-bg-dim",
                        for entry in recent().into_iter() {
                            li {
                                key: "{entry.id}",
                                class: "flex items-center justify-between gap-3 py-3 first:pt-0",
                                div {
                                    class: "min-w-0",
                                    p { class: "text-sm font-medium text-ui-text truncate", "{entry.name}" }
                                    p { class: "text-xs text-ui-text-muted font-mono", "#{entry.id}" }
                                }
                                button {
                                    class: "shrink-0 rounded-lg border border-ui-bg-dim px-3 py-1.5 text-xs font-medium text-ui-text hover:bg-ui-bg-dim",
                                    onclick: {
                                        let id = entry.id;
                                        move |_| {
                                            *lookup_id.write() = id.to_string();
                                            spawn_lookup(
                                                api_base(),
                                                id.to_string(),
                                                lookup_busy,
                                                looked_up,
                                                recent,
                                            );
                                        }
                                    },
                                    "Open"
                                }
                            }
                        }
                    }
                }
            }

            section {
                class: "rounded-xl border border-dashed border-ui-bg-dim bg-ui-bg-dim/30 p-5 space-y-4",
                h2 { class: "text-lg font-medium text-ui-text", "Delete item" }
                p { class: "text-sm text-ui-text-muted",
                    "Soft delete on the server. Items with active children return 409."
                }
                div {
                    class: "flex flex-col sm:flex-row gap-3 sm:items-end max-w-xl",
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
