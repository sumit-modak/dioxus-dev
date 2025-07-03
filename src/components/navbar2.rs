use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar2() -> Element {
    rsx! {
        div {
            id: "navbar2",
            Link {
                class: "url",
                to: Route::Play {},
                "Play"
            }
            Link {
                class: "url",
                to: Route::Form {},
                "Form"
            }
            Link {
                class: "url",
                to: Route::Random {},
                "Random"
            }
            Link {
                class: "url",
                to: Route::Float {},
                "Float"
            }
            Link {
                class: "url",
                to: Route::Column {},
                "Column"
            }
        }

        Outlet::<Route> {}
    }
}
