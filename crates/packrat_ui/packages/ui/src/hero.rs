use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            div {
                p {
                    class: "m-2 p-4 bg-primary text-red",
                    "tester p tag here"
                }
                p {
                    class: "m-6 p-2 bg-primary text-red",
                    "tester 2 p tag here"
                }
                p {
                    class: "m-4 p-4 bg-primary text-red",
                    "tester 3 p tag here"
                }
            }
        }
    }
}
