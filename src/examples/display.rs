use dioxus::prelude::*;

const DISPLAY_CSS: Asset = asset!("/assets/styles/display.css");

#[component]
pub fn Display() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: DISPLAY_CSS }

        main {
            p {
                class: "uniqueblock",
                style: "text-align: right;",
                "This is a paragraph"
            }
            p {
                class: "uniqueblock",
                "This is "
                span {
                    class: "opposite-inline",
                    "another"
                }
                "paragraph"
            }
            p {
                class: "uniqueblock",
                "This is "
                span {
                    class: "opposite-inline-block",
                    "another fucking"
                }
                "paragraph"
            }
        }
    }
}
