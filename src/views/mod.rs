#![allow(unused)]
use dioxus::prelude::*;

mod home;
pub use home::Home;

mod hero;
pub use hero::Hero;

mod dogview;
pub use dogview::DogView;

mod favorites;
pub use favorites::Favorites;

mod play;
pub use play::Play;

mod random;
pub use random::Random;

mod pages;
pub use pages::Pages;

mod youtube;
pub use youtube::Youtube;

mod notfound;
pub use notfound::NotFound;

/// Test Navigation Bar
#[component]
pub fn TestNavbar() -> Element {
    use crate::Route;
    rsx! {
        div {
            id: "test-navbar",
            Link {
                class: "url",
                to: Route::Hero {},
                "Hero"
            }
            Link {
                class: "url",
                to: Route::Play {},
                "Play"
            }
            Link {
                class: "url",
                to: Route::Random {},
                "Random"
            }
            Link {
                class: "url",
                to: Route::Pages {},
                "Pages"
            }
            Link {
                class: "url",
                to: Route::DogView {},
                "DogView"
            }
            Link {
                class: "url",
                to: Route::Youtube {},
                "Youtube"
            }
            Link {
                class: "url",
                to: Route::TodoFn {},
                "Todo"
            }
        }
        div { class: "hr" }

        Outlet::<Route> {}
    }
}

#[component]
pub fn Test() -> Element {
    rsx! {
        crate::components::Echo {}
    }
}
