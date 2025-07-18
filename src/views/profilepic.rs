use dioxus::prelude::*;

const PROFILEPIC_CSS: Asset = asset!("/assets/styles/profilepic.css");

#[component]
pub fn ProfilePic() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: PROFILEPIC_CSS }
        section {
            class: "hero",
            figure {
                class: "profile-pic-figure",
                img {
                    src: asset!("/assets/icons/nix.png"),
                    alt: "Profile Picture",
                    title: "My profile pic",
                    width: 800,
                    height: 800,
                    figcaption {
                        "Jane Doe"
                    }
                }
            }
        }
    }
}
