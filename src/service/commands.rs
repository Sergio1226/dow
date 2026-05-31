use crate::service::spotify::get_playlist;
use crate::service::downloader::{download_audios_in_zip,download_audios};
use crate::service::formatter::get_playlist_id;
use crate::globals::WITH_IMAGE;

use std::path::PathBuf;
pub async fn download_playlist(url: &str, output_path: Option<String>,in_zip:bool,with_image:bool){
    {
        *WITH_IMAGE.lock().unwrap()=with_image;
    }
    
    let playlist_id = get_playlist_id(url); 
    if playlist_id.is_none(){
        eprintln!("Invalid playlist URL provided.");
        return;
    }
    let data;
    let name;
    match get_playlist(playlist_id.unwrap()).await{
        Ok(playlist) => {
            data=playlist.songs;
            name=playlist.name;
        },
        Err(e) => {
            eprintln!("Error occurred while fetching playlist: {}", e);
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
