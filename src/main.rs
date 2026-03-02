mod models;
mod service;

// use std::sync::Arc;

// use std::path::PathBuf;

// use yt_dlp::Downloader; 
// use yt_dlp::client::deps::Libraries;


// use service::scrap::get_yt_data;
// use service::spotify::Spotify;


// #[tokio::main]
// async fn main() {
//     let spotify = Spotify::new();
//     let url="3gh1cUVq6xw082KmdIghwL";
//     let mut data=vec![];
//     match spotify.get_playlist_tracks_titles(url).await{
//         Some(body) => {
//             data=body;
//         },
//         None => {
//             print!("no hay canciones");
//         }
//     }
//     let youtube = crate::service::youtube::Youtube::new();
//     let downloader = crate::service::downloader::Downloader::new(youtube, "C:\\Users\\serpe\\Music\\carpeta").await.unwrap();
//     let arc=Arc::new(downloader);
//     crate::service::downloader::download_audios_from_ids(arc, data).await.unwrap();
// }

use clap::Parser;

/// A simple program to Download music from spotify
#[derive(Parser, Debug)]
#[command(author, version, about, about="A simple program to Download music from spotify",long_about = None,arg_required_else_help(true))]
struct Args {
    /// Download a playlist with url or code 
    #[arg(short, long)]
    playlist: Option<String>,

    /// Path to save the downloaded music 
    #[arg(short, long)]
    output: Option<String>,
    
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    if let Some(url) = args.playlist {
        let output = args.output.unwrap_or_else(|| "C:\\Users\\serpe\\Music".into());
        service::commands::download_playlist(&url, &output).await;
    }
    Ok(())
}