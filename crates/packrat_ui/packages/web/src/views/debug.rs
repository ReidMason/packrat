use dioxus::prelude::*;

use crate::api_base::DEFAULT_API_BASE;
use crate::api_client::{self, HealthDto, ReadyDto};

fn spawn_status_refresh(
    base: String,
    mut busy: Signal<bool>,
    mut health: Signal<Option<Result<HealthDto, String>>>,
    mut ready: Signal<Option<Result<ReadyDto, String>>>,
) {
    busy.set(true);
    spawn(async move {
        let (h, r) = api_client::fetch_api_status(&base).await;
        health.set(Some(h));
        ready.set(Some(r));
        busy.set(false);
    });
}

#[component]
pub fn DebugPage() -> Element {
    let api_base = use_context::<Signal<String>>();
    let status_busy = use_signal(|| false);
    let health = use_signal(|| Option::<Result<HealthDto, String>>::None);
    let ready = use_signal(|| Option::<Result<ReadyDto, String>>::None);

    use_hook(move || {
        spawn_status_refresh(api_base(), status_busy, health, ready);
    });

    rsx! {
        div {
            class: "max-w-3xl mx-auto px-4 py-8 space-y-8",

            div {
                class: "space-y-1",
                h1 { class: "text-2xl font-semibold text-ui-text tracking-tight", "Debug" }
                p { class: "text-sm text-ui-text-muted max-w-2xl leading-relaxed",
                    "Liveness and readiness checks against the API base URL in use (read-only here). The main dashboard uses the same value."
                }
            }

            section {
                class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-5 space-y-4",
                h2 { class: "text-sm font-semibold uppercase tracking-wide text-ui-text-muted", "Connection" }
                div {
                    class: "flex flex-col lg:flex-row gap-4 lg:items-end",
                    label {
                        class: "flex-1 flex flex-col gap-1.5 text-sm text-ui-text-muted",
                        span { "API base URL" }
                        input {
                            class: "bg-ui-bg-dim/60 border border-ui-bg-dim rounded-lg px-3 py-2.5 text-ui-text cursor-default",
                            r#type: "url",
                            placeholder: "{DEFAULT_API_BASE}",
                            value: "{api_base}",
                            readonly: true,
                        }
                    }
                    button {
                        class: "shrink-0 rounded-lg bg-ui-primary text-ui-bg px-4 py-2.5 text-sm font-medium hover:opacity-90 disabled:opacity-50",
                        disabled: status_busy(),
                        onclick: move |_| {
                            spawn_status_refresh(api_base(), status_busy, health, ready);
                        },
                        if status_busy() { "Checking…" } else { "Refresh status" }
                    }
                }
            }

            div {
                class: "grid gap-4 sm:grid-cols-2 lg:grid-cols-3",
                StatusCard {
                    title: "Liveness",
                    subtitle: "GET /api/health",
                    body: health(),
                    ok_label: "API up",
                }
                StatusCardReady {
                    title: "Readiness",
                    subtitle: "GET /api/ready",
                    body: ready(),
                    ok_label: "Database reachable",
                }
                div {
                    class: "rounded-xl border border-ui-bg-dim bg-ui-bg-dim/50 p-5 flex flex-col justify-center",
                    p { class: "text-xs font-medium uppercase tracking-wide text-ui-text-muted", "Tip" }
                    p { class: "text-sm text-ui-text mt-2 leading-relaxed",
                        "Run ",
                        code { class: "text-ui-secondary text-xs", "just run-api" },
                        " with Postgres, then refresh status here."
                    }
                }
            }
        }
    }
}

#[component]
fn StatusCard(
    title: String,
    subtitle: String,
    body: Option<Result<HealthDto, String>>,
    ok_label: &'static str,
) -> Element {
    rsx! {
        div {
            class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-5 flex flex-col gap-2",
            p { class: "text-xs font-medium uppercase tracking-wide text-ui-text-muted", "{title}" }
            p { class: "text-xs text-ui-text-dim", "{subtitle}" }
            match body {
                None => rsx! {
                    div { class: "mt-2 h-8 rounded bg-ui-bg-dim animate-pulse" }
                },
                Some(Ok(h)) => rsx! {
                    p { class: "mt-1 text-lg font-semibold text-ui-success", "{h.status}" }
                    p { class: "text-xs text-ui-text-muted", "{ok_label}" }
                },
                Some(Err(e)) => rsx! {
                    p { class: "mt-1 text-sm text-ui-error leading-snug", "{e}" }
                },
            }
        }
    }
}

#[component]
fn StatusCardReady(
    title: String,
    subtitle: String,
    body: Option<Result<ReadyDto, String>>,
    ok_label: &'static str,
) -> Element {
    rsx! {
        div {
            class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-5 flex flex-col gap-2",
            p { class: "text-xs font-medium uppercase tracking-wide text-ui-text-muted", "{title}" }
            p { class: "text-xs text-ui-text-dim", "{subtitle}" }
            match body {
                None => rsx! {
                    div { class: "mt-2 h-8 rounded bg-ui-bg-dim animate-pulse" }
                },
                Some(Ok(r)) => rsx! {
                    p { class: "mt-1 text-lg font-semibold text-ui-success", "{r.status}" }
                    p { class: "text-xs text-ui-text-muted", "DB: {r.database} — {ok_label}" }
                },
                Some(Err(e)) => rsx! {
                    p { class: "mt-1 text-sm text-ui-error leading-snug", "{e}" }
                },
            }
        }
    }
}
