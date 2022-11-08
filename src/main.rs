mod web_scraper;
mod rss_helper;

use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    routing::get,
    Router,
    response::IntoResponse
};
use axum_prometheus::PrometheusMetricLayer;
use cached::proc_macro::once;
use tower_http::{
    cors::Any,
    cors::CorsLayer
};
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

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
    println!("Sermons available at http://{}/sermons.rss", addr);

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

// Result is cached for 10 minutes to prevent spamming requests
#[once(time=600, option = false, sync_writes = true)]
async fn get_populated_rss_feed() -> rss::Channel {
    let sermons_found = web_scraper::obtain_sermons().await.unwrap();

    rss_helper::populate_rss_feed(
        rss_helper::create_rss_chanel(),
        sermons_found
    ).await
}

async fn serve_sermons() -> impl IntoResponse {
    (StatusCode::OK, get_populated_rss_feed().await.to_string())
}

async fn favicon() -> &'static [u8] {
    include_bytes!("favicon.ico")
}
