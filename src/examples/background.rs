use dioxus::prelude::*;

const BACKGROUND_CSS: Asset = asset!("/assets/styles/background.css");
const SCENIC_IMG: Asset = asset!("/assets/images/scenic.png");

#[component]
pub fn Background() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BACKGROUND_CSS }
        section {
            class: "hero",
            style: format!("background-image: {SCENIC_IMG};"),
            figure {
                class: "profile-pic-figure",
                img {
                    src: asset!("/assets/icons/rust.png"),
                    alt: "Profile Picture",
                    title: "My profile pic",
                    width: 200,
                    height: 200,
                }
                figcaption {
                    "Rust Programming Language"
                }
            }
        }
    }
}
