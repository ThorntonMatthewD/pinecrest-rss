use reqwest::Error;
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct SermonInfo {
  audio_url: String,
  title: String,
  author: String
}

pub async fn scrape_page(client: reqwest::Client, url: &str) -> Result<String, Error> {
  let response = client.get(url).send().await?;

  response.text().await
}

pub async fn parse_sermon_links(page_data: String) -> Vec<SermonInfo> {
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
        author: String::from(element_value.attr("data-author").unwrap())
      }
    )
  }

  sermons
}
