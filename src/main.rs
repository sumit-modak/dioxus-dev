use dioxus_dev::App;

#[cfg(feature = "server")]
use dioxus_dev::routes;

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
    // use dioxus::fullstack::server::DioxusRouterExt;
    use dioxus::prelude::DioxusRouterExt;
    use dioxus::prelude::ServeConfigBuilder;

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
        .nest("/", routes::server_routes())
        .serve_dioxus_application(ServeConfigBuilder::new(), App)
        .into_make_service();

    // And launch it!
    // let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    // axum::serve(listener, router).await.unwrap();

    match tokio::net::TcpListener::bind(socket_addr).await {
        Ok(listener) => {
            println!("Server running on {}", socket_addr);
            axum::serve(listener, router).await.unwrap();
        }
        Err(e) => {
            eprintln!(
                "Failed to bind to {}: {}. Try a different port.",
                socket_addr, e
            );
            // let socket_addr = std::net::SocketAddr::new(
            //     std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
            //     8081,
            // );
            // let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
            // println!("Server running on {}", socket_addr);
            // axum::serve(listener, router).await.unwrap();
            // std::process::exit(1);
        }
    }
}
