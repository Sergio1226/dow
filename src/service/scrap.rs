use regex::Regex;

pub fn get_tracks(body: &str) -> Result<String,Box<dyn std::error::Error>> {
    let document = scraper::Html::parse_document(&body);
    let selector = scraper::Selector::parse("#__NEXT_DATA__")?;
    match document.select(&selector).next() {
        Some(element) => Ok(element.inner_html()),
        None => Err("No tracks found".into()),
    }
}

pub fn get_script(body: &str) -> Option<String> {
    let document = scraper::Html::parse_document(&body);
    let selector = scraper::Selector::parse("script").unwrap();
    let selected=document.select(&selector);
    let mut iter=selected.into_iter().skip(21);
    while let Some(element) = iter.next() {
        if element.inner_html().starts_with("var ytInitialData =") {
            return Some(element.inner_html());
        }
    }
    None
}

pub fn get_video_id(body: &str) -> Option<String> {
    let re = Regex::new(r#""videoId"\s*:\s*"([a-zA-Z0-9_-]{11})""#).ok()?;
    
    re.captures(body)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())}

pub fn get_yt_data(body: &str) -> String {
    if let Some(script) = get_script(body) {
        let data=&script["var ytInitialData =".len()..script.len()-1];
        return data.to_string()
    }
    "".to_string()
}