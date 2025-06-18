use dioxus::prelude::*;

#[component]
pub fn NotFound(a: Vec<String>) -> Element {
    rsx! {
        "Not Found"
        for i in a {
            p { "{i}" }
        }
    }
}
