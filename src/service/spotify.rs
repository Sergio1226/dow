use crate::constants::{SPOTIFY_URL, SPOTIFY_URL_TRACK};

use reqwest::header::CONTENT_TYPE;
use reqwest::{Client, Error, Response};

use crate::models::spotify::{PlayList, PlayListData, TagSong};
use crate::service::scrap::{get_tracks,get_meta};
use crate::service::utils::raw_to_playlist;

pub struct Spotify {
    pub client: Client,
}

impl Spotify {

    /// Creates a new instance of the Spotify struct with an initialized HTTP client.
    pub fn new() -> Self {
        Spotify {
            client: Client::new(),
        }
    }

    /// A helper function to perform a GET request to the specified URL and return the response.
    async fn get_fn(&self, url: &str) -> Result<Response, Error> {
        self.client
            .get(url)
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await
    }

    /// Fetches the raw playlist data from Spotify using the provided playlist ID and returns it as a string.
    async fn consult_playlist(
        &self,
        playlist_id: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}/playlist/{}", SPOTIFY_URL, playlist_id);
        let response = self.get_fn(&url).await?;

        if !response.status().is_success() {
            return Err("Track not found".into());
        }

        response
            .text()
            .await
            .map_err(|e| "Failed to read response body".into())
    }

    /// Fetches a playlist from Spotify using the provided playlist ID and returns it in a structured PlayList format.
    async fn get_playlist_raw(
        &self,
        playlist_id: &str,
    ) -> Result<PlayListData, Box<dyn std::error::Error>> {
        let body = self.consult_playlist(playlist_id).await?;

        let json_content= get_tracks(&body)?;
        
        match serde_json::from_str(&json_content) {
            Ok(playlist_data) => Ok(playlist_data),
            Err(_) => Err("Failed to parse playlist data".into()),
        }
    }

    
   pub async fn get_tags_by_id(&self, song_id: &str) -> Result<TagSong, Box<dyn std::error::Error>> {
        let url = format!("{}{}", SPOTIFY_URL_TRACK, song_id);
        let response=self.get_fn(&url).await?;

        if !response.status().is_success(){
            return Err("Track not found".into());
        }

        let body=response.text().await?;

        let meta=get_meta(&body)?;
    
        Ok(meta) 
    }

    /// Fetches a playlist from Spotify using the provided playlist ID and returns it in a structured PlayList format.
    pub async fn get_playlist_by_id(&self, playlist_id: &str) -> Result<PlayList, Box<dyn std::error::Error>> {
        let playlist_data = self.get_playlist_raw(playlist_id).await?;
        let playlist = raw_to_playlist(playlist_data);
        Ok(playlist)
    }
}

pub async fn get_tags(song_id: &str) -> Result<TagSong, Box<dyn std::error::Error>> {
    let spotify = Spotify::new();
    spotify.get_tags_by_id(song_id).await
}

/// Fetches a playlist from Spotify using the provided playlist ID and returns it in a structured PlayList format.
pub async fn get_playlist(playlist_id: &str) -> Result<PlayList, Box<dyn std::error::Error>> {
    let spotify = Spotify::new();
    spotify.get_playlist_by_id(playlist_id).await
}

pub async fn download_image(url:&str)->Result<Vec<u8>,Box<dyn std::error::Error>>{
    let r=reqwest::get(url).await?;
    let a=r.bytes().await?;
    Ok(a.to_vec())
}


