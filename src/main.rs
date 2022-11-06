mod web_scraper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sermons_found = web_scraper::obtain_sermons().await.unwrap();

    println!("Here are the sermons that have been found:\n\n{:#?}", sermons_found);

    Ok(())
}
