mod blog;
mod components;
mod dioksus;
mod examples;
#[cfg(feature = "server")]
pub mod api;
mod view_router;
mod test;

use dioxus::prelude::*;
pub use view_router::Route;

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
// const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FERRIS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        // document::Stylesheet { href: TAILWIND_CSS }

        Router::<crate::view_router::Route> {}
    }
}
