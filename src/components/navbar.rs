use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styles/navbar.css");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::BlogList {},
                "Blog"
            }
            Link {
                to: Route::DogView {},
                "HotDog"
            }
            Link {
                to: Route::Random {},
                "Random"
            }
            Link {
                to: Route::Misc {},
                "Misc"
            }
        }

        Outlet::<Route> {}
    }
}
