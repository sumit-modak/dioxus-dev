use dioxus::prelude::*;
use dioxus::logger::tracing::info;

use crate::components::{ImageCarousel, ImageCarouselItem};

#[component]
pub fn Shopping() -> Element {
    let mut items = use_signal(|| Vec::new());
    use_resource( move || async move {
        loop {
            match nature_img_list(9).await {
                Ok(v) => {
                    items.extend(
                        v
                        .into_iter()
                        .enumerate()
                        .map(|(i, item)| ImageCarouselItem {
                            image: item.clone(),
                            title: item.split('=').next_back().unwrap().to_string(),
                            no: i,
                        }).collect::<Vec<ImageCarouselItem>>()
                    );
                    break;
                },
                Err(_) => continue,
            };
        }
        
    });

    rsx! {
        ImageCarousel {
            item: items()
        },
    }
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct RandNatureImg {
    location: String,
}

#[server(endpoint = "/api/fetch/nature")]
pub async fn nature_img_list(times: usize) -> Result<Vec<String>, ServerFnError> {
    let client = reqwest::Client::new();
    let mut final_response = Vec::<String>::new();
    for i in 0..times {
        let response = reqwest::Client::get(&client, "https://picsum.photos/800/400?random=1")
            .send()
            .await
            .unwrap()
            .json::<RandNatureImg>()
            .await
            .unwrap_or(RandNatureImg { location: "https://placehold.co/400x300?text=Fallback+Image".to_string() });
        final_response.push(response.location);
    }
    Ok(final_response)
}
