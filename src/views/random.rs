use crate::components::Thumbnail;
use dioxus::{logger::tracing::info, prelude::*};

#[component]
pub fn Random() -> Element {
    let mut images: Signal<Vec<RandImg>> = use_signal(|| Vec::new());
    rsx! {
        // Image & button
        div {
            button {
                onclick: move |e: Event<MouseData>| async move {
                    info!("Clicked");
                    e.stop_propagation();
                    *images.write() = img_list().await.unwrap();
                },
                "Click me!"
            }
            div {
                style: "display: flex; flex-wrap: wrap; gap: 8px;",
                for entry in images.read().iter() {
                    Thumbnail {
                        thumbnail: entry.url.to_string(),
                        duration: "0:00".to_string(),
                        avatar: "https://via.placeholder.com/40".to_string(),
                        title: entry.tags.iter().map(|s| s.to_string()).collect::<String>(),
                        channel_name: entry.artist_name.clone().unwrap_or("null".to_string()),
                        views: "1.2M views".to_string(),
                        created: "2 days ago".to_string(),
                    }
                }
            }
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct RandImg {
    url: String,
    tags: Vec<String>,
    id: i32,
    artist_name: Option<String>,
}

#[server]
async fn img_list() -> Result<Vec<RandImg>, ServerFnError> {
    let client = reqwest::Client::new();
    let response = reqwest::Client::get(&client, "https://api.nekosapi.com/v4/images/random")
        .header("Access-Control-Allow-Origin", "*")
        .send()
        .await
        .unwrap()
        .json::<Vec<RandImg>>()
        .await;

    match response {
        Ok(v) => {
            println!("{v:?}");
            Ok(v)
        }
        Err(e) => {
            eprintln!("{e}");
            Err(e.into())
        }
    }
}
