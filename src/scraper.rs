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

/*
  Sample Entry:

  <div
  class="sqs-audio-embed"
  data-url="https://static1.squarespace.com/static/5ab2b6c9f7939229ff49a57c/t/634cad5f5f42f274256fceba/1665969570284/Lee+Bolton+10-16-22.mp3/original/Lee+Bolton+10-16-22.mp3"
  data-mime-type=""
  data-title="Faith Over Fear - Philippians 1:2-6"
  data-author="Rev. Lee Bolton, 10-16-22"
  data-show-download="false"
  data-design-style="minimal"
  data-duration-in-ms="1536000"
  data-color-theme="dark"
>

*/
