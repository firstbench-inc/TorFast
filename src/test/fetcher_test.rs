
// tests/fetcher_tests.rs

use crate::fetcher::Fetcher;

#[tokio::test]
async fn test_fetch() {
    let fetcher = Fetcher::new();
    let url = "https://example.com";
    let result = fetcher.fetch(url).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_fetch_tor() {
    let fetcher = Fetcher::new();
    let url = "https://check.torproject.org";
    let result = fetcher.fetch(url).await;
    assert!(result.is_ok());
    let text = result.unwrap();
    assert!(text.contains("Congratulations. This browser is configured to use Tor."));
}
