use scraper::{Html, Selector};
use reqwest::{self, header, Error};

const SERMON_PAGE_URL: &str = "https://www.pinecrestbaptistcharleston.org/from-the-pulpit";

#[derive(Debug)]
pub struct SermonInfo {
  pub audio_url: String,
  pub title: String,
  pub description: String,
  pub duration: u32
}

pub async fn obtain_sermons() -> Result<Vec<SermonInfo>, Error> {
  let mut headers = header::HeaderMap::new();
  headers.insert(header::USER_AGENT,
      header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0"));


  let client = reqwest::Client::builder()
      .default_headers(headers)
      .build()?;

  let result = scrape_page(client, SERMON_PAGE_URL)
      .await
      .unwrap_or_else(|error| {
          panic!("Oops! Something went wrong grabbing the site info: {:?}", error)
      });

  Ok(parse_sermon_links(result).await)
}

async fn scrape_page(client: reqwest::Client, url: &str) -> Result<String, Error> {
  let response = client.get(url).send().await?;

  response.text().await
}

async fn parse_sermon_links(page_data: String) -> Vec<SermonInfo> {
  let mut sermons = Vec::<SermonInfo>::new();

  let parsed_page = Html::parse_document(page_data.as_str());

  let audio_selector = Selector::parse("div.sqs-audio-embed")
    .unwrap_or_else(|_error| panic!("Couldn't find any audio embeds, sorry."));

  for audio_embed in parsed_page.select(&audio_selector) {
    let element_value = audio_embed.value();

    sermons.push(
      SermonInfo {
        audio_url: String::from(element_value.attr("data-url").unwrap()),
        title: String::from(element_value.attr("data-title").unwrap()),
        description: String::from(element_value.attr("data-author").unwrap()),
        duration: element_value.attr("data-duration-in-ms").unwrap().parse::<u32>().unwrap_or(0)
      }
    )
  }

  sermons
}
