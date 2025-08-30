use dioxus::logger::tracing::info;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone, Debug)]
pub struct ImageCarouselItem {
    pub image: String, // URLs or paths to images
    pub title: String,
    pub no: usize,
}

#[derive(Props, PartialEq, Clone)]
pub struct ImageCarouselProps {
    item: Vec<ImageCarouselItem>,
}

const IMAGE_CAROUSEL_CSS: Asset = asset!("/assets/styles/imagecarousel.css");

pub fn ImageCarousel(cx: ImageCarouselProps) -> Element {
    let mut current_view = use_signal(|| 0usize);
    // let mut item_count = use_signal(|| cx.item.len());
    const VISIBLE_COUNT: usize = 3;

    rsx! {
        document::Stylesheet { href: IMAGE_CAROUSEL_CSS }
        div {
            class: "carousel-container",
            // Navigation buttons at the top
            div {
                class: "nav-buttons",
                button {
                    class: "nav-button",
                    onclick: move |_| {
                        if current_view() > 0 {
                            current_view -= 1
                        };
                        // info!("{}", item_count);
                    },
                    "←"
                }
                button {
                    class: "nav-button",
                    onclick: move |_| {
                        if current_view() < (cx.item.len() / VISIBLE_COUNT) {
                            current_view += 1
                        };
                        // info!("{}", item_count);
                    },
                    "→"
                }
            }
            // Image display
            div {
                class: "image-gallery",
                for i in (current_view * VISIBLE_COUNT)..std::cmp::min((current_view + 1) * VISIBLE_COUNT, cx.item.len()) {
                    div {
                        class: "image-card",
                        img {
                            src: "{cx.item[i].image}",
                            class: "image",
                            alt: "{cx.item[i].title}"
                        }
                        p {
                            class: "image-title",
                            "{cx.item[i].no}: {cx.item[i].title}"
                        }
                    }
                }
            }
        }
    }
}
