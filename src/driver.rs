use std::env;
use std::process::Command;
use std::time::Duration;
use thirtyfour::error::WebDriverResult;
use thirtyfour::{DesiredCapabilities, WebDriver};

pub(crate) async fn start_chromedriver() {
    let cwd = env::current_dir().unwrap();
    let chromedriver_path = (cwd.to_string_lossy() + "/chromedriver/chromedriver").to_string();
    Command::new(chromedriver_path)
        .arg("--port=9999")
        .spawn()
        .expect("Failed to start chromedriver");
    tokio::time::sleep(Duration::from_secs(1)).await;
}
pub(crate) async fn create_driver() -> WebDriverResult<WebDriver> {
    let caps = DesiredCapabilities::chrome();
    WebDriver::new("http://localhost:9999", caps).await
}