mod web_scraper;

use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    routing::get,
    Router,
    response::IntoResponse
};
use axum_prometheus::PrometheusMetricLayer;
use tower_http::{
    cors::Any,
    cors::CorsLayer
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Let anything through since this only runs locally
    let cors = CorsLayer::new().allow_origin(Any);

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    let app = Router::new()
        .route("/sermons.rss", get(serve_sermons))
        .route("/favicon.ico", get(favicon))
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8001));
    println!("Sermons available at  http://{}/sermons.rss", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler: Something didn't go okay");
    println!("\nShutdown signal received - Server is shutting down.");
}

async fn serve_sermons() -> impl IntoResponse {
    let sermons_found = web_scraper::obtain_sermons().await.unwrap();

    println!("Here are the sermons that have been found:\n\n{:#?}", sermons_found);

    let mut channel = rss::Channel::default();

    channel.set_title("Pinecrest Baptist Church - From the Pulpit");
    channel.set_description("Sermons from Pinecrest Baptist Church");
    channel.set_link("https://www.pinecrestbaptistcharleston.org/from-the-pulpit");

    (StatusCode::OK, channel.to_string())
}

async fn favicon() -> &'static [u8] {
    include_bytes!("favicon.ico")
}
