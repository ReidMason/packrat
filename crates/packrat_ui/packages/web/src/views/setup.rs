use dioxus::prelude::*;

use crate::api_client::{self, TenantDto, UserDto};
use crate::Route;

#[derive(Clone, Copy, PartialEq, Eq)]
enum SetupPhase {
    Choose,
    SignIn,
    RegisterEmail,
    RegisterWorkspace,
}

fn spawn_create_user(
    base: String,
    email: String,
    mut busy: Signal<bool>,
    mut result: Signal<Option<Result<UserDto, String>>>,
    mut phase: Signal<SetupPhase>,
    mut registered_user: Signal<Option<UserDto>>,
) {
    busy.set(true);
    spawn(async move {
        let res = api_client::create_user(&base, email).await;
        if let Ok(ref user) = res {
            registered_user.set(Some(user.clone()));
            phase.set(SetupPhase::RegisterWorkspace);
            result.set(None);
        } else {
            result.set(Some(res));
        }
        busy.set(false);
    });
}

fn spawn_create_tenant(
    base: String,
    name: String,
    mut busy: Signal<bool>,
    mut result: Signal<Option<Result<TenantDto, String>>>,
) {
    busy.set(true);
    spawn(async move {
        let res = api_client::create_tenant(&base, name).await;
        result.set(Some(res));
        busy.set(false);
    });
}

fn reset_registration(
    mut phase: Signal<SetupPhase>,
    mut user_email: Signal<String>,
    mut user_result: Signal<Option<Result<UserDto, String>>>,
    mut tenant_name: Signal<String>,
    mut tenant_result: Signal<Option<Result<TenantDto, String>>>,
    mut registered_user: Signal<Option<UserDto>>,
) {
    phase.set(SetupPhase::Choose);
    user_email.set(String::new());
    user_result.set(None);
    tenant_name.set(String::new());
    tenant_result.set(None);
    registered_user.set(None);
}

/// Onboarding: sign in (placeholder) or register → account email → workspace (tenant).
#[component]
pub fn Setup() -> Element {
    let api_base = use_context::<Signal<String>>();

    let mut phase = use_signal(|| SetupPhase::Choose);

    let mut signin_email = use_signal(String::new);

    let mut user_email = use_signal(String::new);
    let user_busy = use_signal(|| false);
    let mut user_result = use_signal(|| Option::<Result<UserDto, String>>::None);
    let mut registered_user = use_signal(|| Option::<UserDto>::None);

    let mut tenant_name = use_signal(String::new);
    let tenant_busy = use_signal(|| false);
    let mut tenant_result = use_signal(|| Option::<Result<TenantDto, String>>::None);

    rsx! {
        div {
            class: "max-w-lg mx-auto px-4 py-10 space-y-8",

            div {
                class: "space-y-2 text-center",
                h1 { class: "text-2xl font-semibold text-ui-text tracking-tight", "Set up Packrat" }
                p { class: "text-sm text-ui-text-muted leading-relaxed",
                    "Sign in if you already have an account, or create one and a workspace to get started."
                }
            }

            {match phase() {
                SetupPhase::Choose => rsx! {
                    div { class: "grid gap-4 sm:grid-cols-2",
                        button {
                            class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-6 text-left shadow-sm hover:border-ui-secondary/40 transition-colors",
                            onclick: move |_| {
                                signin_email.set(String::new());
                                phase.set(SetupPhase::SignIn);
                            },
                            p { class: "text-xs font-medium uppercase tracking-wide text-ui-text-muted", "Returning" }
                            p { class: "mt-2 text-lg font-semibold text-ui-text", "Sign in" }
                            p { class: "mt-2 text-sm text-ui-text-muted leading-snug",
                                "Use your existing account. Password auth is not connected yet — this step is a preview."
                            }
                        }
                        button {
                            class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-6 text-left shadow-sm hover:border-ui-primary/50 transition-colors ring-1 ring-ui-primary/20",
                            onclick: move |_| {
                                user_email.set(String::new());
                                user_result.set(None);
                                tenant_name.set(String::new());
                                tenant_result.set(None);
                                registered_user.set(None);
                                phase.set(SetupPhase::RegisterEmail);
                            },
                            p { class: "text-xs font-medium uppercase tracking-wide text-ui-text-muted", "New here" }
                            p { class: "mt-2 text-lg font-semibold text-ui-text", "Create account" }
                            p { class: "mt-2 text-sm text-ui-text-muted leading-snug",
                                "Register with your email, then name your first workspace."
                            }
                        }
                    }
                },
                SetupPhase::SignIn => rsx! {
                    div { class: "space-y-6",
                        button {
                            class: "text-sm text-ui-text-muted hover:text-ui-text",
                            onclick: move |_| phase.set(SetupPhase::Choose),
                            "← Back"
                        }
                        div {
                            class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-6 space-y-4",
                            h2 { class: "text-lg font-medium text-ui-text", "Sign in" }
                            p { class: "text-sm text-ui-text-muted",
                                "The API does not authenticate passwords yet. When it does, this form will talk to your session endpoint."
                            }
                            div {
                                class: "space-y-3",
                                label {
                                    class: "flex flex-col gap-1 text-sm text-ui-text-muted",
                                    span { "Email" }
                                    input {
                                        class: "bg-ui-bg-dim border border-ui-bg-dim rounded-lg px-3 py-2 text-ui-text focus:outline-none focus:ring-2 focus:ring-ui-secondary",
                                        r#type: "email",
                                        placeholder: "you@example.com",
                                        value: "{signin_email}",
                                        oninput: move |e| *signin_email.write() = e.value(),
                                    }
                                }
                                label {
                                    class: "flex flex-col gap-1 text-sm text-ui-text-muted",
                                    span { "Password" }
                                    input {
                                        class: "bg-ui-bg-dim/60 border border-ui-bg-dim rounded-lg px-3 py-2 text-ui-text-muted cursor-not-allowed",
                                        r#type: "password",
                                        placeholder: "Not available yet",
                                        disabled: true,
                                    }
                                }
                                button {
                                    r#type: "button",
                                    class: "w-full rounded-lg bg-ui-bg-dim text-ui-text-muted px-4 py-2.5 text-sm font-medium cursor-not-allowed",
                                    disabled: true,
                                    "Sign in (coming soon)"
                                }
                            }
                        }
                    }
                },
                SetupPhase::RegisterEmail => rsx! {
                    div { class: "space-y-6",
                        button {
                            class: "text-sm text-ui-text-muted hover:text-ui-text",
                            onclick: move |_| {
                                reset_registration(
                                    phase,
                                    user_email,
                                    user_result,
                                    tenant_name,
                                    tenant_result,
                                    registered_user,
                                );
                            },
                            "← Back"
                        }
                        div {
                            class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-6 space-y-4",
                            h2 { class: "text-lg font-medium text-ui-text", "Create your account" }
                            p { class: "text-sm text-ui-text-muted",
                                "We only need your email for now. Next you will create a workspace."
                            }
                            form {
                                class: "space-y-4",
                                onsubmit: move |ev| {
                                    ev.prevent_default();
                                    if user_busy() {
                                        return;
                                    }
                                    let email = user_email().trim().to_string();
                                    if email.is_empty() || !email.contains('@') {
                                        user_result.set(Some(Err("Enter a valid email.".into())));
                                        return;
                                    }
                                    user_result.set(None);
                                    spawn_create_user(
                                        api_base(),
                                        email,
                                        user_busy,
                                        user_result,
                                        phase,
                                        registered_user,
                                    );
                                },
                                label {
                                    class: "flex flex-col gap-1 text-sm text-ui-text-muted",
                                    span { "Email" }
                                    input {
                                        class: "bg-ui-bg-dim border border-ui-bg-dim rounded-lg px-3 py-2 text-ui-text focus:outline-none focus:ring-2 focus:ring-ui-secondary",
                                        r#type: "email",
                                        placeholder: "you@example.com",
                                        value: "{user_email}",
                                        oninput: move |e| *user_email.write() = e.value(),
                                    }
                                }
                                if let Some(Err(ref e)) = user_result() {
                                    p { class: "text-sm text-ui-error", "{e}" }
                                }
                                button {
                                    r#type: "submit",
                                    class: "w-full rounded-lg bg-ui-primary text-ui-bg px-4 py-2.5 text-sm font-medium hover:opacity-90 disabled:opacity-50",
                                    disabled: user_busy(),
                                    if user_busy() { "Creating account…" } else { "Continue" }
                                }
                            }
                        }
                    }
                },
                SetupPhase::RegisterWorkspace => rsx! {
                    div { class: "space-y-6",
                        button {
                            class: "text-sm text-ui-text-muted hover:text-ui-text",
                            onclick: move |_| {
                                if let Some(u) = registered_user() {
                                    user_email.set(u.email.clone());
                                }
                                registered_user.set(None);
                                phase.set(SetupPhase::RegisterEmail);
                                tenant_name.set(String::new());
                                tenant_result.set(None);
                                user_result.set(None);
                            },
                            "← Change email"
                        }
                        div {
                            class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-6 space-y-4",
                            h2 { class: "text-lg font-medium text-ui-text", "Name your workspace" }
                            p { class: "text-sm text-ui-text-muted",
                                if let Some(ref u) = registered_user() {
                                    "Account ready for {u.email}. Create a tenant for your inventory — you can invite others later."
                                } else {
                                    "Create a workspace for your inventory."
                                }
                            }
                            if tenant_result().as_ref().and_then(|r| r.as_ref().ok()).is_some() {
                                div { class: "rounded-lg border border-ui-success/40 bg-ui-success/10 p-4 space-y-3",
                                    p { class: "text-sm font-medium text-ui-success", "You are set up." }
                                    if let (Some(Ok(t)), Some(u)) = (tenant_result(), registered_user()) {
                                        p { class: "text-sm text-ui-text-muted",
                                            "Workspace “{t.name}” is ready. Signed in as {u.email} (session not stored in browser yet)."
                                        }
                                    }
                                    Link {
                                        class: "inline-flex rounded-lg bg-ui-primary text-ui-bg px-4 py-2 text-sm font-medium hover:opacity-90",
                                        to: Route::Home {},
                                        "Go to dashboard"
                                    }
                                }
                            } else {
                                form {
                                    class: "space-y-4",
                                    onsubmit: move |ev| {
                                        ev.prevent_default();
                                        if tenant_busy() {
                                            return;
                                        }
                                        let name = tenant_name().trim().to_string();
                                        if name.is_empty() {
                                            tenant_result.set(Some(Err("Name must not be empty.".into())));
                                            return;
                                        }
                                        tenant_result.set(None);
                                        spawn_create_tenant(
                                            api_base(),
                                            name,
                                            tenant_busy,
                                            tenant_result,
                                        );
                                    },
                                    label {
                                        class: "flex flex-col gap-1 text-sm text-ui-text-muted",
                                        span { "Workspace name" }
                                        input {
                                            class: "bg-ui-bg-dim border border-ui-bg-dim rounded-lg px-3 py-2 text-ui-text focus:outline-none focus:ring-2 focus:ring-ui-secondary",
                                            r#type: "text",
                                            placeholder: "e.g. Home inventory",
                                            value: "{tenant_name}",
                                            oninput: move |e| *tenant_name.write() = e.value(),
                                        }
                                    }
                                    if let Some(Err(ref e)) = tenant_result() {
                                        p { class: "text-sm text-ui-error", "{e}" }
                                    }
                                    button {
                                        r#type: "submit",
                                        class: "w-full rounded-lg bg-ui-secondary text-ui-bg px-4 py-2.5 text-sm font-medium hover:opacity-90 disabled:opacity-50",
                                        disabled: tenant_busy(),
                                        if tenant_busy() { "Creating workspace…" } else { "Create workspace" }
                                    }
                                }
                            }
                        }
                    }
                },
            }}
        }
    }
}
