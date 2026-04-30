use dioxus::prelude::*;

#[component]
pub fn TailwindConfig(children: Element) -> Element {
    rsx! {
        document::Stylesheet {
            href: asset!("/assets/tailwind.css"),
        }
        div {
            class: "min-h-screen bg-ui-bg text-ui-text transition-colors duration-500 ease-in-out",
            {children}
        }
    }
}
