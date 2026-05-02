use dioxus::prelude::*;

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
                class: "flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between",
                div {
                    class: "space-y-1 min-w-0",
                    h1 { class: "text-2xl font-semibold text-ui-text tracking-tight", "Debug" }
                    p { class: "text-sm text-ui-text-muted max-w-2xl leading-relaxed",
                        "Liveness and readiness against the same API the app uses (same origin in Docker; localhost when running the API separately)."
                    }
                }
                button {
                    class: "shrink-0 self-start rounded-lg bg-ui-primary text-ui-bg px-4 py-2.5 text-sm font-medium hover:opacity-90 disabled:opacity-50",
                    disabled: status_busy(),
                    onclick: move |_| {
                        spawn_status_refresh(api_base(), status_busy, health, ready);
                    },
                    if status_busy() { "Checking…" } else { "Refresh status" }
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
                        "Ensure Postgres is up (e.g. ",
                        code { class: "text-ui-secondary text-xs", "docker compose up -d postgres" },
                        ") before expecting readiness to pass."
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
