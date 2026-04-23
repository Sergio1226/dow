use std::path::PathBuf;

use crate::constants::DOW_CACHE;

static YT_DLP: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "\\lib\\yt-dlp.exe"
));
static FFMPEG: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "\\lib\\ffmpeg.exe"
));

pub fn create_libs() -> std::io::Result<PathBuf> {
    let build_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("lib");

    let yt_dlp_path = build_path.join("yt-dlp\\yt-dlp.exe");
    let ffmpeg_path = build_path.join("ffmpeg\\ffmpeg.exe");

    let mut is_build = true;

    let save_path = std::env::temp_dir().join(DOW_CACHE).join("lib");
    if !yt_dlp_path.exists() {
        is_build = false;
        let yt_dlp_save_path = save_path.join("yt-dlp.exe");
        if !yt_dlp_save_path.exists() {
            std::fs::create_dir_all(&save_path)?;
            std::fs::write(yt_dlp_save_path, YT_DLP)?;
        }
    }

    if !ffmpeg_path.exists() {
        is_build = false;
        let ffmpeg_save_path = save_path.join("ffmpeg.exe");
        if !ffmpeg_save_path.exists() {
            std::fs::create_dir_all(&save_path)?;
            std::fs::write(ffmpeg_save_path, FFMPEG)?;
        }
    }
    Ok(if is_build { build_path } else { save_path })
}
