use reqwest::Error;


pub async fn scrape_page(client: reqwest::Client, url: &str) -> Result<String, Error> {
  let response = client.get(url).send().await?;

  response.text().await
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
