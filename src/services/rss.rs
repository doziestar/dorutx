use reqwest::Client;
use rss::Channel;
use serde::Serialize;
use axum::http::StatusCode;

#[derive(Serialize)]
pub struct Article {
    pub title: String,
    pub link: String,
    pub description: String,
}

/// Fetch articles from Medium's RSS feed.
///
/// This function sends a request to Medium's RSS feed for the specified
/// user and parses the response to extract the articles.
///
/// # Returns
///
/// If successful, returns a `Result` containing a vector of `Article` structs.
/// If an error occurs, returns a `Result` containing a tuple with the HTTP status
/// code and an error message.
pub async fn fetch_medium_articles() -> Result<Vec<Article>, (StatusCode, String)> {
    let feed_url = "https://medium.com/feed/@doziestar";
    let client = Client::new();

    let response = client.get(feed_url).send().await.map_err(|err| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to fetch RSS feed: {}", err))
    })?;

    let body = response.text().await.map_err(|err| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read RSS feed: {}", err))
    })?;

    let channel = Channel::read_from(body.as_bytes()).map_err(|err| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to parse RSS feed: {}", err))
    })?;

    let articles: Vec<Article> = channel
        .items()
        .iter()
        .map(|item| Article {
            title: item.title().unwrap_or("").to_string(),
            link: item.link().unwrap_or("").to_string(),
            description: item.description().unwrap_or("").to_string(),
        })
        .collect();

    Ok(articles)
}
