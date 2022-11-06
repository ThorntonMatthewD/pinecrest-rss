mod scraper;

use reqwest::{self, header};

const SERMON_PAGE_URL: &str = "https://www.pinecrestbaptistcharleston.org/from-the-pulpit";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT,
        header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0"));


    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let result = scraper::scrape_page(client, SERMON_PAGE_URL)
        .await
        .unwrap_or_else(|error| {
            panic!("Oops! Something went wrong grabbing the site info: {:?}", error)
        });

    let sermons_found = scraper::parse_sermon_links(result).await;

    println!("Here are the sermons that have been found:\n\n{:#?}", sermons_found);

    Ok(())
}
