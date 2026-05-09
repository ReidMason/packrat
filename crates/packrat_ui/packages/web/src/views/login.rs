use dioxus::prelude::*;

use super::setup::Setup;

#[component]
pub fn Login() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-ui-bg text-ui-text",
            Setup {}
        }
    }
}
