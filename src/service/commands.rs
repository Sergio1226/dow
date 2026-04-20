use crate::service::spotify::get_playlist_tracks_titles;
use crate::service::downloader::{download_audios_in_zip,download_audios};
use crate::service::formatter::get_playlist_id;

use std::path::PathBuf;


pub async fn download_playlist(url: &str, output_path: Option<String>,in_zip:bool){
    let playlist_id = get_playlist_id(url); 
    if playlist_id.is_none(){
        eprintln!("Invalid playlist URL provided.");
        return;
    }
    let data;
    let name;
    match get_playlist_tracks_titles(playlist_id.unwrap()).await{
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

    if in_zip{
        match download_audios_in_zip(data, path,name).await{
            Ok(()) => {
                println!("Playlist downloaded successfully.");
            },
            Err(e) => {
                eprintln!("Error initializing downloader: {}", e);
            }
        }
    }else{
        match download_audios(data, path).await{
            Ok(()) => {
                println!("Playlist downloaded successfully.");
            },
            Err(e) => {
                eprintln!("Error initializing downloader: {}", e);
            }
        }
    }     
}
