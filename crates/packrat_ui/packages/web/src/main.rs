use dioxus::prelude::*;
use ui::{Navbar, TailwindConfig};
use views::Home;

mod api_client;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home {},
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
fn WebNavbar() -> Element {
    rsx! {
        header {
            class: "border-b border-ui-bg-dim bg-ui-bg-dim/80 backdrop-blur-sm",
            Navbar {
                div {
                    class: "max-w-3xl mx-auto px-4 py-3 flex items-center gap-6 text-sm font-medium text-ui-text",
                    Link {
                        class: "text-ui-primary hover:underline",
                        to: Route::Home {},
                        "Inventory"
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}
