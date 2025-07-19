use dioxus::prelude::*;

#[component]
pub fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        div {
            h1 { "count: {count}" }
            button { onclick: move |_| count += 1, "+1" }
            button { onclick: move |_| count.set(0), "reset" }
            button { onclick: move |_| count -= 1, "-1" }
        }
    }
}
