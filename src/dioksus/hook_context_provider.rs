use dioxus::prelude::*;

#[derive(Clone)]
struct MusicPlayer {
    song: String,
}

#[component]
pub fn ContextHook() -> Element {
    use_context_provider(|| MusicPlayer {
        song: String::from("Kings Never Die!"),
    });
    rsx! {
        Temp {}
    }
}

#[component]
fn Temp() -> Element {
    let value: MusicPlayer = use_context::<MusicPlayer>();
    rsx! {
        div {
            onclick: move |_| consume_context::<MusicPlayer>().song = "Killshot".to_string(),
            h1 { "message: {value.song}" }
        }
    }
}
