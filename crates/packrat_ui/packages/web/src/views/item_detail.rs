use dioxus::prelude::*;

use crate::Route;
use crate::api_client;
use super::recent_store::{self, RecentBrief};

#[component]
pub fn ItemDetail(id: i64) -> Element {
    let api_base = use_context::<Signal<String>>();
    let recent = use_context::<Signal<Vec<RecentBrief>>>();
    let navigator = use_navigator();

    let mut load_id = use_signal(|| id);
    use_effect(move || {
        *load_id.write() = id;
    });

    let item_res = use_resource(move || {
        let item_id = load_id();
        let base = api_base();
        async move { api_client::get_item(&base, item_id).await }
    });

    let mut delete_confirm = use_signal(|| false);
    let mut delete_busy = use_signal(|| false);
    let mut delete_msg = use_signal(|| Option::<String>::None);

    rsx! {
        div {
            class: "max-w-2xl mx-auto px-4 py-8 space-y-6",

            Link {
                class: "inline-block text-sm font-medium text-ui-primary hover:underline",
                to: Route::Home {},
                "← Dashboard"
            }

            match item_res() {
                None => rsx! {
                    p { class: "text-sm text-ui-text-muted", "Loading…" }
                },
                Some(Err(e)) => rsx! {
                    div { class: "space-y-3",
                        p { class: "text-sm text-ui-error", "{e}" }
                        Link {
                            class: "text-sm font-medium text-ui-primary hover:underline",
                            to: Route::Home {},
                            "Back to dashboard"
                        }
                    }
                },
                Some(Ok(item)) => {
                    let item_id = item.id;
                    let name = item.name.clone();
                    let parent_note = if item.parent_id.is_some() {
                        "Nested under another item"
                    } else {
                        "Top level"
                    };
                    let created = item.created.clone();
                    let deleted = item.deleted.is_some();
                    rsx! {
                        section {
                            class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-6 space-y-5",
                            h1 {
                                class: "text-2xl font-semibold text-ui-text tracking-tight",
                                "{name}"
                            }
                            dl {
                                class: "grid grid-cols-[auto_1fr] gap-x-4 gap-y-2 text-sm text-ui-text-muted max-w-md",
                                dt { "Placement" }
                                dd { "{parent_note}" }
                                dt { "Created" }
                                dd { class: "font-mono text-xs", "{created}" }
                                if deleted {
                                    dt { "Status" }
                                    dd { "Removed" }
                                }
                            }

                            div {
                                class: "pt-4 border-t border-ui-bg-dim space-y-4",

                                if delete_confirm() {
                                    p { class: "text-sm text-ui-text",
                                        "Remove “{name}”? This cannot be undone."
                                    }
                                    div {
                                        class: "flex flex-wrap gap-3",
                                        button {
                                            class: "rounded-lg border border-ui-bg-dim px-4 py-2 text-sm font-medium text-ui-text hover:bg-ui-bg-dim",
                                            disabled: delete_busy(),
                                            onclick: move |_| {
                                                delete_confirm.set(false);
                                                delete_msg.set(None);
                                            },
                                            "Cancel"
                                        }
                                        button {
                                            class: "rounded-lg bg-ui-error text-ui-bg px-4 py-2 text-sm font-medium hover:opacity-90 disabled:opacity-50",
                                            disabled: delete_busy(),
                                            onclick: move |_| {
                                                let base = api_base();
                                                let nav = navigator;
                                                let rec = recent;
                                                delete_busy.set(true);
                                                delete_msg.set(None);
                                                spawn(async move {
                                                    match api_client::delete_item(&base, item_id).await {
                                                        Ok(()) => {
                                                            recent_store::remove_recent(rec, item_id);
                                                            nav.push(Route::Home {});
                                                        }
                                                        Err(err) => delete_msg.set(Some(err)),
                                                    }
                                                    delete_busy.set(false);
                                                });
                                            },
                                            if delete_busy() { "Deleting…" } else { "Delete permanently" }
                                        }
                                    }
                                } else {
                                    button {
                                        class: "rounded-lg border border-ui-error/60 bg-ui-bg-dim/40 px-4 py-2.5 text-sm font-medium text-ui-error hover:bg-ui-error/10",
                                        onclick: move |_| {
                                            delete_confirm.set(true);
                                            delete_msg.set(None);
                                        },
                                        "Delete item"
                                    }
                                }

                                if let Some(msg) = delete_msg() {
                                    p { class: "text-sm text-ui-error", "{msg}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
