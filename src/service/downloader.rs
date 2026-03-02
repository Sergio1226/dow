use crate::service::youtube::Youtube;

use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use yt_dlp::client::deps::Libraries;
use zip::write::ExtendedFileOptions;
use zip::write::FileOptions;
use zip::write::ZipWriter;

const TRACK_URL: &str = "https://www.youtube.com/watch?v=";

pub struct Downloader {
    youtube: Youtube,
    fetcher: yt_dlp::Downloader,
    zip_writer: Arc<Mutex<Option<ZipWriter<File>>>>, 
}

impl Downloader {
    pub async fn new(
        youtube: Youtube,
        output_dir: &str,
        zip_name: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let libraries_dir = PathBuf::from("lib");
        let youtube_dir = libraries_dir.join("yt-dlp\\yt-dlp.exe");
        let ffmpeg_dir = libraries_dir.join("ffmpeg\\ffmpeg.exe");

        let output_cache = std::env::temp_dir().join("dow_cache");
        std::fs::create_dir_all(&output_cache)?;

        let libraries = Libraries::new(youtube_dir, ffmpeg_dir);
        let fetcher = yt_dlp::Downloader::builder(libraries, output_cache)
            .build()
            .await?;

        let zip_file = File::create(format!("{}\\{}.zip", output_dir, zip_name))
            .map_err(|_| "Path not found")?;
        let zip_writer = Arc::new(Mutex::new(Some(ZipWriter::new(zip_file)))); 

        Ok(Downloader {
            youtube,
            fetcher,
            zip_writer,
        })
    }

    pub async fn download_audio(
        &self,
        id: String,
        name: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}{}", TRACK_URL, id);
        let audio_path = self
            .fetcher
            .download_audio_stream_from_url(url, format!("{}.mp3", name))
            .await?;
        {
            let mut zip_guard = self.zip_writer.lock().await;
            if let Some(zip) = zip_guard.as_mut() {
                let options: FileOptions<'_, ExtendedFileOptions> =
                    FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

                zip.start_file(&format!("{}.mp3", name), options)?;

                let mut f = File::open(&audio_path)?;
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer)?;
                zip.write_all(&buffer)?;
            }
        }
        if let Err(e) = std::fs::remove_file(&audio_path) {
            eprintln!(
                "Error removing temporary file {}: {}",
                audio_path.display(),
                e
            );
        }
        Ok(())
    }

    pub async fn close_zip(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut zip_guard = self.zip_writer.lock().await;
        if let Some(zip) = zip_guard.take() {
            zip.finish()?;
        }
        Ok(())
    }
}

pub async fn download_audios_from_ids(
    downloader: Arc<Downloader>,
    ids: Vec<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let clone = Arc::clone(&downloader);
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Error setting Ctrl-C handler");
        println!("Download interrupted, closing ZIP file...");
        if let Err(e) = clone.close_zip().await {
            eprintln!("Error closing ZIP: {}", e);
        }
        std::process::exit(0);
    });

    let amount = ids.len() / 4;

    for i in 0..amount {
        let mut handles = vec![];

        for j in 0..4 {
            let name = ids[i * 4 + j].clone();
            let id = downloader.youtube.get_video_title(&name).await?;

            let s = Arc::clone(&downloader);

            let handle = tokio::spawn(async move {
                let formatted_name=crate::service::utils::format_text(&name);
                println!("Downloaded {}", formatted_name);
                s.download_audio(id, formatted_name).await
            });

            handles.push(handle);
        }

        tokio::time::sleep(std::time::Duration::from_secs(7)).await;
        futures::future::join_all(handles).await;
    }
    downloader.close_zip().await?;
    Ok(())
}