use crate::Route;
use dioxus::prelude::*;

const BLOG_CSS: Asset = asset!("/assets/styles/blog.css");

#[component]
pub fn BlogBar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS }

        h1 {
            class: "blogbar",
            // style: "width: 50%",
            "Blog"
        }
        Outlet::<Route> {}
    }
}

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            class: "blog-content",
            // Content
            h1 {
                class: "blog-header",
                "This is blog #{id}!"
            }
            p {
                "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
            }

            // Navigation links
            div {
                Link {
                    to: Route::Blog { id: id - 1 },
                    class: "url",
                    "Previous"
                }
                span { " <---> " }
                Link {
                    to: Route::Blog { id: id + 1 },
                    class: "url",
                    "Next"
                }
            }
        }
    }
}

#[component]
pub fn BlogList() -> Element {
    rsx! {
        h2 { "Choose a post" }
        ul {
            class: "bloglist",
            li {
                Link {
                    to: Route::Blog { id: 1 },
                    class: "url",
                    "Fetch first blog from database."
                }
            }
            li {
                Link {
                    to: Route::Blog { id: 2 },
                    class: "url",
                    "Fetch second blog from database."
                }
            }
            li {
                Link {
                    to: Route::Blog { id: 3 },
                    class: "url",
                    "Fetch third blog from database."
                }
            }
        }
    }
}
