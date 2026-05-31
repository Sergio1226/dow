pub const MAX_DOWLOADS: usize = 4;
pub const PLAYLIST_ID_LENGTH: usize = 22;

pub const SPOTIFY_URL: &str = "https://open.spotify.com/embed/";
pub const SPOTIFY_URL_TRACK: &str = "https://open.spotify.com/track/";

pub const REPO_LIB:&str="https://github.com/Sergio1226/dow/releases/download/v0.1.5";

pub const YT_DLP_BIN_WINDOWS:&str="yt-dlp.exe";
pub const FFMPEG_BIN_WINDOWS:&str="ffmpeg.exe";


pub const YT_DLP_BIN_LINUX:&str="yt-dlp_linux";
pub const FFMPEG_BIN_LINUX:&str="ffmpeg";

pub const DOW_CACHE: &str = "dow_cache";
pub const DOW_LIB: &str="lib";


pub const IS_WINDOWS:bool=cfg!(windows); 