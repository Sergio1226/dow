use futures::StreamExt;
use reqwest::get;
use std::io::Write;
use std::path::PathBuf;

use crate::constants::{IS_WINDOWS, REPO_LIB};
use crate::os::{
    get_lib_path, get_name_ffmpeg, get_name_yt_dlp, get_path_ffmpeg, get_path_yt_dlp, system_binary,
};

pub struct Libs {
    pub yt_dlp: Option<PathBuf>,
    pub ffmpeg: Option<PathBuf>,
}

pub async fn create_libs() -> std::io::Result<Libs> {
    std::fs::create_dir_all(get_lib_path())?;
    let mut libs = consult_libs();

    if libs.ffmpeg.is_none() {
        libs.ffmpeg = Some(install_ffmepg().await?);
    }
    if libs.yt_dlp.is_none() {
        libs.yt_dlp = Some(install_yt_dlp().await?);
    }
    Ok(libs)
}

pub async fn install_ffmepg() -> std::io::Result<PathBuf> {
    if !IS_WINDOWS {
        println!(
            "ffmpeg not found. Please install it using your distribution's package manager. Or, enter Y to let this program download it automatically."
        );
        let input = std::io::stdin();
        let mut buffer = String::new();
        input.read_line(&mut buffer).unwrap();
        if buffer.trim() == "Y" || buffer.trim() == "y" {
            println!("Downloading ffmpeg ...");
            let path = download_ffmpeg().await;
            println!("Downloaded ffmpeg");
            Ok((path))
        } else {
            std::process::exit(1);
        }
    } else {
        println!("FFMPEG not found, Downloading... ");
        let path = download_ffmpeg().await;
        println!("Downloaded ffmpeg");
        Ok((path))
    }
}

pub async fn install_yt_dlp() -> std::io::Result<PathBuf> {
    if !IS_WINDOWS {
        println!(
            "yt-dlp not found. Please install it using your distribution's package manager. Or, enter Y to let this program download it automatically."
        );
        let input = std::io::stdin();
        let mut buffer = String::new();
        input.read_line(&mut buffer).unwrap();
        if buffer.trim() == "Y" || buffer.trim() == "y" {
            println!("Downloading yt-dlp ...");
            let path = download_yt_dlp().await;
            println!("Downloaded yt-dlp");
            Ok((path))
        } else {
            std::process::exit(1);
        }
    } else {
        println!("yt-dlp not found, Downloading... ");
        let path = download_ffmpeg().await;
        println!("Downloaded yt-dlp");
        Ok((path))
    }
}

async fn download_ffmpeg() -> PathBuf {
    let url = format!("{}/{}", REPO_LIB, get_name_ffmpeg().to_str().unwrap());
    let path = get_path_ffmpeg();
    if let Err(e)=download(&url, &path).await {
    
        panic!("Error downloading yt-dlp {}",e);
    }
    path
}

async fn download_yt_dlp() -> PathBuf {
    let url = format!("{}/{}", REPO_LIB, get_name_yt_dlp().to_str().unwrap());
    let path = get_path_yt_dlp();
    if let Err(e)=download(&url, &path).await {
        panic!("Error downloading yt-dlp {}",e);
    }
    path
}

async fn download(url: &str, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(url).await?;
    let total = response.content_length().unwrap_or(0);
    let mut downloaded = 0u64;
    let mut stream = response.bytes_stream();
    let mut file = std::fs::File::create(path)?;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;

        file.write_all(&chunk)?;

        downloaded += chunk.len() as u64;

        if total > 0 {
            let percent = downloaded as f64 * 100.0 / total as f64;

            print!(
                "\rDownloading... {:.1}% ({:.2}/{:.2} MB)",
                percent,
                downloaded as f64 / 1024.0 / 1024.0,
                total as f64 / 1024.0 / 1024.0
            );
            std::io::stdout().flush()?;
        }
    }
    Ok(())
}

fn consult_libs() -> Libs {
    let mut yt_dlp: Option<PathBuf> = None;
    let mut ffmpeg: Option<PathBuf> = None;
    if !IS_WINDOWS {
        if let Some(x) = system_binary("yt-dlp").ok() {
            yt_dlp = Some(x);
        }
        if let Some(x) = system_binary("ffmpeg").ok() {
            ffmpeg = Some(x);
        }
    }
    let cache_path = get_lib_path();
    if yt_dlp.is_none() && cache_path.join(get_name_yt_dlp()).exists() {
        yt_dlp = Some(cache_path.join(get_name_yt_dlp()));
    }
    if ffmpeg.is_none() && cache_path.join(get_name_ffmpeg()).exists() {
        ffmpeg = Some(cache_path.join(get_name_ffmpeg()));
    }
    Libs { yt_dlp, ffmpeg }
}
