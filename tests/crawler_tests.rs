use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use crate::crawler::Crawler;
use crate::spiders::Spider;

struct MockSpider {
    visited_urls: Arc<Mutex<Vec<String>>>,
}

impl MockSpider {
    fn new() -> Self {
        MockSpider {
            visited_urls: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl Spider for MockSpider {
    type Item = String;

    fn start_urls(&self) -> Vec<String> {
        vec!["https://example.com".to_string()]
    }

    async fn scrape(&self, url: String) -> Result<(Vec<Self::Item>, Vec<String>), Box<dyn std::error::Error + Send + Sync>> {
        let mut visited = self.visited_urls.lock().await;
        visited.push(url.clone());
        
        if url == "https://example.com" {
            Ok((
                vec!["Item 1".to_string(), "Item 2".to_string()],
                vec!["https://example.com/page1".to_string(), "https://example.com/page2".to_string()]
            ))
        } else {
            Ok((vec![], vec![]))
        }
    }

    async fn process(&self, item: Self::Item) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Processing item: {}", item);
        Ok(())
    }
}

#[tokio::test]
async fn test_crawler_basic_functionality() {
    let spider = Arc::new(MockSpider::new());
    let crawler = Crawler::new(Duration::from_millis(100), 2, 2);

    crawler.run(spider.clone()).await;

    let visited_urls = spider.visited_urls.lock().await;
    assert_eq!(visited_urls.len(), 3);
    assert!(visited_urls.contains(&"https://example.com".to_string()));
    assert!(visited_urls.contains(&"https://example.com/page1".to_string()));
    assert!(visited_urls.contains(&"https://example.com/page2".to_string()));
}

#[tokio::test]
async fn test_crawler_respects_delay() {
    let spider = Arc::new(MockSpider::new());
    let delay = Duration::from_millis(500);
    let crawler = Crawler::new(delay, 1, 3);

    let start = std::time::Instant::now();
    crawler.run(spider.clone()).await;
    let duration = start.elapsed();

    assert!(duration >= delay * 2, "Crawler should respect the delay between requests");
}

#[tokio::test]
async fn test_crawler_respects_concurrency() {
    let spider = Arc::new(MockSpider::new());
    let crawler = Crawler::new(Duration::from_millis(100), 1, 3);

    let start = std::time::Instant::now();
    crawler.run(spider.clone()).await;
    let duration = start.elapsed();

    let visited_urls = spider.visited_urls.lock().await;
    assert_eq!(visited_urls.len(), 3);
    assert!(duration >= Duration::from_millis(300), "Crawler should process URLs sequentially with concurrency 1");
}

#[tokio::test]
async fn test_crawler_respects_page_limit() {
    let spider = Arc::new(MockSpider::new());
    let crawler = Crawler::new(Duration::from_millis(100), 2, 2);

    crawler.run(spider.clone()).await;

    let visited_urls = spider.visited_urls.lock().await;
    assert_eq!(visited_urls.len(), 2, "Crawler should stop after reaching the page limit");
}
