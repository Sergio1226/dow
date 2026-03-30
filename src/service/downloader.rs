#![allow(dead_code)]
use crate::constants::{DOW_CACHE, MAX_DOWLOADS};

use crate::service::progress::DownloadProgress;
use crate::service::utils::format_text;

use std::fs::{File, remove_dir_all};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::Mutex;
use zip::write::ExtendedFileOptions;
use zip::write::FileOptions;
use zip::write::ZipWriter;

struct DownloadData {
    name: String,
    bytes: Vec<u8>,
}

/// A struct to download audios from YouTube
struct Downloader {
    yt_dlp: PathBuf,
    ffmpeg: PathBuf,
    cache: PathBuf,
}

impl Downloader {
    /// Creates a new instance of Downloader
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let yt_dlp = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("lib\\yt-dlp\\yt-dlp.exe");
        let ffmpeg = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("lib\\ffmpeg");

        let output_cache = std::env::temp_dir().join(DOW_CACHE);
        std::fs::create_dir_all(&output_cache)?;

        let output_cache = std::env::temp_dir().join(DOW_CACHE).join("batch");
        let downloader = Downloader {
            yt_dlp,
            ffmpeg,
            cache: output_cache,
        };
        downloader.clean_batch().ok();
        Ok(downloader)
    }

    ///Clean the cache of dow
    pub fn clean_batch(&self) -> Result<(), std::io::Error> {
        remove_dir_all(&self.cache)
    }

    pub async fn get_audios(
        &self,
        names: &[String],
        progress: &DownloadProgress,
    ) -> Result<Vec<DownloadData>, Box<dyn std::error::Error>> {
        let output_cache = &self.cache;
        self.download_batch(names, progress).await?;
        let mut audios = Vec::new();

        for entry in std::fs::read_dir(&output_cache)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let filename = path.file_name().unwrap().to_str().unwrap();
                let code = filename[0..5].parse::<usize>().unwrap();
                let mut file = File::open(&path)?;
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes)?;
                std::fs::remove_file(&path)?;
                audios.push(DownloadData {
                    name: format!("{}.mp3", names[code - 1].clone()),
                    bytes,
                });
            }
        }
        Ok(audios)
    }

    pub async fn download_batch(
        &self,
        names: &[String],
        progress: &DownloadProgress,
    ) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::create_dir_all(&self.cache)?;

        let mut child = Command::new(&self.yt_dlp)
            .args(&[
                "--ffmpeg-location",
                self.ffmpeg.to_str().unwrap(),
                "--extract-audio",
                "--audio-format",
                "mp3",
                "-N",
                "5",
                "--output",
                &format!("{}\\%(autonumber)s.mp3", self.cache.display()),
            ])
            .args(
                names
                    .iter()
                    .map(|n| format!("ytsearch1:{} Official video", n)),
            )
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn()?;

        let stdout = child.stdout.take().ok_or("Error stdout")?;
        let mut reader = BufReader::new(stdout);
        let mut line_bytes = Vec::new();

        while let Ok(n) = reader.read_until(b'\n', &mut line_bytes).await {
            if n == 0 {
                break;
            }

            let line = String::from_utf8_lossy(&line_bytes);
            if line.contains("[ExtractAudio]") {
                progress.inc(1);
            }
            line_bytes.clear(); 
        }

        child.wait().await?;
        Ok(())
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
    /// * `name` - The name of the zip file
    fn new(mut path: PathBuf, name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        path.push(format!("{}.zip", format_text(name)));
        let zip_file = File::create(&path).map_err(|_| "Path not found")?;
        let zip = ZipWriter::new(zip_file);
        Ok(SaveAudio { zip: Some(zip) })
    }

    /// Adds a downloaded audio to the zip file
    /// ### Arguments
    /// * `data` - The DownloadData struct
    fn add_to_zip(&mut self, data: &DownloadData) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(zip) = self.zip.as_mut() {
            let options: FileOptions<ExtendedFileOptions> = FileOptions::default()
                .compression_method(zip::CompressionMethod::Stored)
                .unix_permissions(0o755);
            zip.start_file(&data.name, options)?;
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
    name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }

    let progress = DownloadProgress::new(ids_audios.len() as u64);

    let downloader = Downloader::new().await?;

    let saver = Arc::new(Mutex::new(SaveAudio::new(path, &name)?));

    let saver_for_signal = Arc::clone(&saver);
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Error al escuchar Ctrl-C");
        println!("\nInterrupted ...");
        let mut lock = saver_for_signal.lock().await;
        let _ = lock.close_zip();
        std::process::exit(0);
    });

    for chunk in ids_audios.chunks(MAX_DOWLOADS) {
        let results = downloader.get_audios(chunk, &progress).await?;
        {
            let mut lock = saver.lock().await;
            for song in results {
                lock.add_to_zip(&song)?;
            }
        }
    }

    let mut final_lock = saver.lock().await;
    final_lock.close_zip()?;
    progress.close("Download finished".into());
    Ok(())
}
