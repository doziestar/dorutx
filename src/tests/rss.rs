#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Router, routing::get};
    use reqwest::Client;
    use std::net::SocketAddr;
    use tokio::net::TcpListener;
    use tower_http::trace::TraceLayer;
    use tracing_subscriber::{fmt, EnvFilter, Registry};
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    use crate::controllers::article_controller::get_articles;
    use crate::health_check;

    async fn spawn_app() -> String {
        Registry::default()
            .with(fmt::layer().with_writer(std::io::stdout))
            .with(EnvFilter::new("info"))
            .init();

        let trace_layer = TraceLayer::new_for_http();

        let app = Router::new()
            .route("/", get(health_check))
            .route("/health", get(health_check))
            .route("/articles", get(get_articles))
            .layer(trace_layer);

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service());

        tokio::spawn(server);

        format!("http://127.0.0.1:{}", port)
    }

    #[tokio::test]
    async fn get_articles_returns_ok() {
        let address = spawn_app().await;
        let client = Client::new();

        let response = client
            .get(&format!("{}/articles", address))
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(response.status(), reqwest::StatusCode::OK);
    }

    #[tokio::test]
    async fn get_articles_returns_articles() {
        let address = spawn_app().await;
        let client = Client::new();

        let response = client
            .get(&format!("{}/articles", address))
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(response.status(), reqwest::StatusCode::OK);

        let articles: Vec<crate::services::Article> = response.json().await.expect("Failed to parse response body.");
        assert!(!articles.is_empty(), "No articles found");
    }
}
