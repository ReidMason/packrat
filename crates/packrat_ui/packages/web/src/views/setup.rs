use dioxus::prelude::*;
use dioxus::router::Navigator;

use crate::api_client::{self, TenantDto, UserDto};
use crate::Route;

#[derive(Clone, Copy, PartialEq, Eq)]
enum SetupPhase {
    Choose,
    SignIn,
    RegisterEmail,
    RegisterWorkspace,
    PickWorkspace,
}

#[derive(Clone, Copy)]
enum PostAuthFlow {
    SignIn,
    Register,
}

async fn resolve_workspaces_after_auth(
    base: String,
    token: String,
    mut phase: Signal<SetupPhase>,
    mut active_tenant: Signal<Option<i64>>,
    mut workspace_options: Signal<Vec<TenantDto>>,
    mut signin_result: Signal<Option<Result<(), String>>>,
    mut workspace_list_error: Signal<Option<String>>,
    navigator: Navigator,
    flow: PostAuthFlow,
) {
    workspace_list_error.set(None);
    match api_client::list_my_tenants(&base, Some(token.as_str())).await {
        Ok(list) if list.is_empty() => {
            crate::api_base::clear_tenant_id();
            *active_tenant.write() = None;
            workspace_options.set(Vec::new());
            phase.set(SetupPhase::RegisterWorkspace);
            if matches!(flow, PostAuthFlow::SignIn) {
                signin_result.set(Some(Ok(())));
            }
        }
        Ok(list) if list.len() == 1 => {
            let t = &list[0];
            crate::api_base::persist_tenant_id(t.id);
            *active_tenant.write() = Some(t.id);
            workspace_options.set(Vec::new());
            navigator.push(Route::Home {});
        }
        Ok(list) => {
            crate::api_base::clear_tenant_id();
            *active_tenant.write() = None;
            workspace_options.set(list);
            phase.set(SetupPhase::PickWorkspace);
            if matches!(flow, PostAuthFlow::SignIn) {
                signin_result.set(None);
            }
        }
        Err(e) => {
            crate::api_base::clear_tenant_id();
            *active_tenant.write() = None;
            workspace_options.set(Vec::new());
            phase.set(SetupPhase::RegisterWorkspace);
            match flow {
                PostAuthFlow::SignIn => {
                    signin_result.set(Some(Err(format!(
                        "Could not load workspaces: {e}. Create one below."
                    ))));
                }
                PostAuthFlow::Register => {
                    workspace_list_error.set(Some(format!(
                        "Could not load workspaces: {e}. Create a workspace below."
                    )));
                }
            }
        }
    }
}

fn spawn_create_user(
    base: String,
    email: String,
    password: String,
    mut busy: Signal<bool>,
    mut result: Signal<Option<Result<UserDto, String>>>,
    phase: Signal<SetupPhase>,
    mut registered_user: Signal<Option<UserDto>>,
    mut auth_token: Signal<Option<String>>,
    active_tenant: Signal<Option<i64>>,
    workspace_options: Signal<Vec<TenantDto>>,
    signin_result: Signal<Option<Result<(), String>>>,
    workspace_list_error: Signal<Option<String>>,
    navigator: Navigator,
) {
    busy.set(true);
    spawn(async move {
        let res = api_client::create_user(&base, email.clone(), password.clone()).await;
        match res {
            Ok(user) => match api_client::login(&base, email, password).await {
                Ok(login) => {
                    *auth_token.write() = Some(login.token.clone());
                    crate::api_base::persist_auth_token(&login.token);
                    registered_user.set(Some(user.clone()));
                    result.set(None);
                    let token = login.token;
                    resolve_workspaces_after_auth(
                        base,
                        token,
                        phase,
                        active_tenant,
                        workspace_options,
                        signin_result,
                        workspace_list_error,
                        navigator,
                        PostAuthFlow::Register,
                    )
                    .await;
                }
                Err(e) => {
                    result.set(Some(Err(format!(
                        "Account created but sign-in failed: {e}"
                    ))));
                }
            },
            Err(e) => result.set(Some(Err(e))),
        }
        busy.set(false);
    });
}

fn spawn_login(
    base: String,
    email: String,
    password: String,
    mut busy: Signal<bool>,
    mut result: Signal<Option<Result<(), String>>>,
    mut auth_token: Signal<Option<String>>,
    phase: Signal<SetupPhase>,
    mut registered_user: Signal<Option<UserDto>>,
    mut tenant_name: Signal<String>,
    mut tenant_result: Signal<Option<Result<TenantDto, String>>>,
    active_tenant: Signal<Option<i64>>,
    workspace_options: Signal<Vec<TenantDto>>,
    workspace_list_error: Signal<Option<String>>,
    navigator: Navigator,
) {
    busy.set(true);
    result.set(None);
    spawn(async move {
        match api_client::login(&base, email, password).await {
            Ok(d) => {
                *auth_token.write() = Some(d.token.clone());
                crate::api_base::persist_auth_token(&d.token);
                registered_user.set(None);
                tenant_name.set(String::new());
                tenant_result.set(None);
                let token = d.token;
                resolve_workspaces_after_auth(
                    base,
                    token,
                    phase,
                    active_tenant,
                    workspace_options,
                    result,
                    workspace_list_error,
                    navigator,
                    PostAuthFlow::SignIn,
                )
                .await;
            }
            Err(e) => result.set(Some(Err(e))),
        }
        busy.set(false);
    });
}

fn spawn_create_tenant(
    base: String,
    name: String,
    token: Option<String>,
    mut busy: Signal<bool>,
    mut result: Signal<Option<Result<TenantDto, String>>>,
    mut active_tenant: Signal<Option<i64>>,
    navigator: Navigator,
) {
    busy.set(true);
    spawn(async move {
        let res = api_client::create_tenant(&base, name, token.as_deref()).await;
        if let Ok(ref t) = res {
            crate::api_base::persist_tenant_id(t.id);
            *active_tenant.write() = Some(t.id);
            navigator.push(Route::Home {});
        }
        result.set(Some(res));
        busy.set(false);
    });
}

fn reset_registration(
    mut phase: Signal<SetupPhase>,
    mut user_email: Signal<String>,
    mut user_password: Signal<String>,
    mut user_result: Signal<Option<Result<UserDto, String>>>,
    mut tenant_name: Signal<String>,
    mut tenant_result: Signal<Option<Result<TenantDto, String>>>,
    mut registered_user: Signal<Option<UserDto>>,
    mut auth_token: Signal<Option<String>>,
    mut active_tenant: Signal<Option<i64>>,
    mut workspace_options: Signal<Vec<TenantDto>>,
    mut workspace_list_error: Signal<Option<String>>,
) {
    phase.set(SetupPhase::Choose);
    user_email.set(String::new());
    user_password.set(String::new());
    user_result.set(None);
    tenant_name.set(String::new());
    tenant_result.set(None);
    registered_user.set(None);
    auth_token.set(None);
    crate::api_base::clear_auth_token();
    crate::api_base::clear_tenant_id();
    *active_tenant.write() = None;
    workspace_options.set(Vec::new());
    workspace_list_error.set(None);
}

#[component]
pub fn Setup() -> Element {
    let api_base = use_context::<Signal<String>>();
    let mut auth_token = use_context::<Signal<Option<String>>>();
    let mut active_tenant = use_context::<Signal<Option<i64>>>();
    let navigator = use_navigator();

    let mut phase = use_signal(|| SetupPhase::Choose);

    let mut signin_email = use_signal(String::new);
    let mut signin_password = use_signal(String::new);
    let signin_busy = use_signal(|| false);
    let mut signin_result = use_signal(|| Option::<Result<(), String>>::None);

    let mut user_email = use_signal(String::new);
    let mut user_password = use_signal(String::new);
    let user_busy = use_signal(|| false);
    let mut user_result = use_signal(|| Option::<Result<UserDto, String>>::None);
    let mut registered_user = use_signal(|| Option::<UserDto>::None);

    let mut tenant_name = use_signal(String::new);
    let tenant_busy = use_signal(|| false);
    let mut tenant_result = use_signal(|| Option::<Result<TenantDto, String>>::None);

    let mut workspace_options = use_signal(Vec::<TenantDto>::new);
    let mut workspace_list_error = use_signal(|| Option::<String>::None);

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
                                signin_password.set(String::new());
                                signin_result.set(None);
                                workspace_options.set(Vec::new());
                                workspace_list_error.set(None);
                                phase.set(SetupPhase::SignIn);
                            },
                            p { class: "text-xs font-medium uppercase tracking-wide text-ui-text-muted", "Returning" }
                            p { class: "mt-2 text-lg font-semibold text-ui-text", "Sign in" }
                            p { class: "mt-2 text-sm text-ui-text-muted leading-snug",
                                "Use your email and password. If you already have workspaces, we open or list them automatically."
                            }
                        }
                        button {
                            class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-6 text-left shadow-sm hover:border-ui-primary/50 transition-colors ring-1 ring-ui-primary/20",
                            onclick: move |_| {
                                user_email.set(String::new());
                                user_password.set(String::new());
                                user_result.set(None);
                                tenant_name.set(String::new());
                                tenant_result.set(None);
                                registered_user.set(None);
                                workspace_options.set(Vec::new());
                                workspace_list_error.set(None);
                                phase.set(SetupPhase::RegisterEmail);
                            },
                            p { class: "text-xs font-medium uppercase tracking-wide text-ui-text-muted", "New here" }
                            p { class: "mt-2 text-lg font-semibold text-ui-text", "Create account" }
                            p { class: "mt-2 text-sm text-ui-text-muted leading-snug",
                                "Register with your email, then name your first workspace (or pick one if you were invited)."
                            }
                        }
                    }
                },
                SetupPhase::SignIn => rsx! {
                    div { class: "space-y-6",
                        button {
                            class: "text-sm text-ui-text-muted hover:text-ui-text",
                            onclick: move |_| {
                                signin_result.set(None);
                                workspace_options.set(Vec::new());
                                workspace_list_error.set(None);
                                phase.set(SetupPhase::Choose);
                            },
                            "← Back"
                        }
                        div {
                            class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-6 space-y-4",
                            h2 { class: "text-lg font-medium text-ui-text", "Sign in" }
                            p { class: "text-sm text-ui-text-muted",
                                "Use the same credentials as the Packrat API. If you have no workspace yet, you can create one after sign-in."
                            }
                            form {
                                class: "space-y-3",
                                onsubmit: move |ev| {
                                    ev.prevent_default();
                                    if signin_busy() {
                                        return;
                                    }
                                    let email = signin_email().trim().to_string();
                                    let password = signin_password();
                                    if email.is_empty() || !email.contains('@') {
                                        signin_result.set(Some(Err("Enter a valid email.".into())));
                                        return;
                                    }
                                    if password.len() < 8 {
                                        signin_result.set(Some(Err("Password must be at least 8 characters.".into())));
                                        return;
                                    }
                                    spawn_login(
                                        api_base(),
                                        email,
                                        password,
                                        signin_busy,
                                        signin_result,
                                        auth_token,
                                        phase,
                                        registered_user,
                                        tenant_name,
                                        tenant_result,
                                        active_tenant,
                                        workspace_options,
                                        workspace_list_error,
                                        navigator,
                                    );
                                },
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
                                        class: "bg-ui-bg-dim border border-ui-bg-dim rounded-lg px-3 py-2 text-ui-text focus:outline-none focus:ring-2 focus:ring-ui-secondary",
                                        r#type: "password",
                                        placeholder: "Your password",
                                        value: "{signin_password}",
                                        oninput: move |e| *signin_password.write() = e.value(),
                                    }
                                }
                                if let Some(Err(ref e)) = signin_result() {
                                    p { class: "text-sm text-ui-error", "{e}" }
                                }
                                button {
                                    r#type: "submit",
                                    class: "w-full rounded-lg bg-ui-secondary text-ui-bg px-4 py-2.5 text-sm font-medium hover:opacity-90 disabled:opacity-50",
                                    disabled: signin_busy(),
                                    if signin_busy() { "Signing in…" } else { "Sign in" }
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
                                    user_password,
                                    user_result,
                                    tenant_name,
                                    tenant_result,
                                    registered_user,
                                    auth_token,
                                    active_tenant,
                                    workspace_options,
                                    workspace_list_error,
                                );
                            },
                            "← Back"
                        }
                        div {
                            class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-6 space-y-4",
                            h2 { class: "text-lg font-medium text-ui-text", "Create your account" }
                            p { class: "text-sm text-ui-text-muted",
                                "Email and password are sent to the API (TLS in production). Next you will pick or create a workspace."
                            }
                            form {
                                class: "space-y-4",
                                onsubmit: move |ev| {
                                    ev.prevent_default();
                                    if user_busy() {
                                        return;
                                    }
                                    let email = user_email().trim().to_string();
                                    let password = user_password();
                                    if email.is_empty() || !email.contains('@') {
                                        user_result.set(Some(Err("Enter a valid email.".into())));
                                        return;
                                    }
                                    if password.len() < 8 {
                                        user_result.set(Some(Err("Password must be at least 8 characters.".into())));
                                        return;
                                    }
                                    user_result.set(None);
                                    spawn_create_user(
                                        api_base(),
                                        email,
                                        password,
                                        user_busy,
                                        user_result,
                                        phase,
                                        registered_user,
                                        auth_token,
                                        active_tenant,
                                        workspace_options,
                                        signin_result,
                                        workspace_list_error,
                                        navigator,
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
                                label {
                                    class: "flex flex-col gap-1 text-sm text-ui-text-muted",
                                    span { "Password" }
                                    input {
                                        class: "bg-ui-bg-dim border border-ui-bg-dim rounded-lg px-3 py-2 text-ui-text focus:outline-none focus:ring-2 focus:ring-ui-secondary",
                                        r#type: "password",
                                        placeholder: "At least 8 characters",
                                        value: "{user_password}",
                                        oninput: move |e| *user_password.write() = e.value(),
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
                SetupPhase::PickWorkspace => rsx! {
                    div { class: "space-y-6",
                        button {
                            class: "text-sm text-ui-text-muted hover:text-ui-text",
                            onclick: move |_| {
                                auth_token.set(None);
                                crate::api_base::clear_auth_token();
                                crate::api_base::clear_tenant_id();
                                *active_tenant.write() = None;
                                workspace_options.set(Vec::new());
                                workspace_list_error.set(None);
                                signin_result.set(None);
                                phase.set(SetupPhase::Choose);
                            },
                            "← Back"
                        }
                        div {
                            class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-6 space-y-4",
                            h2 { class: "text-lg font-medium text-ui-text", "Choose a workspace" }
                            p { class: "text-sm text-ui-text-muted",
                                "You have access to more than one. Pick one to open the dashboard."
                            }
                            div { class: "space-y-2",
                                for t in workspace_options() {
                                    button {
                                        r#type: "button",
                                        class: "w-full text-left rounded-lg border border-ui-bg-dim bg-ui-bg-dim/50 px-4 py-3 text-sm font-medium text-ui-text hover:border-ui-secondary/40 transition-colors",
                                        onclick: move |_| {
                                            crate::api_base::persist_tenant_id(t.id);
                                            *active_tenant.write() = Some(t.id);
                                            workspace_options.set(Vec::new());
                                            navigator.push(Route::Home {});
                                        },
                                        "{t.name}"
                                    }
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
                                user_password.set(String::new());
                                phase.set(SetupPhase::RegisterEmail);
                                tenant_name.set(String::new());
                                tenant_result.set(None);
                                user_result.set(None);
                                workspace_options.set(Vec::new());
                                workspace_list_error.set(None);
                            },
                            "← Change account"
                        }
                        div {
                            class: "rounded-xl border border-ui-bg-dim bg-ui-bg-accent p-6 space-y-4",
                            h2 { class: "text-lg font-medium text-ui-text", "Name your workspace" }
                            if matches!(signin_result(), Some(Ok(()))) && registered_user().is_none() {
                                p { class: "text-sm text-ui-success",
                                    "Signed in. Name your workspace below."
                                }
                            }
                            if let Some(ref msg) = workspace_list_error() {
                                p { class: "text-sm text-ui-error", "{msg}" }
                            }
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
                                    if let Some(Ok(ref t)) = tenant_result() {
                                        if let Some(ref u) = registered_user() {
                                            p { class: "text-sm text-ui-text-muted",
                                                "Workspace “{t.name}” is ready. Signed in as {u.email}. Your session is stored in this browser for API calls."
                                            }
                                        } else {
                                            p { class: "text-sm text-ui-text-muted",
                                                "Workspace “{t.name}” is ready."
                                            }
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
                                        if auth_token().is_none() {
                                            tenant_result.set(Some(Err(
                                                "Not signed in. Go back and sign in again.".into(),
                                            )));
                                            return;
                                        }
                                        tenant_result.set(None);
                                        spawn_create_tenant(
                                            api_base(),
                                            name,
                                            auth_token(),
                                            tenant_busy,
                                            tenant_result,
                                            active_tenant,
                                            navigator,
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
