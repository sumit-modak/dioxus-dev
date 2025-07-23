use dioxus::prelude::*;

const THUMBNAIL_CSS: Asset = asset!("/assets/styles/thumbnail.css");

#[derive(PartialEq, Clone, Props)]
pub struct ThumbnailProps {
    thumbnail: String,
    duration: String,
    avatar: String,
    title: String,
    channel_name: String,
    views: String,
    created: String,
}

pub fn Thumbnail(props: ThumbnailProps) -> Element {
    rsx! {
        document::Stylesheet { href: THUMBNAIL_CSS }
        div {
            class: "video-card max-w-sm bg-white rounded-lg shadow-md overflow-hidden hover:shadow-lg transition-shadow duration-200",
            div {
                class: "thumbnail relative aspect-video",
                img {
                    src: "{props.thumbnail}",
                    alt: "Video thumbnail",
                    class: "w-full h-full object-cover"
                }
                div {
                    class: "duration absolute bottom-2 right-2 bg-black bg-opacity-70 text-white text-xs font-semibold px-2 py-1 rounded",
                    "{props.duration}"
                }
            }
            div {
                class: "p-4",
                div {
                    class: "flex items-start gap-3",
                    img {
                        src: "{props.avatar}",
                        alt: "Channel avatar",
                        class: "w-10 h-10 rounded-full"
                    }
                    div {
                        class: "flex-1",
                        h3 {
                            class: "title text-base font-semibold text-gray-900 line-clamp-2",
                            "{props.title}"
                        }
                        p {
                            class: "channel text-sm text-gray-600 mt-1",
                            "{props.channel_name}"
                        }
                        div {
                            class: "meta text-xs text-gray-500 mt-1",
                            span { "{props.views}" }
                            span { class: "mx-1", "•" }
                            span { "{props.created}" }
                        }
                    }
                }
            }
        }
    }
}
