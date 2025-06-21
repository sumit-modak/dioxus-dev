use dioxus::prelude::*;

#[component]
pub fn NotFound(unknownroute: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {unknownroute:?}" }
    }
}
