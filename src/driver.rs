use port_check::free_local_port;
use serde::Deserialize;
use std::process::{Command, Stdio};
use std::time::Duration;
use thirtyfour::error::WebDriverResult;
use thirtyfour::{DesiredCapabilities, WebDriver};

#[derive(Deserialize)]
pub(crate) struct ChromedriverConfig {
    pub(crate) chromedriver_path: String,
}

pub(crate) async fn start_chromedriver(chromedriver_path: String) -> String {
    let chromedriver_path = chromedriver_path;
    let free_port = free_local_port().unwrap().to_string();
    Command::new(chromedriver_path)
        .arg(format!("{}{}", "--port=", free_port))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start chromedriver");
    tokio::time::sleep(Duration::from_secs(1)).await;
    free_port
}
pub(crate) async fn create_driver(port: String) -> WebDriverResult<WebDriver> {
    let     caps = DesiredCapabilities::chrome();
    WebDriver::new(format!("{}{}", { "http://localhost:" }, { port }), caps).await
}
