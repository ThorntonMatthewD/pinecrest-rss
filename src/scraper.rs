use reqwest::Error;

pub fn scrape_page(url: &str) -> Result<String, Error> {
  let response= reqwest::blocking::get(url)?;

  response.text()
}
