use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar2() -> Element {
    rsx! {
        div {
            id: "navbar2",
            Link {
                to: Route::Play {},
                "Play"
            }
            Link {
                to: Route::Form {},
                "Form"
            }
            Link {
                to: Route::Random {},
                "Random"
            }
        }

        Outlet::<Route> {}
    }
}
