use crate::components::*;
// use dioxus::events::AudioExtension::{autoplay, controls, r#loop, muted};
// use dioxus::events::AudioExtension::{autoplay, controls, r#loop, muted};
use dioxus::prelude::*;

#[component]
pub fn Misc() -> Element {
    rsx! {
        Hello {}
        Echo {}
        Counter {}
        a {
            href: "https://youtube.com/",
            target: "_blank",
            title: "Goes to youtube.com",
            "YOUTUBE",
        }
    }
}

const TOM_SCREAM: Asset = asset!("/assets/audio/Tom_Screaming.mp3");
const OHHH_MEME: Asset = asset!("/assets/video/OHHHH_MEME!.mp4");

#[component]
pub fn Play() -> Element {
    rsx! {
        audio {
            src: TOM_SCREAM,
            // type: "mpeg/mp3",
            controls: true,
            // autoplay: true,
            "Your browser does not support the audio element"
        }
        video {
            src: OHHH_MEME,
            controls: true,
            autoplay: true,
            muted: true,
            loop: true,
        }
    }
}
