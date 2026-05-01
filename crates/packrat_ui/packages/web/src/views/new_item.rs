use dioxus::prelude::*;

use crate::Route;
use crate::api_client;
use super::recent_store::{remember_recent, RecentBrief};

#[component]
pub fn NewItem() -> Element {
    let api_base = use_context::<Signal<String>>();
    let recent = use_context::<Signal<Vec<RecentBrief>>>();

    let mut name = use_signal(String::new);
    // Empty string = no parent; otherwise a decimal item id.
    let mut parent_sel = use_signal(String::new);
    let mut flash = use_signal(|| Option::<String>::None);
    let mut busy = use_signal(|| false);
    let mut list_gen = use_signal(|| 0u32);

    let items = use_resource(move || {
        let _ = list_gen();
        let base = api_base();
        async move { api_client::list_items(&base).await }
    });

    rsx! {
        div {
            class: "max-w-lg mx-auto px-4 py-8 space-y-8",

            div {
                class: "space-y-1",
                h1 { class: "text-2xl font-semibold text-ui-text tracking-tight", "New item" }
                p { class: "text-sm text-ui-text-muted",
                    "Name is required. Parent is optional — pick an existing item or leave this row at the top level."
                }
            }

            div {
                class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-6 space-y-5",

                label {
                    class: "flex flex-col gap-2 text-sm text-ui-text-muted",
                    span { "Name" }
                    input {
                        class: "bg-ui-bg-dim border border-ui-bg-dim rounded-lg px-3 py-2.5 text-ui-text placeholder:text-ui-text-dim focus:outline-none focus:ring-2 focus:ring-ui-secondary",
                        placeholder: "e.g. Canon R5",
                        value: "{name}",
                        oninput: move |e| *name.write() = e.value(),
                    }
                }

                div {
                    class: "flex flex-col gap-2 text-sm text-ui-text-muted",
                    span { "Parent" }
                    match items() {
                        None => rsx! {
                            p { class: "text-sm text-ui-text-dim", "Loading items…" }
                        },
                        Some(Err(err)) => rsx! {
                            p { class: "text-sm text-ui-error", "{err}" }
                        },
                        Some(Ok(list)) => rsx! {
                            select {
                                class: "bg-ui-bg-dim border border-ui-bg-dim rounded-lg px-3 py-2.5 text-ui-text focus:outline-none focus:ring-2 focus:ring-ui-secondary",
                                value: "{parent_sel}",
                                onchange: move |e| *parent_sel.write() = e.value(),
                                option { value: "", "— None (top level) —" }
                                for row in list.iter() {
                                    option {
                                        key: "{row.id}",
                                        value: "{row.id}",
                                        "#{row.id} — {row.name}"
                                    }
                                }
                            }
                        },
                    }
                }

                div {
                    class: "flex flex-wrap gap-3",
                    button {
                        class: "rounded-lg bg-ui-primary text-ui-bg px-4 py-2.5 text-sm font-medium hover:opacity-90 disabled:opacity-50",
                        disabled: busy(),
                        onclick: move |_| {
                            let base = api_base();
                            let n = name().trim().to_string();
                            if n.is_empty() {
                                flash.set(Some("Name is required.".into()));
                                return;
                            }
                            let parent_id = match parent_sel().trim() {
                                "" => None,
                                s => match s.parse::<i64>() {
                                    Ok(id) => Some(id),
                                    Err(_) => {
                                        flash.set(Some("Parent must be a valid item id.".into()));
                                        return;
                                    }
                                },
                            };
                            let recent_sig = recent;
                            busy.set(true);
                            flash.set(None);
                            spawn(async move {
                                match api_client::create_item(&base, n, parent_id).await {
                                    Ok(item) => {
                                        remember_recent(recent_sig, item.id, item.name.clone());
                                        flash.set(Some(format!("Created #{} — {}", item.id, item.name)));
                                        name.write().clear();
                                        parent_sel.write().clear();
                                        *list_gen.write() += 1;
                                    }
                                    Err(err) => flash.set(Some(format!("Error: {err}"))),
                                }
                                busy.set(false);
                            });
                        },
                        if busy() { "Saving…" } else { "Create item" }
                    }
                    Link {
                        class: "rounded-lg border border-ui-bg-dim px-4 py-2.5 text-sm font-medium text-ui-text hover:bg-ui-bg-dim",
                        to: Route::Home {},
                        "Back to dashboard"
                    }
                }

                if let Some(msg) = flash() {
                    p { class: "text-sm text-ui-info", "{msg}" }
                }
            }
        }
    }
}
