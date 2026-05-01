use dioxus::prelude::*;

use super::inventory::Inventory;

#[component]
pub fn Home() -> Element {
    rsx! {
        Inventory {}
    }
}
