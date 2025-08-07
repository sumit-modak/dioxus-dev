#![allow(unused)]
use dioxus::prelude::*;

mod boolean;
pub use boolean::Boolean;

mod iter;
pub use iter::Iter;

mod hook_signal;
pub use hook_signal::Counter;

mod hook_context_provider;
pub use hook_context_provider::ContextHook;

mod hook_global_signal;
pub use hook_global_signal::GlobalSignalHook;

mod element_crudops;
pub use element_crudops::TodoFn;

mod on_visible;
pub use on_visible::RandomAnime;

mod shopping;
pub use shopping::Shopping;

/// Navigation bar for example route
#[component]
pub fn DioksusNavbar() -> Element {
    use crate::Route;
    rsx! {
        div {
            id: "example-navbar",
            Link {
                class: "url",
                to: Route::TodoFn {},
                "Todo"
            }
            Link {
                class: "url",
                to: Route::RandomAnime {},
                "Anime"
            }
            Link {
                class: "url",
                to: Route::Shopping {},
                "Shopping"
            }
        }
        div { class: "hr" }

        Outlet::<Route> {}
    }
}

#[component]
pub fn Dioksus() -> Element {
    rsx!()
}

// Use a future to handle async loading when sentinel is visible
// let load_more = use_coroutine(move |mut rx: UnboundedReceiver<()>| async move {
//     while rx.try_next().unwrap().is_some() {
//         if !*loading.read() {
//             loading.set(true);
//             if let Ok(new_images) = img_list().await {
//                 images.extend(new_images);
//             }
//             loading.set(false);
//         }
//     }
// });

// Use use_future to handle async loading
// use_future(move || async move {
//     if *loading.read() {
//         return;
//     }
//     loading.set(true);
//     if let Ok(new_images) = img_list().await {
//         images.extend(new_images);
//     }
//     loading.set(false);
// });
