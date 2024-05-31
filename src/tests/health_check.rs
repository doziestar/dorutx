#[cfg(test)]
mod tests {
    use reqwest::Client;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn health_check_works() {
        sleep(Duration::from_secs(2)).await;

        let client = Client::new();
        let address = "http://0.0.0.0:3020/health";

        let response = client
            .get(address)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(response.status(), reqwest::StatusCode::OK);
        let body = response.text().await.expect("Failed to read response body.");
        assert_eq!(body, "OK");
    }
}
