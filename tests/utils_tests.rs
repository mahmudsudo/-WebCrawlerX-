use crate::utils::{extract_links, normalize_url};

#[test]
fn test_extract_links() {
    let html = r#"
        <html>
            <body>
                <a href="https://example.com">Example</a>
                <a href="/relative/path">Relative</a>
                <a href="mailto:test@example.com">Email</a>
            </body>
        </html>
    "#;

    let base_url = "https://test.com";
    let links = extract_links(html, base_url);

    assert_eq!(links.len(), 2);
    assert!(links.contains(&"https://example.com".to_string()));
    assert!(links.contains(&"https://test.com/relative/path".to_string()));
}

#[test]
fn test_normalize_url() {
    assert_eq!(normalize_url("https://example.com"), "https://example.com/");
    assert_eq!(normalize_url("https://example.com/"), "https://example.com/");
    assert_eq!(normalize_url("https://example.com/path/../other"), "https://example.com/other");
    assert_eq!(normalize_url("https://example.com/path/./file"), "https://example.com/path/file");
}
