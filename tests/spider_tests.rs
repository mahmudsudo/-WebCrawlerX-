use std::sync::Arc;
use crate::spiders::{Spider, cvedetails::CveDetailsSpider, github::GitHubSpider, quotes::QuotesSpider};

async fn test_spider<T: Spider + Send + Sync>(spider: Arc<T>) {
    let start_urls = spider.start_urls();
    assert!(!start_urls.is_empty(), "Spider should have at least one start URL");

    for url in start_urls {
        let result = spider.scrape(url).await;
        assert!(result.is_ok(), "Scraping should succeed");

        if let Ok((items, new_urls)) = result {
            assert!(!items.is_empty(), "Scraping should yield at least one item");
            
            for item in items {
                let process_result = spider.process(item).await;
                assert!(process_result.is_ok(), "Processing items should succeed");
            }

            assert!(!new_urls.is_empty(), "Scraping should yield at least one new URL");
        }
    }
}

#[tokio::test]
async fn test_cvedetails_spider() {
    let spider = Arc::new(CveDetailsSpider::new());
    test_spider(spider).await;
}

#[tokio::test]
async fn test_github_spider() {
    let spider = Arc::new(GitHubSpider::new());
    test_spider(spider).await;
}

#[tokio::test]
async fn test_quotes_spider() {
    let spider = QuotesSpider::new().await.unwrap();
    let spider = Arc::new(spider);
    test_spider(spider).await;
}
