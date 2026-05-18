use crate::models::spotify::TagSong;

pub fn get_tracks(body: &str) -> Result<String, Box<dyn std::error::Error>> {
    let document = scraper::Html::parse_document(&body);
    let selector = scraper::Selector::parse("#__NEXT_DATA__")?;
    match document.select(&selector).next() {
        Some(element) => Ok(element.inner_html()),
        None => Err("No tracks found".into()),
    }
}

pub fn get_meta(body: &str) -> Result<TagSong, Box<dyn std::error::Error>> {
    let document = scraper::Html::parse_document(&body);
    let selector = scraper::Selector::parse(
        r#"meta[name="twitter:image"], meta[name="twitter:title"], meta[name="twitter:description"]"#,
    )?;
    let meta_elements: Vec<&str> = document.select(&selector).map(|e| e.attr("content").unwrap_or("")).collect();
    Ok(format_meta(meta_elements))
}

fn format_meta(info:Vec<&str>)->TagSong{
    let arr:Vec<&str>=info[1].split(" · ").collect();
    TagSong{
        title:info[0].into(),
        artists:arr[0].into(),
        year:arr[3].parse().unwrap_or(2026),
        image:Some(info[2].into()) ,
        album:arr[1].into()
    } 
}