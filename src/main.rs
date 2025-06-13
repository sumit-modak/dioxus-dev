use dioxus_dev::{App, routes};

fn main() {
    // #[allow(
    //     clippy::expect_used,
    //     clippy::diverging_sub_expression,
    //     clippy::needless_return
    // )]

    #[cfg(feature = "server")]
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
        .block_on(launch_server());

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

#[cfg(feature = "server")]
async fn launch_server() {
    // use dioxus::fullstack::server::DioxusRouterExt::serve_dioxus_application;
    use dioxus::prelude::DioxusRouterExt;
    use dioxus::prelude::ServeConfigBuilder;

    // Connect to dioxus' logging infrastructure
    dioxus::logger::initialize_default();

    // Connect to the IP and PORT env vars passed by the Dioxus CLI (or your dockerfile)
    let socket_addr = dioxus::cli_config::fullstack_address_or_localhost();

    // Build a custom axum router
    let router = axum::Router::new()
        .nest("/", routes::server_routes())
        .serve_dioxus_application(ServeConfigBuilder::new(), App)
        .into_make_service();

    // And launch it!
    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
