// tests/poster_tests.rs

use crate::poster::Poster;
use std::collections::HashMap;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};
use tokio;

#[tokio::test]
async fn test_post_url_data() {
    // Start a mock server
    let mock_server = MockServer::start().await;

    // Define the mock behavior
    Mock::given(method("POST"))
        .and(path("/logs/_doc"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    // Create the Poster instance
    let poster = Poster::new();

    // Prepare the data to be sent
    let mut data = HashMap::new();
    data.insert("key1".to_string(), "value1".to_string());
    data.insert("key2".to_string(), "value2".to_string());

    // Call the post_url_data method with the mock server's URL
    let result = poster.post_url_data(&data).await;

    // Assert the result is Ok
    assert!(result.is_ok());
}
