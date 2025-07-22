use dioxus::prelude::*;

#[component]
pub fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    // >> two ways of reading the value of count
    // let value: i32 = count();
    // let mut value2 = count.read();
    rsx! {
        div {
            h1 { "count: {count}" }
            // >> three ways of writing to count
            button { onclick: move |_| count += 1, "+1" }
            button { onclick: move |_| count.set(0), "reset" }
            button {
                onclick: move |_| {
                    let mut value = count.write();
                    *value -= 1
                },
                "-1"
            }
        }
    }
}
