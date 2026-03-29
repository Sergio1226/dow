use crate::constants::SPOTIFY_URL;

use reqwest::Client;
use reqwest::header::CONTENT_TYPE;

use crate::models::spotify::PlayListData;
use crate::service::scrap::get_tracks;
use crate::service::utils::get_tracks_titles;

struct Spotify {
    pub client: Client,
}

impl Spotify {
    pub fn new() -> Self {
        Spotify {
            client: Client::new(),
        }
    }

    pub async fn get_playlist_by_id(
        &self,
        playlist_id: &str,
    ) -> Result<PlayListData, Box<dyn std::error::Error>> {
        let url = format!("{}/playlist/{}", SPOTIFY_URL, playlist_id);
        let response = self
            .client
            .get(url)
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await?;
        if !response.status().is_success() {
            return Err("Track not found".into());
        }
        let body = response.text().await?;
        Ok(serde_json::from_str(&get_tracks(&body)?)?)
    }
    
}

pub async fn get_playlist_tracks_titles( playlist_id: &str) -> Option<Vec<String>> {
    let spotify = Spotify::new();
    match spotify.get_playlist_by_id(playlist_id).await {
        Ok(playlist_data) => Some(get_tracks_titles(&playlist_data)),
        Err(_) => None,
    }
}