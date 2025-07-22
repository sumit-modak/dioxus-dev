use dioxus::prelude::*;

static GREET: GlobalSignal<String> = Signal::global(|| "Hello, World!".to_string());

#[component]
pub fn GlobalSignalHook() -> Element {
    rsx! {
        h3 { "message: {GREET}" }
        button {
            onclick: move |_| *GREET.write() = "Hello, User!".to_string(),
            "Shuffle"
        }
    }
}
