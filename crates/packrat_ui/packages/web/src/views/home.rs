use dioxus::prelude::*;

use super::dashboard::Dashboard;

#[component]
pub fn Home() -> Element {
    rsx! {
        Dashboard {}
    }
}
