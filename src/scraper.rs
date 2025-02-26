use std::collections::HashSet;
use std::fmt;
use std::fs::{read_to_string, File};
use std::io::Write;
use ntfy::{dispatcher, Error, Payload, Priority, Url};
use thirtyfour::error::WebDriverResult;
use thirtyfour::{By, WebDriver};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub(crate) struct Scraper {
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) base_url_to_prepend: String,
    pub(crate) driver: WebDriver,
    pub(crate) listing: Vec<String>,
    pub(crate) house_link_css: String,
}
#[derive(Deserialize)]
pub(crate) struct ScraperConfigVec {
    pub(crate) scraper: Vec<ScraperConfig>,
}
#[derive(Deserialize, Serialize)]
pub(crate) struct ScraperConfig {
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) base_url_to_prepend: String,
    pub(crate) house_link_css: String,
}
impl fmt::Display for ScraperConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.name, self.url, self.base_url_to_prepend, self.house_link_css)
    }
}
impl Scraper {
    async fn scrape(&self) -> WebDriverResult<Vec<String>> {
        self.driver.goto(&self.url).await?;
        let list = self.driver.find_all(By::Css(&self.house_link_css)).await?;
        let mut new_houses: Vec<String> = vec![];
        for house in list.iter() {
            let mut link = house.attr("href").await?.unwrap_or_default();
            if !self.base_url_to_prepend.is_empty() {
                link.insert_str(0, self.base_url_to_prepend.as_str());
            }
            new_houses.push(link);
        }
        Ok(new_houses)
    }

    async fn detect(&mut self, new_items: Vec<String>) {
        let item_set: HashSet<_> = self.listing.iter().collect();
        let difference: Vec<_> = new_items
            .into_iter()
            .filter(|item| !item_set.contains(item))
            .collect();

        if difference.is_empty() {
            println!("No new items to be scraped");
        } else {
            for item in difference {
                self.listing.push(item.to_string());
                self.notify(item).await;
            }
            let file_path =
                "./prev_session/".to_owned() + str::replace(&self.url, "/", "_").as_str() + ".txt";
            let mut file = File::create(file_path).unwrap();
            for line in self.listing.iter() {
                file.write_all(line.as_bytes()).unwrap();
                file.write_all("\n".as_bytes()).unwrap();
            }
        }
    }

    async fn notify(&self, new_url: String) -> Result<(), Error> {
        let dispatcher = dispatcher::builder("https://ntfy.sh").build_async()?; // Build dispatcher

        let payload = Payload::new("rust-scraper-test")
            .message("Get a house!")
            .title("Nieuwe aanbod")
            .tags([&self.name])
            .priority(Priority::High)
            .click(Url::parse(new_url.as_str())?)
            .attach(Url::parse(new_url.as_str())?)
            .markdown(true);

        dispatcher.send(&payload).await?;

        Ok(())
    }

    fn load_previous_session_file(&mut self) -> std::io::Result<()> {
        &self.listing.clear();
        let file_path =
            "./prev_session/".to_owned() + str::replace(&self.url, "/", "_").as_str() + ".txt";

        match read_to_string(file_path.to_string()) {
            Ok(contents) => {
                for link in contents.split_whitespace() {
                    &self.listing.push(link.to_string());
                }
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    File::create(file_path.to_string())?;
                }
                std::io::ErrorKind::PermissionDenied => {}
                _ => println!("{:?}", e),
            },
        }
        Ok(())
    }

    pub(crate) async fn run(&mut self) {
        self.load_previous_session_file()
            .expect("Unexpected error in reading the file");
        loop {
            //TODO: manage scraped_results fail
            let scraped_results = self.scrape().await.expect("TODO: panic message");
            self.detect(scraped_results).await;
            tokio::time::sleep(tokio::time::Duration::from_secs(120)).await;
        } 
    }
}