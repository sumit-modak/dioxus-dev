use dioxus::prelude::*;

use components::Navbar;
use views::{Blog, DogView, Home};

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/dog")]
    DogView {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FERRIS: Asset = asset!(
    "/assets/icons/ferris.png",
    ImageAssetOptions::new()
        .with_size(ImageSize::Manual {
            width: 300,
            height: 300
        })
        .with_avif()
);
const MAIN_CSS: Asset = asset!("/assets/styles/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FERRIS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Stylesheet { href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
