use dioxus::prelude::*;

use super::recent_store::{self, RecentBrief};
use crate::api_client::{self, AssetDto, SearchTagsRequest, TagDto};
use crate::Route;

const TAG_SUGGESTION_ROWS: usize = 5;

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
    let mut tag_focused = use_signal(|| false);
    let mut tag_busy = use_signal(|| false);
    let mut tag_msg = use_signal(|| Option::<String>::None);

    let server_tags_for_sync = server_tags.clone();
    use_effect(use_reactive((&revision, &asset_id), move |_| {
        draft.set(server_tags_for_sync.clone());
    }));

    // Signals must be read inside the async future so `use_resource` subscribes (see dioxus use_resource docs).
    let sug_res = use_resource(move || async move {
        let needle = tag_input().trim().to_string();
        let prefix = if needle.is_empty() {
            None
        } else {
            Some(needle)
        };
        let base = api_base();
        let token = auth_token();
        api_client::search_tags(
            &base,
            tenant_id,
            &SearchTagsRequest { prefix },
            token.as_deref(),
        )
        .await
    });

    rsx! {
        div {
            class: "pt-2 border-t border-ui-bg-dim space-y-2",
            h2 {
                class: "text-sm font-semibold text-ui-text",
                "Tags"
            }
            p { class: "text-xs text-ui-text-muted leading-relaxed",
                "Add labels for this asset. Changes apply immediately."
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
                                        let base = api_base();
                                        let token = auth_token();
                                        let ids: Vec<i64> =
                                            draft().iter().map(|t| t.id).collect();
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
                                                Err(e) => {
                                                    tag_msg.set(Some(e));
                                                    on_saved.call(());
                                                }
                                            }
                                            tag_busy.set(false);
                                        });
                                    }
                                },
                                "×"
                            }
                        }
                    }
                }
            }

            div {
                class: "mt-2 flex flex-col gap-2",
                span { class: "text-xs font-medium text-ui-text-muted tracking-wide",
                    "Find or create a tag"
                }
                div {
                    class: "flex gap-1.5 rounded-xl border border-ui-secondary/25 bg-ui-bg-dim/90 p-1 pl-1.5 shadow-[inset_0_1px_2px_rgba(0,0,0,0.18)]",
                    input {
                        class: "min-w-0 flex-1 rounded-lg border-0 bg-transparent px-2.5 py-2 text-sm text-ui-text placeholder:text-ui-text-dim outline-none focus-visible:ring-2 focus-visible:ring-ui-secondary/45 focus-visible:ring-offset-2 focus-visible:ring-offset-ui-bg-dim",
                        placeholder: "Search tags or type a new name…",
                        value: "{tag_input}",
                        onfocus: move |_| tag_focused.set(true),
                        onblur: move |_| tag_focused.set(false),
                        oninput: move |e| *tag_input.write() = e.value(),
                    }
                    button {
                        r#type: "button",
                        class: "shrink-0 self-center rounded-lg bg-ui-secondary px-4 py-2 text-sm font-semibold text-ui-bg shadow-sm shadow-ui-secondary/10 transition hover:brightness-110 active:scale-[0.98] disabled:opacity-45 disabled:hover:brightness-100 disabled:active:scale-100",
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
                                        let ids: Vec<i64> =
                                            draft().iter().map(|t| t.id).collect();
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
                                            Err(e) => {
                                                tag_msg.set(Some(e));
                                                on_saved.call(());
                                            }
                                        }
                                    }
                                    Err(e) => tag_msg.set(Some(e)),
                                }
                                tag_busy.set(false);
                            });
                        },
                    if tag_busy() { "Adding…" } else { "Add" }
                }
            }
            }

            {
                let q = tag_input().trim().to_string();
                let show_suggestions = tag_focused() || !q.is_empty();
                let sug = sug_res();
                let applied_ids: std::collections::HashSet<i64> =
                    draft().iter().map(|t| t.id).collect();
                rsx! {
                    if let Some(Err(e)) = sug.as_ref() {
                        if show_suggestions {
                            p { class: "text-xs text-ui-error", "Could not load tag suggestions: {e}" }
                        }
                    }
                    if show_suggestions {
                        if let Some(Ok(list)) = sug.as_ref() {
                            {
                                let filtered: Vec<TagDto> = list
                                    .iter()
                                    .filter(|s| !applied_ids.contains(&s.id))
                                    .cloned()
                                    .take(TAG_SUGGESTION_ROWS)
                                    .collect();
                                let all_matched_applied =
                                    !list.is_empty() && filtered.is_empty();
                                rsx! {
                                    if all_matched_applied {
                                        p { class: "text-xs leading-relaxed text-ui-text-muted rounded-xl border border-dashed border-ui-secondary/25 bg-ui-bg-dim/40 px-3 py-2",
                                            "Every tag that matches is already on this asset."
                                        }
                                    }
                                    if !filtered.is_empty() {
                                        div {
                                            class: "mt-2 overflow-hidden rounded-xl border border-ui-bg-dim bg-ui-bg-accent shadow-md shadow-black/20",
                                            onmousedown: move |evt| evt.prevent_default(),
                                            div {
                                                class: "border-b border-ui-bg-dim bg-ui-bg-accent px-3 py-2",
                                                p {
                                                    class: "text-xs font-semibold leading-tight text-ui-text",
                                                    if q.is_empty() {
                                                        "Suggestions"
                                                    } else {
                                                        "Matching tags"
                                                    }
                                                }
                                                p {
                                                    class: "mt-0.5 max-w-prose text-[11px] leading-snug text-ui-text-dim",
                                                    "Pick below or type a new name and Add."
                                                }
                                            }
                                            ul {
                                                class: "max-h-36 overflow-y-auto px-1.5 py-1.5",
                                                for s in filtered.iter() {
                                                    li {
                                                        key: "{s.id}",
                                                        class: "py-px",
                                                        button {
                                                            r#type: "button",
                                                            class: "group flex w-full cursor-pointer items-center gap-2 rounded-lg px-2.5 py-2 text-left transition-colors hover:bg-ui-secondary/18 active:bg-ui-secondary/28 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ui-secondary focus-visible:ring-offset-2 focus-visible:ring-offset-ui-bg-accent",
                                                            onclick: {
                                                                let tag = s.clone();
                                                                move |_| {
                                                                    if !draft().iter().any(|x| x.id == tag.id) {
                                                                        draft.with_mut(|v| v.push(tag.clone()));
                                                                    }
                                                                    tag_input.write().clear();
                                                                    let base = api_base();
                                                                    let token = auth_token();
                                                                    let ids: Vec<i64> =
                                                                        draft().iter().map(|t| t.id).collect();
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
                                                                            Err(e) => {
                                                                                tag_msg.set(Some(e));
                                                                                on_saved.call(());
                                                                            }
                                                                        }
                                                                        tag_busy.set(false);
                                                                    });
                                                                }
                                                            },
                                                            span {
                                                                class: "min-w-0 flex-1 truncate text-sm font-medium text-ui-text",
                                                                "{s.name}"
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
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
                            class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-5 space-y-4",
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
                                class: "pt-2 border-t border-ui-bg-dim",
                                h2 {
                                    class: "text-sm font-semibold text-ui-text mb-2",
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
                                class: "pt-2 border-t border-ui-bg-dim space-y-4",

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
