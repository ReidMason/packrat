use dioxus::prelude::*;

#[component]
pub fn TailwindConfig() -> Element {
    rsx! {
        document::Stylesheet {
            href: asset!("/assets/tailwind.css")
        }
    }
}
