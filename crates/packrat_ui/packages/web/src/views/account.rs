use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Account() -> Element {
    let active_tenant = use_context::<Signal<Option<i64>>>();

    rsx! {
        div {
            class: "max-w-lg mx-auto px-4 py-10 space-y-6",
            h1 {
                class: "text-2xl font-semibold text-ui-text tracking-tight",
                "Account"
            }
            match active_tenant() {
                Some(id) => rsx! {
                    p { class: "text-sm text-ui-text-muted leading-relaxed",
                        "Current workspace id: {id}."
                    }
                },
                None => rsx! {
                    p { class: "text-sm text-ui-text-muted leading-relaxed",
                        "No workspace is selected. Choose or create one from the sign-in page if you need a workspace."
                    }
                },
            }
            div { class: "flex flex-wrap gap-3",
                Link {
                    class: "inline-flex rounded-lg bg-ui-secondary text-ui-bg px-4 py-2 text-sm font-medium hover:opacity-90",
                    to: Route::Home {},
                    "Back to dashboard"
                }
                Link {
                    class: "inline-flex rounded-lg border border-ui-bg-dim px-4 py-2 text-sm font-medium text-ui-text hover:bg-ui-bg-accent/60",
                    to: Route::Login {},
                    "Sign in again or add a workspace"
                }
            }
        }
    }
}
