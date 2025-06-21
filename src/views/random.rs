use dioxus::{logger::tracing::info, prelude::*};

#[component]
pub fn Random() -> Element {
    let mut images: Signal<Vec<String>> = use_signal(|| Vec::new());
    rsx! {
        // Image & button
        div {
            class: "bg-red-100",
            for img_src in images.read().iter() {
                img {
                    max_height: "300px",
                    src: "{img_src}"
                }
            }
            button {
                onclick: move |e: Event<MouseData>| async move {
                    info!("Clicked");
                    e.stop_propagation();
                    *images.write() = img_list().await.unwrap();
                },
                "Click me!"
            }
        }
    }
}

#[derive(serde::Deserialize)]
struct RandImg {
    url: String,
}

#[server]
async fn img_list() -> Result<Vec<String>, ServerFnError> {
    // let response = reqwest::get("https://api.nekosapi.com/v4/images/random")
    //     .await
    //     .unwrap() // <------------------------ error
    //     .json::<Vec<RandImg>>()
    //     .await
    //     .unwrap();
    let client = reqwest::Client::new();
    let response = reqwest::Client::get(&client, "https://api.nekosapi.com/v4/images/random")
        .header("Access-Control-Allow-Origin", "*")
        .send()
        .await
        .unwrap()
        .json::<Vec<RandImg>>()
        .await
        .unwrap();
    Ok(response.into_iter().map(|s| s.url).collect::<Vec<String>>())
}
