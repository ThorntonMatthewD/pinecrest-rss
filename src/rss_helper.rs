use regex::Regex;

use super::web_scraper::SermonInfo;

pub trait RssFeedItem {
  fn convert_to_rss_item(&self) -> rss::Item;
}

impl RssFeedItem for SermonInfo {
  fn convert_to_rss_item(&self) -> rss::Item {
    let mut rss_item = rss::Item::default();

    // Try to get rid of all of the clones
    let link_to_media = self.audio_url.clone();

    rss_item.set_title(self.title.clone());
    rss_item.set_link(link_to_media.clone());
    rss_item.set_description(self.description.clone());

    let mut guid = rss::Guid::default();
    guid.set_value(link_to_media.clone());
    rss_item.set_guid(guid);

    // Try to get the published date from title/description
    let time = chrono::offset::Utc::now();
    rss_item.set_pub_date(time.to_rfc2822());

    let mut enclosure = rss::Enclosure::default();
    enclosure.set_url(link_to_media.clone());
    enclosure.set_mime_type("audio/mpeg");
    rss_item.set_enclosure(enclosure);

    let mut extension = rss::extension::itunes::ITunesItemExtension::default();
    extension.set_duration(format!("{}", self.duration / 1000));
    rss_item.set_itunes_ext(extension);

    rss_item
  }
}

pub fn create_rss_chanel() -> rss::Channel {
  let mut channel = rss::Channel::default();

  channel.set_title("Pinecrest Baptist Church - From the Pulpit");
  channel.set_description("Sermons from Pinecrest Baptist Church");
  channel.set_link("https://www.pinecrestbaptistcharleston.org/from-the-pulpit");

  channel
}

pub async fn populate_rss_feed<T>(
  mut channel: rss::Channel,
  items_to_add: Vec<T>
) -> rss::Channel where
  T: RssFeedItem
{
    let rss_items: Vec<rss::Item> = items_to_add
      .iter()
      .map(|item | item.convert_to_rss_item())
      .collect();

    channel.set_items(rss_items);

    channel
}

// Use fallback date of 01/011/2000 if a date cannot be
// found in either the title or description of sermon
fn extract_date_from_sermon(sermon: SermonInfo) -> String {
  todo!()
}
