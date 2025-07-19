use dioxus::prelude::*;

const FLEXBOX_CSS: Asset = asset!("/assets/styles/flexbox.css");

#[component]
pub fn FlexBox() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: FLEXBOX_CSS }

        div {
            id: "flex-parent",
            div { class: "myflex", "1" }
            div { class: "myflex", "2" }
            div { class: "myflex", "3" }
            div { class: "myflex", "4" }
            div { class: "myflex", "5" }
            div { class: "myflex", "6" }
            div { class: "myflex", "7" }
            div { class: "myflex", "8" }
            div { class: "myflex", "9" }
        }
    }
}
