mod scraper;
mod driver;

use futures::{future, FutureExt};
use std::future::IntoFuture;
use std::io::prelude::*;
use std::path::Path;
use std::{env, fs};
use thirtyfour::prelude::*;
use tokio;



async fn watchdog_logic() {
    driver::start_chromedriver().await;
    let config_path = Path::new("config.toml");
    let config_str = fs::read_to_string(config_path).unwrap();
    let configs: scraper::ScraperConfigVec = toml::from_str(config_str.as_str()).unwrap();
    let mut scraper_structs = vec![];
    for config in configs.scraper {
        scraper::from_config(&mut scraper_structs, config).await;
    }

    let futures: Vec<_> = scraper_structs
        .iter_mut()
        .map(|scraper| scraper.run())
        .collect();
    let results = future::join_all(futures).await;
}

#[tokio::main]
async fn main() {
    watchdog_logic().await;
}
