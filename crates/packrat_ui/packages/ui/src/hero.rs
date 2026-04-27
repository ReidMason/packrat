use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            div {
                p {
                    class: "m-2 p-4",
                    "tester p tag here"
                }
                p {
                    class: "m-6 p-2",
                    "tester 2 p tag here"
                }
                p {
                    class: "m-4 p-4",
                    "tester 3 p tag here"
                }
            }
        }
    }
}
