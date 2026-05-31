use std::path::PathBuf;

use crate::constants::{
    DOW_CACHE, DOW_LIB, FFMPEG_BIN_LINUX, FFMPEG_BIN_WINDOWS, IS_WINDOWS, YT_DLP_BIN_LINUX,
    YT_DLP_BIN_WINDOWS,
};

pub fn get_path_yt_dlp() -> PathBuf {
    if IS_WINDOWS {
        get_name(YT_DLP_BIN_WINDOWS)
    } else {
        get_name(YT_DLP_BIN_LINUX)
    }
}

pub fn get_path_ffmpeg() -> PathBuf {
    if IS_WINDOWS {
        get_name(FFMPEG_BIN_WINDOWS)
    } else {
        get_name(FFMPEG_BIN_LINUX)
    }
}

pub fn get_name_yt_dlp() -> PathBuf {
    if IS_WINDOWS {
        PathBuf::from(YT_DLP_BIN_WINDOWS)
    } else {
        PathBuf::from(YT_DLP_BIN_LINUX)
    }
}

pub fn get_name_ffmpeg() -> PathBuf {
    if IS_WINDOWS {
        PathBuf::from(FFMPEG_BIN_WINDOWS)
    } else {
        PathBuf::from(FFMPEG_BIN_LINUX)
    }
}

pub fn get_lib_path() -> PathBuf {
    get_cache_path().join(DOW_LIB)
}

///Path to the cache folder
pub fn get_cache_path() -> PathBuf {
    std::env::temp_dir().join(DOW_CACHE)
}

fn get_name(name: &str) -> PathBuf {
    get_lib_path().join(name)
}

pub fn set_permissions(path: &PathBuf)->Result<(),std::io::Error> {
    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o777))?;
    }
    Ok(())
}

pub fn system_binary(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let output = std::process::Command::new("which")
        .arg(name)
        .output()?;
    if output.status.success() {
        let path = String::from_utf8(output.stdout)?.trim().to_string();
        Ok(PathBuf::from(path))
    } else {
        Err("Binary not found".into())
    }
}