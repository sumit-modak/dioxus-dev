use crate::Route;
use dioxus::prelude::*;

const BLOG_CSS: Asset = asset!("/assets/styles/blog.css");

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS }

        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}

#[component]
pub fn BlogList() -> Element {
    rsx! {
        h2 { "Choose a post" }
        ul {
            li {
                Link {
                    to: Route::Blog { id: 1 },
                    "Read the first blog post"
                }
            }
            li {
                Link {
                    to: Route::Blog { id: 2 },
                    "Read the second blog post"
                }
            }
        }
    }
}

#[component]
pub fn BlogBar() -> Element {
    rsx! {
        h1 { "Blog" }
        Outlet::<Route> {}
    }
}
