use dioxus::prelude::*;

const PAGES_CSS: Asset = asset!("/assets/styles/pages.css");

#[component]
pub fn Pages() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: PAGES_CSS }

        button { class: "social", }
        section {
            id: "one",
            header {
                class: "pageheader red",
                "Header One"
            }
            h2 {
                "One"
            }
        }
        section {
            id: "two",
            header {
                class: "pageheader green",
                "Header Two"
            }
            h2 {
                "two"
            }
        }
        section {
            id: "three",
            header {
                class: "pageheader blue",
                "Header Three"
            }
            h2 {
                "three"
            }
        }
        footer {
            class: "pagelinks",
            Link { to: "#one", "One" }
            " | "
            Link { to: "#two", "Two" }
            " | "
            Link { to: "#three", "Three" }
        }
    }
}
