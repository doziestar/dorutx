use axum::{Router, routing::get, http::StatusCode, response::IntoResponse};
use tokio::signal;
use tower_http::trace::{TraceLayer, DefaultOnRequest, DefaultOnResponse, DefaultOnFailure};
use tracing_subscriber::{fmt, EnvFilter, Registry};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod routes;
mod utilities;
mod services;
mod models;
mod controllers;

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("Failed to listen for ctrl+c event");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = async {
        std::future::pending::<()>().await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    println!("Signal received, starting graceful shutdown.");
}

#[tokio::main]
async fn main() {
    Registry::default()
        .with(fmt::layer().with_writer(std::io::stdout))
        .with(EnvFilter::new("info"))
        .init();

    let trace_layer = TraceLayer::new_for_http()
        .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
        .on_response(DefaultOnResponse::new().level(tracing::Level::INFO))
        .on_failure(DefaultOnFailure::new().level(tracing::Level::ERROR));

    let app = Router::new()
        .route("/", get(routes::home::home))
        .route("/health", get(health_check))
        .route("/articles", get(controllers::rss::get_articles))
        .layer(trace_layer);

    let addr = "0.0.0.0:3020";
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind port");
    println!("Server running on {}", addr);

    let server = axum::serve(listener, app.into_make_service()); // Serve the app
    let graceful = server.with_graceful_shutdown(shutdown_signal()); // Graceful shutdown

    // Await the graceful server future and handle errors
    if let Err(err) = graceful.await {
        eprintln!("Server error: {}", err);
    }
}
