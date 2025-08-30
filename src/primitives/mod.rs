use dioxus::prelude::*;

mod accordion;
pub use accordion::MyAccordion;

#[component]
pub fn PrimitiveNavbar() -> Element {
    use crate::Route;
    rsx! {
        div {
            id: "test-navbar",
            Link {
                class: "url",
                to: Route::MyAccordion {},
                "Accordion"
            }
            div { class: "hr" }

            Outlet::<Route> {}
        }
    }
}

#[component]
pub fn PrimitiveRoot() -> Element {
    rsx! {}
}

