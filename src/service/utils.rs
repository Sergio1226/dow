use crate::models::spotify::PlayListData;

pub fn get_tracks_titles(playlist_data: &PlayListData) -> Vec<String> {
    playlist_data
        .props
        .page_props
        .state
        .data
        .entity
        .track_list
        .iter()
        .map(|track| format!("{} {}", format_text(&track.title), format_text(&track.subtitle)))
        .collect()
}

/// Formats a text by removing control characters, replacing multiple spaces with a single space, and replacing special HTML entities
pub fn format_text(text: &str) -> String {
    text.chars()
        .filter_map(|c| {
            if c.is_control() {
                None
            } else if c.is_whitespace() {
                Some(' ')
            } else {
                Some(c)
            }
        })
        .collect::<String>()
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|',' '], "_")
        .trim()
        .to_string()
}