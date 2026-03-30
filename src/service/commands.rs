use crate::service::spotify::get_playlist_tracks_titles;
use crate::service::downloader::download_audios_from_ids;

use std::path::PathBuf;


pub async fn download_playlist(url: &str, output_path: Option<String>){

    let data;
    let name;
    match get_playlist_tracks_titles(url).await{
        Some(body) => {
            data=body.0;
            name=body.1;
        },
        None => {
            eprintln!("Tracks not found for the provided playlist ID.");
            return;
        }
    }

    println!("Downloading Playlist {} with {} songs",name,data.len());

    let path = PathBuf::from(output_path.unwrap_or_else(|| String::new()));
     
    match download_audios_from_ids(data, path,name).await{
        Ok(()) => {
            println!("Playlist downloaded successfully.");
        },
        Err(e) => {
            eprintln!("Error initializing downloader: {}", e);
        }
    }
}
