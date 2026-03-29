use crate::constants::{DOW_CACHE, MAX_DOWLOADS, TRACK_URL, WAIT_TIME_SECS};

use crate::service::utils::format_text;

use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use yt_dlp::client::deps::Libraries;
use yt_dlp::model::video::Video;
use zip::write::ExtendedFileOptions;
use zip::write::FileOptions;
use zip::write::ZipWriter;

struct DownloadData {
    name: String,
    bytes: Vec<u8>,
}

/// A struct to download audios from YouTube
struct Downloader {
    fetcher: yt_dlp::Downloader,
}

impl Downloader {
    /// Creates a new instance of Downloader
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let libraries_dir = PathBuf::from("lib");
        let youtube_dir = libraries_dir.join("yt-dlp\\yt-dlp.exe");
        let ffmpeg_dir = libraries_dir.join("ffmpeg\\ffmpeg.exe");

        let output_cache = std::env::temp_dir().join(DOW_CACHE);
        std::fs::create_dir_all(&output_cache)?;

        let libraries = Libraries::new(youtube_dir, ffmpeg_dir);
        let fetcher = yt_dlp::Downloader::builder(libraries, output_cache)
            .build()
            .await?;

        Ok(Downloader { fetcher })
    }

    /// Fetches the YouTube video information based on the provided name
    async fn get_yt_info(&self, name: &str) -> Result<Video, Box<dyn std::error::Error>> {
        let video = self
            .fetcher
            .fetch_video_infos(format!("ytsearch:{}", name))
            .await?;
        Ok(video)
    }

    /// Downloads an audio from YouTube
    /// ### Arguments
    /// * `id` - The ID of the YouTube video to download
    /// * `name` - The name to save the downloaded audio as
    /// ### Returns
    /// * `DownloadData` - The name and bytes of the downloaded audio
    pub async fn download_audio(
        &self,
        name: String,
    ) -> Result<DownloadData, Box<dyn std::error::Error>> {
        let video = self.get_yt_info(&name).await?;
        let url = format!("{}{}", TRACK_URL, video.id);
        let audio_path = self
            .fetcher
            .download_audio_stream_from_url(url, format!("{}.mp3", name))
            .await?;
        let mut file = File::open(&audio_path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        let _ = std::fs::remove_file(&audio_path);

        Ok(DownloadData { name, bytes: bytes })
    }
}

/// A struct to save the downloaded audios in a zip file
///
/// ### Fields
/// * `zip` - An optional ZipWriter to write the downloaded audios in a zip
/// ### Methods
/// * `new` - A method to create a new instance of SaveAudio with the provided
/// path. It creates a zip file at the specified path and initializes the ZipWriter.
/// * `add_to_zip` - A method to add a downloaded audio to the zip file
/// * `close_zip` - A method to close the zip file and finish writing the downloaded audios
struct SaveAudio {
    zip: Option<ZipWriter<File>>,
}

impl SaveAudio {
    /// Creates a new instance of SaveAudio with the provided path
    /// ### Arguments
    /// * `path` - The path where the zip file will be created
    fn new(mut path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        path.push("playlist.zip");
        let zip_file = File::create(&path).map_err(|_| "Path not found")?;
        let zip = ZipWriter::new(zip_file);
        Ok(SaveAudio { zip: Some(zip) })
    }

    /// Adds a downloaded audio to the zip file
    /// ### Arguments
    /// * `data` - The DownloadData struct
    fn add_to_zip(&mut self, data: DownloadData) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(zip) = self.zip.as_mut() {
            let options: FileOptions<ExtendedFileOptions> = FileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated)
                .unix_permissions(0o755);
            zip.start_file(&format!("{}.mp3", data.name), options)?;
            zip.write_all(&data.bytes)?;
        }
        Ok(())
    }

    /// Closes the zip file and finishes writing the downloaded audios
    pub fn close_zip(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(x) = self.zip.take() {
            x.finish()?;
        }
        Ok(())
    }
}

/// Download audios from a list of IDs and save them in a zip file
/// ### Arguments
/// * `ids_audios` - A vector of audio IDs to download
/// * `path` - The path where the downloaded audios will be saved
pub async fn download_audios_from_ids(
    ids_audios: Vec<String>,
    path: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }

    let downloader = Downloader::new().await?;
    let saver = Arc::new(Mutex::new(SaveAudio::new(path)?));
    let saver_clone = Arc::clone(&saver);
    
    let ctrlc_handle = tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Error setting Ctrl-C handler");
        println!("\nInterrupted...");
        if let Err(e) = saver_clone.lock().await.close_zip() {
            eprintln!("Error closing zip: {}", e);
        }
        std::process::exit(0);
    });

    for chunk in ids_audios.chunks(MAX_DOWLOADS) {
        let futures: Vec<_> = chunk
            .iter()
            .map(|name| async { downloader.download_audio(format_text(name)).await })
            .collect();
        let results = futures::future::join_all(futures).await;
        for song in results {
            match song {
                Ok(data) => {
                    println!("Downloaded: {}", data.name);
                    if let Err(e) = saver.lock().await.add_to_zip(data) {
                        eprintln!("Error adding to zip: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Error downloading audio: {}", e);
                }
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(WAIT_TIME_SECS)).await;
    }
    ctrlc_handle.abort();
    saver.lock().await.close_zip()
}
