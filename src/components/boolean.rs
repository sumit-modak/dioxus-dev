use dioxus::prelude::*;

#[component]
pub fn Boolean() -> Element {
    let name = "Alice";
    let show_name = true;
    let show_title = true;
    rsx! {
        if show_name {
            div {
                background_color: "green",
                h1 {
                    font_size: "100px",
                    "Hello, {name}!"
                }
                p {
                    font_size: "20px",
                    "A quick brown fox jumps over the lazy dog"
                }
            }
        }

        div {
            background_color: "blue",
            { show_title.then(|| rsx! { "your title!" } ) }
        }

    }
}
