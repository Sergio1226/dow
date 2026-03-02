use crate::service::spotify::Spotify;
use std::sync::Arc;

pub async fn download_playlist(url: &str, output_path: &str) {
    let spotify = Spotify::new();
    let data;
    match spotify.get_playlist_tracks_titles(url).await{
        Some(body) => {
            data=body;
        },
        None => {
            eprintln!("Tracks not found for the provided playlist ID.");
            return;
        }
    }
    let youtube = crate::service::youtube::Youtube::new();
    match crate::service::downloader::Downloader::new(youtube, output_path, "playlist").await{
        Ok(downloader) => {
            let arc=Arc::new(downloader);
            if let Err(e) =crate::service::downloader::download_audios_from_ids(arc, data).await {
                eprintln!("Error downloading audios: {}", e);
            }
        },
        Err(e) => {
            eprintln!("Error initializing downloader: {}", e);
        }
    }
}
