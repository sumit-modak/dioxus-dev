use dioxus::prelude::*;

mod background;
pub use background::Background;

mod form;
pub use form::Form;

mod display;
pub use display::Display;

mod list;
pub use list::List;

mod float;
pub use float::Float;

mod column;
pub use column::Column;

mod position;
pub use position::Position;

mod flexbox;
pub use flexbox::FlexBox;

mod grid;
pub use grid::Grid;

/// Navigation bar for example route
#[component]
pub fn ExampleNavbar() -> Element {
    use crate::Route;
    rsx! {
        div {
            id: "example-navbar",
            Link {
                class: "url",
                to: Route::Background {},
                "Background"
            }
            Link {
                class: "url",
                to: Route::Form {},
                "Form"
            }
            Link {
                class: "url",
                to: Route::List {},
                "List"
            }
            Link {
                class: "url",
                to: Route::Display {},
                "Display"
            }
            Link {
                class: "url",
                to: Route::Float {},
                "Float"
            }
            Link {
                class: "url",
                to: Route::Column {},
                "Column"
            }
            Link {
                class: "url",
                to: Route::Position {},
                "Position"
            }
            Link {
                class: "url",
                to: Route::FlexBox {},
                "FlexBox"
            }
            Link {
                class: "url",
                to: Route::Grid {},
                "Grid"
            }
        }
        div { class: "hr" }

        Outlet::<Route> {}
    }
}

#[component]
pub fn Example() -> Element {
    rsx!()
}
