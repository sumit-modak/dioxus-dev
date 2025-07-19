use crate::views::*;
use dioxus::prelude::*;

#[rustfmt::skip]
#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    /// main routes
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
        #[nest("/test")]
            #[layout(crate::components::Navbar2)]
                #[route("/")]
                Test {},
                #[route("/play")]
                Play {},
                #[route("/random")]
                Random {},
                #[route("/pages")]
                Pages {},
                #[route("/profile")]
                ProfilePic {},
                #[route("/form")]
                Form {},
                #[route("/list")]
                List {},
                #[route("/display")]
                Display {},
                #[route("/float")]
                Float {},
                #[route("/column")]
                Column {},
                #[route("/position")]
                Position {},
                #[route("/flexbox")]
                FlexBox {},
                #[route("/grid")]
                Grid {},
            #[end_layout]
        #[end_nest]
        #[route("/dog")]
        DogView {},
    #[end_layout]

    /// route to redirect to other endpoints
    #[nest("/myblog")]
        #[redirect("/", || Route::BlogList {})]
        #[redirect("/:id", |id: i32| Route::Blog { id })]
    #[end_nest]

    /// fallback route
    #[route("/:..unknownroute")]
    NotFound { unknownroute: Vec<String> },
}
