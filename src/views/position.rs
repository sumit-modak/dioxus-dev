use dioxus::prelude::*;

const POSITION_CSS: Asset = asset!("/assets/styles/position.css");

#[component]
pub fn Position() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: POSITION_CSS }
        div {
            class: "outer-container",
            div {
                class: "inner-container",
                p {
                    class: "box absolute",
                    "Absolute"
                }
                p {
                    class: "box relative",
                    "Relative"
                }
                p {
                    class: "box fixed",
                    "Fixed"
                }
                p {
                    class: "box sticky",
                    "Sticky"
                }
            }
        }
        div {
            style: "height: 100vh;",
        }
    }
}
