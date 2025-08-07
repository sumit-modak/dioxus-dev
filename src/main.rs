use dioxus_dev::App;

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    // #[cfg(feature = "server")]
    // tokio::runtime::Builder::new_multi_thread()
    //     .enable_all()
    //     .build()
    //     .expect("Failed building the Runtime")
    //     .block_on(launch_server());

    // tokio::runtime::Runtime::new().unwrap().block_on(async {});
    launch_server().await;
}

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}

#[cfg(feature = "server")]
async fn launch_server() {
    use dioxus::prelude::DioxusRouterExt;
    use dioxus::prelude::ServeConfig;
    use dioxus_dev::api;

    // Connect to dioxus' logging infrastructure
    dioxus::logger::initialize_default();

    // Connect to the IP and PORT env vars passed by the Dioxus CLI (or your dockerfile)
    let socket_addr = dioxus::cli_config::fullstack_address_or_localhost();
    // let socket_addr = std::net::SocketAddr::new(
    //     std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
    //     8080,
    // );

    // Build a custom axum router
    let router = axum::Router::new()
        .nest("/", api::server_routes())
        .serve_dioxus_application(ServeConfig::new().unwrap(), App);
        // .into_make_service();

    // And launch it!
    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
