use dioxus::prelude::*;

use crate::Route;
use crate::api_client::{self, AssetDto};
use super::recent_store::{remember_recent, save_recent_disk, RecentBrief};

fn spawn_search(
    base: String,
    query: String,
    mut search_busy: Signal<bool>,
    mut search_results: Signal<Option<Result<Vec<AssetDto>, String>>>,
    recent: Signal<Vec<RecentBrief>>,
) {
    search_busy.set(true);
    spawn(async move {
        let res = if query.trim().is_empty() {
            Err("Enter a search term.".into())
        } else {
            api_client::search_assets(&base, &query).await
        };
        if let Ok(ref list) = res {
            if let Some(asset) = list.first() {
                remember_recent(recent, asset.id, asset.name.clone());
            }
        }
        search_results.set(Some(res));
        search_busy.set(false);
    });
}

#[component]
pub fn Dashboard() -> Element {
    let api_base = use_context::<Signal<String>>();
    let mut recent = use_context::<Signal<Vec<RecentBrief>>>();

    let mut search_term = use_signal(String::new);
    let search_results = use_signal(|| Option::<Result<Vec<AssetDto>, String>>::None);
    let search_busy = use_signal(|| false);

    rsx! {
        div {
            class: "max-w-6xl mx-auto px-4 py-8 space-y-8",

            div {
                class: "space-y-1",
                h1 { class: "text-2xl font-semibold text-ui-text tracking-tight", "Dashboard" }
                p { class: "text-sm text-ui-text-muted max-w-2xl leading-relaxed",
                    "Search assets by name. Recent opens stay in this browser only. ",
                    "API URL and health checks are under Debug."
                }
                Link {
                    class: "inline-flex mt-4 rounded-lg bg-ui-primary text-ui-bg px-4 py-2.5 text-sm font-medium hover:opacity-90",
                    to: Route::NewAsset {},
                    "Add new asset"
                }
            }

            section {
                class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-5 space-y-4 max-w-2xl",
                h2 { class: "text-lg font-medium text-ui-text", "Search assets" }
                p { class: "text-sm text-ui-text-muted",
                    "Case-insensitive partial match on the asset name. Click a result to open its page."
                }
                div {
                    class: "flex flex-col sm:flex-row gap-3 sm:items-end",
                    form {
                        class: "contents",
                        onsubmit: move |ev| {
                            ev.prevent_default();
                            if search_busy() {
                                return;
                            }
                            let base = api_base();
                            let q = search_term().trim().to_string();
                            spawn_search(
                                base,
                                q,
                                search_busy,
                                search_results,
                                recent,
                            );
                        },
                        label {
                            class: "flex-1 flex flex-col gap-1 text-sm text-ui-text-muted",
                            span { "Name contains" }
                            input {
                                class: "bg-ui-bg-dim border border-ui-bg-dim rounded-lg px-3 py-2 text-ui-text focus:outline-none focus:ring-2 focus:ring-ui-secondary",
                                r#type: "search",
                                placeholder: "e.g. toolbox",
                                value: "{search_term}",
                                oninput: move |e| *search_term.write() = e.value(),
                            }
                        }
                        button {
                            r#type: "submit",
                            class: "shrink-0 rounded-lg bg-ui-secondary text-ui-bg px-4 py-2 text-sm font-medium hover:opacity-90 disabled:opacity-50",
                            disabled: search_busy(),
                            if search_busy() { "Searching…" } else { "Search" }
                        }
                    }
                }
                if let Some(res) = search_results() {
                    match res {
                        Ok(assets) if assets.is_empty() => rsx! {
                            p { class: "text-sm text-ui-text-muted", "No matches." }
                        },
                        Ok(assets) => rsx! {
                            div { class: "space-y-3",
                                for it in assets {
                                    Link {
                                        key: "{it.id}",
                                        class: "block rounded-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ui-secondary",
                                        to: Route::AssetDetail { id: it.id },
                                        AssetCard { asset: it }
                                    }
                                }
                            }
                        },
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
                                Link {
                                    class: "shrink-0 rounded-lg border border-ui-bg-dim px-3 py-1.5 text-xs font-medium text-ui-text hover:bg-ui-bg-dim",
                                    to: Route::AssetDetail { id: entry.id },
                                    "Open"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AssetCard(asset: AssetDto) -> Element {
    let parent_note = if asset.parent_id.is_some() {
        "Nested under another asset"
    } else {
        "Top level"
    };
    rsx! {
        div {
            class: "rounded-lg border border-ui-bg-dim bg-ui-bg-dim/40 p-4 space-y-2 text-sm cursor-pointer hover:opacity-95 transition-opacity",
            p { class: "text-base font-medium text-ui-text", "{asset.name}" }
            dl {
                class: "grid grid-cols-[auto_1fr] gap-x-4 gap-y-1 text-ui-text-muted text-xs",
                dt { "Placement" }
                dd { "{parent_note}" }
                dt { "Created" }
                dd { class: "font-mono", "{asset.created}" }
                if asset.deleted.is_some() {
                    dt { "Status" }
                    dd { "Removed" }
                }
            }
        }
    }
}
