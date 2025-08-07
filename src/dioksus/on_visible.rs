use crate::components::Thumbnail;
use dioxus::{logger::tracing::info, prelude::*};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

#[component]
pub fn RandomAnime() -> Element {
    let mut images: Signal<Vec<RandImg>> = use_signal(|| Vec::<RandImg>::with_capacity(20));
    let mut image_resource = use_resource(move || async move {
        if let Ok(new_images) = anime_img_list().await {
            images.extend(new_images);
        }
        images
    });
    let mut loading = use_signal(|| false); // Loading state

    rsx! {
        div {
            div {
                style: "display: flex; flex-wrap: wrap; justify-content: center; gap: 8px;",
                for (index, entry) in images.read().iter().enumerate() {
                    Thumbnail {
                        thumbnail: entry.url.to_string(),
                        duration: "0:00".to_string(),
                        avatar: "https://via.placeholder.com/40".to_string(),
                        title: entry.tags.iter().map(|s| format!("{s} ")).collect::<String>(),
                        channel_name: entry.artist_name.clone().unwrap_or("null".to_string()),
                        views: "1.2M views".to_string(),
                        created: index.to_string(),
                    }
                }
            }
            if !*loading.read() {
                div {
                    class: "loading text-center text-gray-500 py-4",
                    "Loading more videos..."
                }
            }
            div {
                id: "sentinel",
                class: "sentinel h-1",
                onvisible: move |event: Event<VisibleData>| async move {
                    if let Ok(true) = event.data.is_intersecting() {
                        info!("Sentinel is visible, triggering fetch...");
                        loading.set(true);
                        if let Ok(new_images) = anime_img_list().await {
                            images.extend(new_images);
                        }
                        loading.set(false);
                    }
                }
            }
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub struct RandImg {
    url: String,
    tags: Vec<String>,
    id: i32,
    artist_name: Option<String>,
}

#[server(endpoint = "/api/fetch/anime")]
async fn anime_img_list() -> Result<Vec<RandImg>, ServerFnError> {
    let client = reqwest::Client::new();
    let response = reqwest::Client::get(&client, "https://api.nekosapi.com/v4/images/random")
        .header("Access-Control-Allow-Origin", "*")
        .send()
        .await
        .unwrap()
        .json::<Vec<RandImg>>()
        .await;

    match response {
        Ok(v) => Ok(v),
        Err(e) => {
            eprintln!("{e}");
            Err(e.into())
        }
    }
}
