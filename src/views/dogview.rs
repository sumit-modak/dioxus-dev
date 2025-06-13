use dioxus::prelude::*;

const HOTDOG_CSS: Asset = asset!("/assets/styles/hotdog.css");

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
pub fn DogView() -> Element {
    let mut img_src: Signal<String> = use_signal(|| "".to_string());
    let fetch_new = move |e: Event<MouseData>| async move {
        e.stop_propagation();
        let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap();
        img_src.set(response.message);
    };

    let save = move |e: Event<MouseData>| {
        e.stop_propagation();
    };

    rsx! {
        document::Stylesheet { href: HOTDOG_CSS }

        div { id: "title",
            h1 { "HotDog! 🌭" }
        }
        div { id: "dogview",
            img {
                max_height: "300px",
                src: "{img_src}"
            }
        }
        div { id: "buttons",
            button { id: "skip", onclick: fetch_new, "skip" }
            button { id: "save", onclick: save, "save!" }
        }
    }
}
