use crate::views::*;
use dioxus::prelude::*;

#[rustfmt::skip]
#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[layout(crate::components::Navbar)]
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
            #[route("/form")]
            Form {},
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
