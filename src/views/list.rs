use dioxus::prelude::*;

const LIST_CSS: Asset = asset!("/assets/styles/list.css");

#[component]
pub fn List() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: LIST_CSS }

        header { class: "myheader", "CSS Lists" }
        main {
            article {
                h2 { "Orderded Lists" }
                ol {
                    id: "list1",
                    start: 3,
                    reversed: true,
                    li { "Step One" }
                    li { "Step Two" }
                    li { value: 26, "Step Three" }
                }
            }
            article {
                h2 { "Unordered Lists" }
                ul {
                    id: "list2",
                    li { "Step One" }
                    li { "Step Two" }
                    li { "Step Three" }
                }
            }
        }
    }
}
