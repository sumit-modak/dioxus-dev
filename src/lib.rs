mod components;
#[cfg(feature = "server")]
pub mod routes;
mod views;

use dioxus::prelude::*;
use views::*;

#[rustfmt::skip]
#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(components::Navbar)]
        #[route("/")]
        Home {},
        #[nest("/blog")]
            #[layout(BlogBar)]
            #[route("/")]
            BlogList {},
            #[route("/post/:id")]
            Blog { id: i32 },
            #[end_layout]
        #[end_nest]
        #[route("/dog")]
        DogView {},
        #[route("/random")]
        Random {},
        #[nest("/misc")]
            #[route("/")]
            Misc {},
            #[route("/play")]
            Play {},
        #[end_nest]
    #[end_layout]
    #[nest("/myblog")]
        #[redirect("/", || Route::BlogList {})]
        #[redirect("/:id", |id: i32| Route::Blog { id })]
    #[end_nest]
    #[route("/:..unknownroute")]
    NotFound { unknownroute: Vec<String> },
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

#[component]
pub fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FERRIS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Stylesheet { href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
