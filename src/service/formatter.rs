use crate::constants::PLAYLIST_ID_LENGTH;

pub fn get_playlist_id(url: &str) -> Option<&str> {
    let url = url.trim();
    return if let Some(pos) = url.find("/playlist/") {
        let start = pos + "/playlist/".len();
        if start + PLAYLIST_ID_LENGTH <= url.len() {
             Some(&url[start..start + PLAYLIST_ID_LENGTH])
        }else{
            None
        }
    }else if url.len()==PLAYLIST_ID_LENGTH{
        Some(url)
    }else{
        None
    }
}