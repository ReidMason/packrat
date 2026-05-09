use dioxus::prelude::*;
use ui::TailwindConfig;
use views::recent_store;
use views::{Account, AssetDetail, DebugPage, Home, Login, NewAsset};

mod api_base;
mod api_client;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/login")]
    Login {},
    #[layout(AppShell)]
    #[route("/")]
    Home {},
    #[route("/account")]
    Account {},
    #[route("/tenants/:tenant_id/assets/new")]
    NewAsset { tenant_id: i64 },
    #[route("/tenants/:tenant_id/assets/:id")]
    AssetDetail { tenant_id: i64, id: i64 },
    #[route("/debug")]
    DebugPage {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let api_base = use_signal(crate::api_base::initial_api_base);
    let recent = use_signal(|| recent_store::load_recent_disk());
    let auth_token = use_signal(|| crate::api_base::initial_auth_token());
    let active_tenant = use_signal(|| crate::api_base::initial_tenant_id());
    use_context_provider(|| api_base);
    use_context_provider(|| recent);
    use_context_provider(|| auth_token);
    use_context_provider(|| active_tenant);

    rsx! {
        document::Title { "Packrat" }
        document::Link { rel: "icon", href: FAVICON }
        TailwindConfig {
            Router::<Route> {}
        }
    }
}

#[component]
fn AppShell() -> Element {
    let mut auth_token = use_context::<Signal<Option<String>>>();
    let mut active_tenant = use_context::<Signal<Option<i64>>>();
    let nav = use_navigator();

    use_hook(move || {
        if auth_token().is_none() {
            spawn(async move {
                nav.replace(Route::Login {});
            });
        }
    });

    if auth_token().is_none() {
        return rsx! {
            div {
                class: "min-h-screen flex items-center justify-center bg-ui-bg text-ui-text",
                p { class: "text-sm text-ui-text-muted", "Redirecting to sign in…" }
            }
        };
    }

    rsx! {
        div {
            class: "flex min-h-screen",
            aside {
                class: "hidden sm:flex w-52 shrink-0 flex-col border-r border-ui-bg-dim bg-ui-bg-dim/90 py-6 px-4",
                div {
                    class: "text-lg font-semibold text-ui-text tracking-tight",
                    "Packrat"
                }
                p {
                    class: "mt-1 text-xs text-ui-text-muted leading-snug",
                    "Inventory"
                }
                nav {
                    class: "mt-8 flex flex-col gap-1",
                    Link {
                        class: "rounded-lg px-3 py-2 text-sm font-medium text-ui-text hover:bg-ui-bg-accent/60",
                        to: Route::Home {},
                        "Dashboard"
                    }
                    {
                        match active_tenant() {
                            Some(tid) => rsx! {
                                Link {
                                    class: "rounded-lg px-3 py-2 text-sm font-medium text-ui-text hover:bg-ui-bg-accent/60",
                                    to: Route::NewAsset { tenant_id: tid },
                                    "New asset"
                                }
                            },
                            None => rsx! {
                                p {
                                    class: "rounded-lg px-3 py-2 text-sm text-ui-text-muted",
                                    "New asset — add a workspace in Account"
                                }
                            },
                        }
                    }
                    Link {
                        class: "rounded-lg px-3 py-2 text-sm font-medium text-ui-text hover:bg-ui-bg-accent/60",
                        to: Route::Account {},
                        "Account"
                    }
                    Link {
                        class: "rounded-lg px-3 py-2 text-sm font-medium text-ui-text hover:bg-ui-bg-accent/60",
                        to: Route::DebugPage {},
                        "Debug"
                    }
                    button {
                        r#type: "button",
                        class: "mt-4 rounded-lg px-3 py-2 text-sm font-medium text-ui-error hover:bg-ui-error/10 text-left",
                        onclick: move |_| {
                            crate::api_base::clear_auth_token();
                            crate::api_base::clear_tenant_id();
                            *auth_token.write() = None;
                            *active_tenant.write() = None;
                            nav.push(Route::Login {});
                        },
                        "Sign out"
                    }
                }
            }
            div {
                class: "flex-1 flex flex-col min-w-0",
                header {
                    class: "sm:hidden border-b border-ui-bg-dim bg-ui-bg-dim/80 px-4 py-3 flex flex-wrap gap-3",
                    Link {
                        class: "text-sm font-medium text-ui-primary",
                        to: Route::Home {},
                        "Dashboard"
                    }
                    {
                        match active_tenant() {
                            Some(tid) => rsx! {
                                Link {
                                    class: "text-sm font-medium text-ui-text-muted",
                                    to: Route::NewAsset { tenant_id: tid },
                                    "New asset"
                                }
                            },
                            None => rsx! {
                                span { class: "text-sm text-ui-text-muted", "New asset" }
                            },
                        }
                    }
                    Link {
                        class: "text-sm font-medium text-ui-text-muted",
                        to: Route::Account {},
                        "Account"
                    }
                    Link {
                        class: "text-sm font-medium text-ui-text-muted",
                        to: Route::DebugPage {},
                        "Debug"
                    }
                    button {
                        r#type: "button",
                        class: "text-sm font-medium text-ui-error",
                        onclick: move |_| {
                            crate::api_base::clear_auth_token();
                            crate::api_base::clear_tenant_id();
                            *auth_token.write() = None;
                            *active_tenant.write() = None;
                            nav.push(Route::Login {});
                        },
                        "Sign out"
                    }
                }
                main {
                    class: "flex-1 overflow-y-auto",
                    Outlet::<Route> {}
                }
            }
        }
    }
}
