use dioxus::{prelude::*};

#[component]
pub fn Global() -> Element {
    use_context_provider(|| Signal::new(String::from("This is a global value")));
    rsx! {
        Temp {}
    }
}

#[component]
fn Temp() -> Element {
    let value: Signal<String> = use_context::<Signal<String>>();
    rsx! {
        div {
            h1 { "message: {value}" }
        }
    }
}
