use dioxus::{logger::tracing::info, prelude::*};

const YOUTUBE_CSS: Asset = asset!("/assets/styles/youtube.css");

pub fn Youtube() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: YOUTUBE_CSS }

        div {
            class: "bg-gray-100",
            header {
                class: "fixed top-0 w-full bg-white p-4 flex items-center justify-between border shadow-sm",
                div {
                    class: "flex items-center",
                    h1 {
                        class: "text-lg font-bold",
                        "Video Platform"
                    }
                }
                div {
                    class: "search-bar flex",
                    input {
                        r#type: "text",
                        placeholder: "Search",
                        class: "w-full p-2 rounded"
                    }
                    button { "Search" }
                }
                div {
                    class: "flex items-center",
                    span {
                        class: "text-sm",
                        "User Profile"
                    }
                }
            }
            div {
                class: "flex h-screen mt-16",
                aside {
                    class: "fixed left-0 w-64 bg-white h-screen p-4 border overflow-auto",
                    h2 {
                        class: "text-lg font-bold mb-4",
                        "Subscriptions"
                    }
                    ul {
                        for i in 0..5 {
                            li {
                                class: "p-2 hover-bg-gray-200 cursor-pointer",
                                "Channel {i}"
                            }
                        }
                    }
                }
                main {
                    class: "ml-64 p-4 w-full overflow-auto",
                    div {
                        class: "grid grid-cols-4 gap-4",
                        for _ in 0..8 {
                            div {
                                class: "flex flex-col",
                                img {
                                    src: "https://via.placeholder.com/1920x1080",
                                    alt: "Video Thumbnail",
                                    class: "w-48 h-28 object-cover rounded"
                                }
                                div {
                                    class: "p-2",
                                    h3 {
                                        class: "text-sm font-bold",
                                        // "Video Title {(_ + 1)}"
                                        "Video Title"
                                    }
                                    p {
                                        class: "text-sm text-gray-600",
                                        "Channel Name"
                                    }
                                    p {
                                        class: "text-sm text-gray-600",
                                        // match _ {
                                        //     0 => "1M views • 2 days ago",
                                        //     1 => "500K views • 1 week ago",
                                        //     2 => "2M views • 3 days ago",
                                        //     3 => "300K views • 5 days ago",
                                        //     4 => "800K views • 4 days ago",
                                        //     5 => "1.5M views • 1 day ago",
                                        //     6 => "400K views • 6 days ago",
                                        //     _ => "600K views • 2 weeks ago",
                                        // }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
