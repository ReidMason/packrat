use dioxus::prelude::*;
use ui::TailwindConfig;
use views::{DebugPage, Home, ItemDetail, NewItem};
use views::recent_store;

mod api_base;
mod api_client;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(AppShell)]
    #[route("/")]
    Home {},
    #[route("/items/new")]
    NewItem {},
    #[route("/items/:id")]
    ItemDetail { id: i64 },
    #[route("/debug")]
    DebugPage {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        TailwindConfig {
            Router::<Route> {}
        }

    }
}

#[component]
fn AppShell() -> Element {
    let api_base = use_signal(crate::api_base::initial_api_base);
    let recent = use_signal(|| recent_store::load_recent_disk());
    use_context_provider(|| api_base);
    use_context_provider(|| recent);

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
                    Link {
                        class: "rounded-lg px-3 py-2 text-sm font-medium text-ui-text hover:bg-ui-bg-accent/60",
                        to: Route::NewItem {},
                        "New item"
                    }
                    Link {
                        class: "rounded-lg px-3 py-2 text-sm font-medium text-ui-text hover:bg-ui-bg-accent/60",
                        to: Route::DebugPage {},
                        "Debug"
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
                    Link {
                        class: "text-sm font-medium text-ui-text-muted",
                        to: Route::NewItem {},
                        "New item"
                    }
                    Link {
                        class: "text-sm font-medium text-ui-text-muted",
                        to: Route::DebugPage {},
                        "Debug"
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
