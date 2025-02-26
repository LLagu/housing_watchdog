mod scraper;

use crate::scraper::{Scraper, ScraperConfig};
use futures::{future, FutureExt};
use std::future::IntoFuture;
use std::io::prelude::*;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;
use std::{env, fs};
use thirtyfour::prelude::*;
use tokio;

async fn start_chromedriver() {
    let cwd = env::current_dir().unwrap();
    let chromedriver_path = (cwd.to_string_lossy() + "/chromedriver/chromedriver").to_string();
    Command::new(chromedriver_path)
        .arg("--port=9999")
        .spawn()
        .expect("Failed to start chromedriver");
    tokio::time::sleep(Duration::from_secs(1)).await;
}
async fn create_driver() -> WebDriverResult<WebDriver> {
    let caps = DesiredCapabilities::chrome();
    WebDriver::new("http://localhost:9999", caps).await
}

async fn from_config(scraper_structs: &mut Vec<Scraper>, config: ScraperConfig) {
    scraper_structs.push(Scraper {
        name: config.name,
        url: config.url,
        base_url_to_prepend: config.base_url_to_prepend,
        driver: create_driver().await.unwrap(),
        listing: vec![],
        house_link_css: config.house_link_css,
    })
}

#[tokio::main]
async fn main() {
    start_chromedriver().await;
    let config_path = Path::new("config.toml");
    let config_str = fs::read_to_string(config_path).unwrap();
    let configs: scraper::ScraperConfigVec = toml::from_str(config_str.as_str()).unwrap();
    let mut scraper_structs = vec![];
    for config in configs.scraper {
        from_config(&mut scraper_structs, config).await;
    }

    let futures: Vec<_> = scraper_structs
        .iter_mut()
        .map(|scraper| scraper.run())
        .collect();
    let results = future::join_all(futures).await;
}
