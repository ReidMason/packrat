use dioxus::prelude::*;

use super::recent_store::{self, RecentBrief};
use crate::api_client::{self, AssetDto, SearchTagsRequest, TagDto};
use crate::Route;

#[component]
fn AssetTagsSection(
    tenant_id: i64,
    asset_id: i64,
    server_tags: Vec<TagDto>,
    revision: u32,
    on_saved: EventHandler<()>,
) -> Element {
    let api_base = use_context::<Signal<String>>();
    let auth_token = use_context::<Signal<Option<String>>>();

    let mut draft = use_signal(Vec::<TagDto>::new);
    let mut tag_input = use_signal(String::new);
    let mut tag_busy = use_signal(|| false);
    let mut tag_msg = use_signal(|| Option::<String>::None);

    let server_tags_for_sync = server_tags.clone();
    use_effect(use_reactive((&revision, &asset_id), move |_| {
        draft.set(server_tags_for_sync.clone());
    }));

    let sug_res = use_resource({
        let api_base_sig = api_base;
        let token_sig = auth_token;
        move || {
            let needle = tag_input().trim().to_string();
            let prefix = if needle.is_empty() {
                None
            } else {
                Some(needle.clone())
            };
            let base = api_base_sig();
            let token = token_sig();
            let tid = tenant_id;
            async move {
                api_client::search_tags(&base, tid, &SearchTagsRequest { prefix }, token.as_deref())
                    .await
            }
        }
    });

    rsx! {
        div {
            class: "pt-4 border-t border-ui-bg-dim space-y-4",
            h2 {
                class: "text-sm font-semibold text-ui-text mb-2",
                "Tags"
            }
            p { class: "text-xs text-ui-text-muted leading-relaxed",
                "Add labels for this asset. Save applies your changes to the server."
            }

            if !draft().is_empty() {
                div {
                    class: "flex flex-wrap gap-2",
                    for t in draft().into_iter() {
                        span {
                            key: "{t.id}",
                            class: "inline-flex items-center gap-1.5 rounded-full border border-ui-bg-dim bg-ui-bg-dim/50 px-3 py-1 text-xs font-medium text-ui-text",
                            span { "{t.name}" }
                            button {
                                r#type: "button",
                                class: "text-ui-text-muted hover:text-ui-error",
                                disabled: tag_busy(),
                                onclick: {
                                    let rm_id = t.id;
                                    move |_| {
                                        draft.with_mut(|v| v.retain(|x| x.id != rm_id));
                                    }
                                },
                                "×"
                            }
                        }
                    }
                }
            }

            div {
                class: "flex flex-col sm:flex-row gap-2 sm:items-end",
                label {
                    class: "flex-1 flex flex-col gap-1.5 text-xs text-ui-text-muted",
                    span { "Add tag" }
                    input {
                        class: "bg-ui-bg-dim border border-ui-bg-dim rounded-lg px-3 py-2 text-sm text-ui-text focus:outline-none focus:ring-2 focus:ring-ui-secondary",
                        placeholder: "Type to search or create…",
                        value: "{tag_input}",
                        oninput: move |e| *tag_input.write() = e.value(),
                    }
                }
                button {
                    r#type: "button",
                    class: "shrink-0 rounded-lg bg-ui-secondary text-ui-bg px-4 py-2 text-sm font-medium hover:opacity-90 disabled:opacity-50",
                    disabled: tag_busy(),
                    onclick: move |_| {
                        let base = api_base();
                        let token = auth_token();
                        let raw = tag_input().trim().to_string();
                        if raw.is_empty() {
                            return;
                        }
                        if draft().iter().any(|x| x.name.eq_ignore_ascii_case(&raw)) {
                            tag_input.write().clear();
                            return;
                        }
                        let pick = sug_res()
                            .and_then(|r| r.clone().ok())
                            .and_then(|list| {
                                list.into_iter()
                                    .find(|x| x.name.eq_ignore_ascii_case(&raw))
                            });
                        tag_busy.set(true);
                        tag_msg.set(None);
                        spawn(async move {
                            let res = if let Some(t) = pick {
                                Ok(t)
                            } else {
                                api_client::ensure_tag(&base, tenant_id, raw.clone(), token.as_deref())
                                    .await
                            };
                            match res {
                                Ok(t) => {
                                    draft.with_mut(|v| {
                                        if !v.iter().any(|x| x.id == t.id) {
                                            v.push(t);
                                        }
                                    });
                                    tag_input.write().clear();
                                }
                                Err(e) => tag_msg.set(Some(e)),
                            }
                            tag_busy.set(false);
                        });
                    },
                    if tag_busy() { "Adding…" } else { "Add" }
                }
            }

            match sug_res() {
                Some(Ok(list)) if !list.is_empty() && !tag_input().trim().is_empty() => rsx! {
                    div {
                        class: "rounded-lg border border-ui-bg-dim bg-ui-bg-dim/30 px-3 py-2 text-xs text-ui-text-muted max-h-36 overflow-y-auto",
                        p { class: "text-[11px] uppercase tracking-wide mb-1.5", "Matching tags" }
                        ul { class: "space-y-1",
                            for s in list.iter().take(12) {
                                li {
                                    key: "{s.id}",
                                    button {
                                        r#type: "button",
                                        class: "w-full text-left rounded px-2 py-1 hover:bg-ui-bg-dim/80 text-ui-text",
                                        onclick: {
                                            let tag = s.clone();
                                            move |_| {
                                                if !draft().iter().any(|x| x.id == tag.id) {
                                                    draft.with_mut(|v| v.push(tag.clone()));
                                                }
                                                tag_input.write().clear();
                                            }
                                        },
                                        "{s.name}"
                                    }
                                }
                            }
                        }
                    }
                },
                _ => rsx! {},
            }

            button {
                r#type: "button",
                class: "rounded-lg border border-ui-bg-dim px-4 py-2 text-sm font-medium text-ui-text hover:bg-ui-bg-dim disabled:opacity-50",
                disabled: tag_busy(),
                onclick: move |_| {
                    let base = api_base();
                    let token = auth_token();
                    let ids: Vec<i64> = draft().iter().map(|t| t.id).collect();
                    tag_busy.set(true);
                    tag_msg.set(None);
                    spawn(async move {
                        match api_client::set_asset_tags(
                                &base,
                                tenant_id,
                                asset_id,
                                ids,
                                token.as_deref(),
                            )
                            .await
                        {
                            Ok(()) => {
                                on_saved.call(());
                            }
                            Err(e) => tag_msg.set(Some(e)),
                        }
                        tag_busy.set(false);
                    });
                },
                if tag_busy() { "Saving…" } else { "Save tags" }
            }

            if let Some(m) = tag_msg() {
                p { class: "text-sm text-ui-error", "{m}" }
            }
        }
    }
}

#[component]
#[allow(unused_variables)]
pub fn AssetDetail(tenant_id: i64, id: i64) -> Element {
    let _ = use_route::<Route>();

    let api_base = use_context::<Signal<String>>();
    let auth_token = use_context::<Signal<Option<String>>>();
    let recent = use_context::<Signal<Vec<RecentBrief>>>();
    let navigator = use_navigator();

    let mut refresh_gen = use_signal(|| 0u32);
    let detail_res = use_resource(move || {
        let _gen = refresh_gen();
        let api_base_sig = api_base;
        let token_sig = auth_token;
        let tid = tenant_id;
        async move {
            let router = try_router().ok_or_else(|| "router unavailable".to_string())?;
            let (route_tid, asset_id) = match router.current::<Route>() {
                Route::AssetDetail { tenant_id, id } => (tenant_id, id),
                _ => return Err("unexpected route".into()),
            };
            let base = api_base_sig();
            let token = token_sig();
            let asset = api_client::get_asset(&base, route_tid, asset_id, token.as_deref()).await?;
            let children =
                api_client::list_child_assets(&base, route_tid, asset_id, token.as_deref()).await?;
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
                    let route_tid = asset.tenant_id;
                    let name = asset.name.clone();
                    let parent_note = if asset.parent_id.is_some() {
                        "Nested under another asset"
                    } else {
                        "Top level"
                    };
                    let created = asset.created.clone();
                    let deleted = asset.deleted.is_some();
                    let tags_snapshot = asset.tags.clone();
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

                            AssetTagsSection {
                                tenant_id: route_tid,
                                asset_id,
                                server_tags: tags_snapshot,
                                revision: refresh_gen(),
                                on_saved: move |_| {
                                    refresh_gen.set(refresh_gen() + 1);
                                },
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
                                                    to: Route::AssetDetail { tenant_id: child.tenant_id, id: child.id },
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
                                                let token = auth_token();
                                                let nav = navigator;
                                                let rec = recent;
                                                delete_busy.set(true);
                                                delete_msg.set(None);
                                                spawn(async move {
                                                    match api_client::delete_asset(
                                                            &base,
                                                            route_tid,
                                                            asset_id,
                                                            token.as_deref(),
                                                        )
                                                        .await
                                                    {
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
