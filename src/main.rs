mod scraper;

const SERMON_PAGE_URL: &str = "https://www.pinecrestbaptistcharleston.org/from-the-pulpit";

fn main() {
    scraper::scrape_page(SERMON_PAGE_URL)
        .unwrap_or_else(|error| {
            panic!("Oops! Something went wrong grabbing the site info: {:?}", error)
        });
}
