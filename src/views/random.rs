use dioxus::prelude::*;

#[component]
pub fn Random() -> Element {
    rsx! {
        div {
            crate::components::Img {}
        }
    }
}
