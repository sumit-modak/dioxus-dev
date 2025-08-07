use crate::blog::*;
use crate::dioksus::*;
use crate::examples::*;
use crate::test::*;
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
        #[nest("/example")]
            #[layout(crate::examples::ExampleNavbar)]
            #[route("/")]
            Example {},
            #[route("/background")]
            Background {},
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
        #[nest("/examples_dyn")]
            #[layout(crate::dioksus::DioksusNavbar)]
            #[route("/")]
            Dioksus {},
            #[route("/todo")]
            TodoFn {},
            #[route("/random")]
            RandomAnime {},
            #[route("/shop")]
            Shopping {},
            #[end_layout]
        #[end_nest]
        #[nest("/test")]
            #[layout(crate::test::TestNavbar)]
                #[route("/")]
                Test {},
                #[route("/hero")]
                Hero {},
                #[route("/play")]
                Play {},
                #[route("/pages")]
                Pages {},
                #[route("/dog")]
                DogView {},
                #[route("/youtube")]
                Youtube {},
            #[end_layout]
        #[end_nest]
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
