use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;
use serde::Deserialize;
use thirtyfour::error::WebDriverResult;
use thirtyfour::{DesiredCapabilities, WebDriver};
use crate::scraper::ScraperConfig;

#[derive(Deserialize)]
pub(crate) struct ChromedriverConfig {
    pub(crate) chromedriver_path: String,
}

pub(crate) async fn start_chromedriver(chromedriver_path: String) {
    // let cwd = env::current_dir().unwrap();
    // let chromedriver_path = (cwd.to_string_lossy() + "/chromedriver/chromedriver").to_string();
    let chromedriver_path = (chromedriver_path);
    tokio::time::sleep(Duration::from_secs(2)).await;
    Command::new(chromedriver_path)
        .arg("--port=9999")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start chromedriver");
    tokio::time::sleep(Duration::from_secs(1)).await;
}
pub(crate) async fn create_driver() -> WebDriverResult<WebDriver> {
    let caps = DesiredCapabilities::chrome();
    WebDriver::new("http://localhost:9999", caps).await
}
