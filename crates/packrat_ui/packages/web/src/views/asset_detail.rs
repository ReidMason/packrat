use dioxus::prelude::*;

use crate::Route;
use crate::api_client::{self, AssetDto};
use super::recent_store::{self, RecentBrief};

/// Asset detail is driven by the URL, not only the `id` prop from the router outlet: the outlet can
/// reuse the same component slot when only `/assets/:id` changes, so we read [`Route`] from the
/// router (subscribes to history) for [`use_resource`] and for this scope’s render subscription.
#[component]
#[allow(unused_variables)] // `id` comes from the router; we read the active segment from history.
pub fn AssetDetail(id: i64) -> Element {
    let _ = use_route::<Route>();

    let api_base = use_context::<Signal<String>>();
    let recent = use_context::<Signal<Vec<RecentBrief>>>();
    let navigator = use_navigator();

    let detail_res = use_resource(move || {
        let api_base_sig = api_base;
        async move {
            let router = try_router().ok_or_else(|| "router unavailable".to_string())?;
            let asset_id = match router.current::<Route>() {
                Route::AssetDetail { id } => id,
                _ => return Err("unexpected route".into()),
            };
            let base = api_base_sig();
            let asset = api_client::get_asset(&base, asset_id).await?;
            let children = api_client::list_child_assets(&base, asset_id).await?;
            Ok::<(AssetDto, Vec<AssetDto>), String>((asset, children))
        }
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

            match detail_res() {
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
                Some(Ok((asset, children))) => {
                    let asset_id = asset.id;
                    let name = asset.name.clone();
                    let parent_note = if asset.parent_id.is_some() {
                        "Nested under another asset"
                    } else {
                        "Top level"
                    };
                    let created = asset.created.clone();
                    let deleted = asset.deleted.is_some();
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
                                class: "pt-4 border-t border-ui-bg-dim",
                                h2 {
                                    class: "text-sm font-semibold text-ui-text mb-3",
                                    "Nested assets"
                                }
                                if children.is_empty() {
                                    p { class: "text-sm text-ui-text-muted", "None — nothing is filed under this asset yet." }
                                } else {
                                    ul {
                                        class: "divide-y divide-ui-bg-dim rounded-lg border border-ui-bg-dim bg-ui-bg-dim/30",
                                        for child in children {
                                            li {
                                                key: "{child.id}",
                                                class: "flex items-center justify-between gap-3 px-4 py-3 first:rounded-t-lg last:rounded-b-lg",
                                                Link {
                                                    class: "min-w-0 flex-1 text-sm font-medium text-ui-primary hover:underline truncate",
                                                    to: Route::AssetDetail { id: child.id },
                                                    "{child.name}"
                                                }
                                            }
                                        }
                                    }
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
                                                    match api_client::delete_asset(&base, asset_id).await {
                                                        Ok(()) => {
                                                            recent_store::remove_recent(rec, asset_id);
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
                                        "Delete asset"
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
