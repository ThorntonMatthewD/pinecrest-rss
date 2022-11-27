use chrono::offset::FixedOffset;
use chrono::{DateTime, NaiveDate};
use lazy_static::lazy_static;
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

        let time_str = extract_date_from_sermon(self);
        rss_item.set_pub_date(time_string_to_datetime(&time_str).to_rfc2822());

        let mut enclosure = rss::Enclosure::default();
        enclosure.set_url(link_to_media);
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

pub async fn populate_rss_feed<T>(mut channel: rss::Channel, items_to_add: Vec<T>) -> rss::Channel
where
    T: RssFeedItem,
{
    let rss_items: Vec<rss::Item> = items_to_add
        .iter()
        .map(|item| item.convert_to_rss_item())
        .collect();

    channel.set_items(rss_items);

    channel
}

fn time_string_to_datetime(time_str: &str) -> DateTime<FixedOffset> {
    DateTime::parse_from_str(time_str, "%m-%d-%Y %H:%M:%S %z").unwrap_or_else(|_| {
        DateTime::<FixedOffset>::from_local(
            NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0),
            FixedOffset::east_opt(8 * 60 * 60).unwrap(),
        )
    })
}

// Use fallback date of 01/01/2000 if a date cannot be
// found in either the title or description of sermon
fn extract_date_from_sermon(sermon: &SermonInfo) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[0-9]{1,2}(/|-)[0-9]{1,2}(/|-)[0-9]{2,4}").unwrap();
    };

    let mut combined_sermon_info: String = sermon.title.to_owned();
    combined_sermon_info.push_str(sermon.description.as_str());

    let matches: Vec<String> = RE
        .find_iter(combined_sermon_info.as_str())
        .filter_map(|date| date.as_str().parse().ok())
        .collect();

    let mut found_date = match matches.len() {
        1.. => {
            if let Some(first_match) = matches.into_iter().next() {
                first_match.replace('/', "-")
            } else {
                unreachable!()
            }
        }
        _ => String::from("01-01-2000"),
    };

    // Just going to use midnight lol
    found_date.push_str(" 00:00:00 -0500");

    found_date
}
