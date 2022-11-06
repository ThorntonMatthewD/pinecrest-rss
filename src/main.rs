const SERMON_PAGE_URL: &str = "https://www.pinecrestbaptistcharleston.org/from-the-pulpit";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(SERMON_PAGE_URL)?;
    response.text();

    Ok(())
}
