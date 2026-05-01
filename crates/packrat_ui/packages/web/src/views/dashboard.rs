use dioxus::prelude::*;

use crate::Route;
use crate::api_client::{self, ItemDto};
use super::recent_store::{remember_recent, save_recent_disk, RecentBrief};

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
            Err(_) => Err("Enter a whole number.".into()),
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
    let mut recent = use_context::<Signal<Vec<RecentBrief>>>();

    let mut lookup_id = use_signal(|| String::new());
    let looked_up = use_signal(|| Option::<Result<ItemDto, String>>::None);
    let lookup_busy = use_signal(|| false);

    let mut delete_id = use_signal(|| String::new());
    let mut delete_flash = use_signal(|| Option::<String>::None);

    rsx! {
        div {
            class: "max-w-6xl mx-auto px-4 py-8 space-y-8",

            div {
                class: "space-y-1",
                h1 { class: "text-2xl font-semibold text-ui-text tracking-tight", "Dashboard" }
                p { class: "text-sm text-ui-text-muted max-w-2xl leading-relaxed",
                    "Look up or remove items. Recent opens stay in this browser only. ",
                    "API URL and health checks are under Debug."
                }
                Link {
                    class: "inline-flex mt-4 rounded-lg bg-ui-primary text-ui-bg px-4 py-2.5 text-sm font-medium hover:opacity-90",
                    to: Route::NewItem {},
                    "Add new item"
                }
            }

            section {
                class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-5 space-y-4 max-w-2xl",
                h2 { class: "text-lg font-medium text-ui-text", "Look up item" }
                p { class: "text-sm text-ui-text-muted", "Load one item from the API using its reference number." }
                div {
                    class: "flex flex-col sm:flex-row gap-3 sm:items-end",
                    label {
                        class: "flex-1 flex flex-col gap-1 text-sm text-ui-text-muted",
                        span { "Reference" }
                        input {
                            class: "bg-ui-bg-dim border border-ui-bg-dim rounded-lg px-3 py-2 text-ui-text focus:outline-none focus:ring-2 focus:ring-ui-secondary",
                            r#type: "text",
                            inputmode: "numeric",
                            placeholder: "Whole number",
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

            if !recent().is_empty() {
                section {
                    class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-5 space-y-3 max-w-2xl",
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
                                }
                                button {
                                    class: "shrink-0 rounded-lg border border-ui-bg-dim px-3 py-1.5 text-xs font-medium text-ui-text hover:bg-ui-bg-dim",
                                    onclick: {
                                        let id = entry.id;
                                        move |_| {
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
                class: "rounded-xl border border-dashed border-ui-bg-dim bg-ui-bg-dim/30 p-5 space-y-4 max-w-2xl",
                h2 { class: "text-lg font-medium text-ui-text", "Delete item" }
                p { class: "text-sm text-ui-text-muted",
                    "Soft delete on the server. Items with active children return 409."
                }
                div {
                    class: "flex flex-col sm:flex-row gap-3 sm:items-end max-w-xl",
                    label {
                        class: "flex-1 flex flex-col gap-1 text-sm text-ui-text-muted",
                        span { "Reference" }
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
                                        Ok(()) => "Item removed.".into(),
                                        Err(e) => format!("Error: {e}"),
                                    },
                                    Err(_) => "Enter a whole number.".into(),
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
    let parent_note = if item.parent_id.is_some() {
        "Nested under another item"
    } else {
        "Top level"
    };
    rsx! {
        div {
            class: "rounded-lg border border-ui-bg-dim bg-ui-bg-dim/40 p-4 space-y-2 text-sm",
            p { class: "text-base font-medium text-ui-text", "{item.name}" }
            dl {
                class: "grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 text-ui-text-muted text-xs",
                dt { "Placement" }
                dd { "{parent_note}" }
                dt { "Created" }
                dd { class: "font-mono", "{item.created}" }
                if item.deleted.is_some() {
                    dt { "Status" }
                    dd { "Removed" }
                }
            }
        }
    }
}
