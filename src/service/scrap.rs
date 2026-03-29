pub fn get_tracks(body: &str) -> Result<String,Box<dyn std::error::Error>> {
    let document = scraper::Html::parse_document(&body);
    let selector = scraper::Selector::parse("#__NEXT_DATA__")?;
    match document.select(&selector).next() {
        Some(element) => Ok(element.inner_html()),
        None => Err("No tracks found".into()),
    }
}