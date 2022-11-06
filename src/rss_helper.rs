use rss;

pub async fn create_rss_chanel() -> rss::Channel {
  let mut channel = rss::Channel::default();

  channel.set_title("Pinecrest Baptist Church - From the Pulpit");
  channel.set_description("Sermons from Pinecrest Baptist Church");
  channel.set_link("https://www.pinecrestbaptistcharleston.org/from-the-pulpit");

  channel
}
