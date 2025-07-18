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
            class: "div-blue", style: "left: 270px; top: 860px;"
        }
        div {
            class: "div-white", style: "left: 270px; top: 908px;"
        }
        div {
            class: "div-white", style: "left: 218px; top: 980px;"
        }
        div {
            class: "div-blue", style: "left: 230px; top: 920px;"
        }
        div {
            style: "height: 100vh;",
        }
    }
}
