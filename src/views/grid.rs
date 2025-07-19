use dioxus::prelude::*;

const GRID_CSS: Asset = asset!("/assets/styles/grid.css");

#[component]
pub fn Grid() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: GRID_CSS }

        main {
            class: "grid-container",
            for i in 1..=8 {
                div { class: "grid-box box-{i}", style: "display: grid; place-content: center;", "{i}" }
            }
        }

        div { style: "background-color: brown; height: 4px; margin: 16px 0px;" }

        main {
            class: "new-grid-container",
            for i in 1..=16 {
                div { class: "new-grid-box", "{i}" }
            }
        }
    }
}
