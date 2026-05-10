use dioxus::prelude::*;

use super::asset_row::AssetCard;
use crate::api_client;
use crate::Route;

#[component]
pub fn AssetBrowse(tenant_id: i64) -> Element {
    let active_tenant = use_context::<Signal<Option<i64>>>();

    rsx! {
        div {
            class: "max-w-6xl mx-auto px-4 py-8 space-y-8",

            Link {
                class: "inline-block text-sm font-medium text-ui-primary hover:underline",
                to: Route::Home {},
                "← Dashboard"
            }

            div {
                class: "space-y-1",
                h1 { class: "text-2xl font-semibold text-ui-text tracking-tight", "Assets" }
                p { class: "text-sm text-ui-text-muted max-w-2xl leading-relaxed",
                    "Browse everything in this workspace. Root-level assets are listed first. Filter by name when needed."
                }
            }

            if active_tenant().is_none() {
                p { class: "text-sm text-ui-text-muted",
                    "Choose a workspace under Account first."
                }
            } else {
                AssetBrowseBody { tenant_id }
            }
        }
    }
}

#[component]
fn AssetBrowseBody(tenant_id: i64) -> Element {
    let api_base = use_context::<Signal<String>>();
    let auth_token = use_context::<Signal<Option<String>>>();

    let mut search_input = use_signal(String::new);
    // Applied filter: empty lists all assets (GET); non-empty uses fuzzy search (POST).
    let mut applied_filter = use_signal(String::new);

    let list_res = use_resource(move || {
        let q = applied_filter().trim().to_string();
        async move {
            let base = api_base();
            let token = auth_token();
            let tid = tenant_id;
            if q.is_empty() {
                api_client::list_assets(&base, tid, token.as_deref()).await
            } else {
                api_client::search_assets(&base, tid, &q, token.as_deref()).await
            }
        }
    });

    rsx! {
        section {
            class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-5 space-y-4 max-w-3xl",
            form {
                class: "flex flex-col sm:flex-row gap-3 sm:flex-wrap sm:items-end",
                onsubmit: move |ev| {
                    ev.prevent_default();
                    applied_filter.set(search_input().trim().to_string());
                },
                label {
                    class: "flex-1 flex flex-col gap-1 text-sm text-ui-text-muted min-w-[200px]",
                    span { "Filter by name" }
                    input {
                        class: "bg-ui-bg-dim border border-ui-bg-dim rounded-lg px-3 py-2 text-ui-text focus:outline-none focus:ring-2 focus:ring-ui-secondary",
                        r#type: "search",
                        placeholder: "Search...",
                        value: "{search_input}",
                        oninput: move |e| *search_input.write() = e.value(),
                    }
                }
                button {
                    r#type: "submit",
                    class: "shrink-0 rounded-lg bg-ui-secondary text-ui-bg px-4 py-2 text-sm font-medium hover:opacity-90",
                    "Apply filter"
                }
                button {
                    r#type: "button",
                    class: "shrink-0 rounded-lg border border-ui-bg-dim px-4 py-2 text-sm font-medium text-ui-text hover:bg-ui-bg-dim",
                    onclick: move |_| {
                        search_input.write().clear();
                        applied_filter.set(String::new());
                    },
                    "Show all"
                }
            }

            match list_res() {
                None => rsx! {
                    p { class: "text-sm text-ui-text-muted", "Loading…" }
                },
                Some(Err(e)) => rsx! {
                    p { class: "text-sm text-ui-error", "{e}" }
                },
                Some(Ok(assets)) if assets.is_empty() => rsx! {
                    p { class: "text-sm text-ui-text-muted",
                        if applied_filter().trim().is_empty() {
                            "No assets in this workspace yet."
                        } else {
                            "No assets match this filter."
                        }
                    }
                },
                Some(Ok(assets)) => rsx! {
                    p { class: "text-xs text-ui-text-dim",
                        "{assets.len()} asset(s)"
                    }
                    div { class: "space-y-3 max-h-[min(70vh,560px)] overflow-y-auto pr-1",
                        for asset in assets.into_iter() {
                            Link {
                                key: "{asset.id}",
                                class: "block rounded-lg focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ui-secondary",
                                to: Route::AssetDetail { tenant_id: asset.tenant_id, id: asset.id },
                                AssetCard { asset }
                            }
                        }
                    }
                },
            }
        }
    }
}
