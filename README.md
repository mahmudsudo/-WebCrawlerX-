🕷️ WebCrawlerX 🚀

A flexible and efficient web crawler written in Rust.

## Features

- Multiple spider implementations (CVE Details, GitHub, Quotes)
- Configurable crawling parameters (delay, concurrent requests, page limit)
- Easy to extend with new spiders

## Installation

```bash
cargo install webcrawlerx
```


## Usage

List available spiders:
```bash
webcrawlerx spiders
```

Run a specific spider:
```bash
webcrawlerx run --spider <spider_name>
--spider <spider_name> [--delay <ms>] [--concurrent <num>] [--limit <num>]
```


Example:
```bash
webcrawlerx run --spider cvedetails --delay 200 --concurrent 2 --limit 10
```


## Adding a New Spider

To add a new spider, create a new module in the `spiders` directory and implement the `Spider` trait. Then, update the `run_spider` function in `main.rs` to include your new spider.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
