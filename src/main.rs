use clap::{Command, Arg};
use std::{env, sync::Arc, time::Duration};

mod crawler;
mod error;
mod spiders;

use crate::crawler::Crawler;
use error::Error;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .subcommand(Command::new("spiders").about("List all spiders"))
        .subcommand(
            Command::new("run")
                .about("Run a spider")
                .arg(
                    Arg::new("spider")
                        .short('s')
                        .long("spider")
                        .help("The spider to run")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("delay")
                        .short('d')
                        .long("delay")
                        .help("Delay between requests in milliseconds")
                        .takes_value(true)
                        .default_value("200"),
                )
                .arg(
                    Arg::new("concurrent")
                        .short('c')
                        .long("concurrent")
                        .help("Number of concurrent requests")
                        .takes_value(true)
                        .default_value("2"),
                )
                .arg(
                    Arg::new("limit")
                        .short('l')
                        .long("limit")
                        .help("Maximum number of pages to crawl")
                        .takes_value(true)
                        .default_value("500"),
                ),
        )
        .arg_required_else_help(true)
        .get_matches();

    env::set_var("RUST_LOG", "info,crawler=debug");
    env_logger::init();

    if cli.subcommand_matches("spiders").is_some() {
        list_spiders();
    } else if let Some(matches) = cli.subcommand_matches("run") {
        run_spider(matches).await?;
    }

    Ok(())
}

fn list_spiders() {
    let spider_names = vec!["cvedetails", "github", "quotes"];
    println!("Available spiders:");
    for name in spider_names {
        println!("- {}", name);
    }
}

async fn run_spider(matches: &clap::ArgMatches) -> Result<(), anyhow::Error> {
    let spider_name = matches.value_of("spider").unwrap();
    let delay = matches.value_of("delay").unwrap().parse().unwrap_or(200);
    let concurrent = matches.value_of("concurrent").unwrap().parse().unwrap_or(2);
    let limit = matches.value_of("limit").unwrap().parse().unwrap_or(500);

    let crawler = Crawler::new(Duration::from_millis(delay), concurrent, limit);

    match spider_name {
        "cvedetails" => {
            let spider = Arc::new(spiders::cvedetails::CveDetailsSpider::new());
            crawler.run(spider).await;
        }
        "github" => {
            let spider = Arc::new(spiders::github::GitHubSpider::new());
            crawler.run(spider).await;
        }
        "quotes" => {
            let spider = spiders::quotes::QuotesSpider::new().await?;
            let spider = Arc::new(spider);
            crawler.run(spider).await;
        }
        _ => return Err(Error::InvalidSpider(spider_name.to_string()).into()),
    };

    Ok(())
}

#[cfg(test)]
mod tests;