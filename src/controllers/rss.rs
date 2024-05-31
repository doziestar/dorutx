use axum::{http::StatusCode, response::IntoResponse, Json};
use crate::services::rss::fetch_medium_articles;
use serde_json::json;

/// Get articles from Medium.
///
/// This function handles the HTTP request to get articles from Medium's
/// RSS feed. It calls the `fetch_medium_articles` service function and
/// returns the articles as a JSON response.
///
/// # Returns
///
/// A JSON response containing a list of articles if successful,
/// or an error message if the request fails.
pub async fn get_articles() -> impl IntoResponse {
    match fetch_medium_articles().await {
        Ok(articles) => (StatusCode::OK, Json(json!(articles))),
        Err((status, err)) => (status, Json(json!({ "error": err }))),
    }
}
