# housing_watchdog
This is housing_watchdog, a terminal application to scrape real estate websites and get notified through ntfy when a new house offer is posted.

![Alt text](/house_watchdog_screenshot.png?raw=true)
![Alt text](/notification_screenshot.jpeg?raw=true)

### Supported platforms
- Linux (developed and tested on Debian 12 bookworm).

### Download and run
#### Executable
- Download the latest version from the releases page.
- cd into the directory and run `./housing_watchdog`

#### Build
- Clone the repo 
- cd into the cloned directory
- Build with `cargo build` or `cargo build --release`
- Run with `cargo run` 

### How to use
#### Prerequisites
- Install ntfy on your phone or computer (https://ntfy.sh/)
- Install Chrome web browser and chromedriver (https://developer.chrome.com/docs/chromedriver/downloads)
- Important: make sure that the user that launches the program has the privileges to create folders and file in the launch directory.
- Do not run the script as sudo, or chromedriver throw an error (https://github.com/SeleniumHQ/selenium/issues/15327)
- Basic knowledge of the websites than need to be scraped. Specifically its structure and details of the elements containing the url to the house offer.

#### The config.toml file
Everything revolves around `config.toml`, a configuration file that allows you to specify which website to scrape, how and where to notify. 
Here's an example:

```toml
chromedriver_path = "/home/username/Documents/chromedriver-linux64/chromedriver"

[[scraper]]
name = "RealEstateSite1"
url = "https://example.com/listings"
base_url_to_prepend = "https://example.com"
house_link_css = "a.listing-item.a"
ntfy_topic = "ntfy-topic-1"

[[scraper]]
name = "RealEstateSite2"
url = "https://another-example.com/properties"
base_url_to_prepend = "https://another-example.com"
house_link_css = "div.property-card.link"
ntfy_topic = "ntfy-topic-2"
```

`chromedriver_path`: defines where your chromedriver executable is in your machine. Without it there is no Chrome automation.

`[[scraper]]`: defines the properties of one scraper. There can be multiple scrapers running at the same time! Just define them separately like in the example.

`name`: identifier used as a tag in the notification message

`url`: defines the url to scrape. Ideally your search parameters (houses price range, number of bedrooms etc.) are specified in the url. Website navigation for filtering results is not supported.

`base_url_to_prepend`: if the 'href' of the html/css element corresponding to a house does not contain the base url, it needs to be specified here.


For example if you are scraping "https://example.com/listings/amsterdam" and the `href` of one house element is `/house_streetname_42`, then you might need to specify 
`https://example.com/listings` in `base_url_to_prepend` to get `https://example.com/listings/house_streetname_42` (website dependent). This way once you click on the notification
message you are redirected to the specific house link.

If the element's `href` contains the full url, put `""` in this field.

`house_link_css`: the identifier of the html/css element corresponding to a house. Typically, the websites search results page will have a list of these.
the identifier's structure is `<tag>.<class>`. `<tag>` can be any relevant HTML tag (div, li, a, etc.). `<tag>`and `<class>` must be separated by `.`. 
`<class>` is the class name of the element. If it contains spaces yopu should remove them and replace them with a `.`. 

For example, if the element is `<div class="property-card link" href="https://another-example.com/my-next-house"<div/>` then `house_link_css` will be `div.property-card.link`.

`ntfy_topic`: the ntfy topic the scraper will send the notification to. See Receiving the notification.

#### Navigate the interface
- Press Tab to navigate.
- Press Enter to confirm.
- Press Esc or q to quit.

#### Receiving the notification
Read the Subscribing chapter from the official ntfy documentation: https://docs.ntfy.sh/