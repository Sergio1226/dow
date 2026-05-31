use crate::globals::WITH_IMAGE;
use crate::models::spotify::{PlayList, PlayListData, Song, TagSong};
use crate::service::spotify::download_image;

use id3::frame::{Picture, PictureType};
use id3::{Tag, TagLike, Version};
use std::collections::HashSet;
use std::path::PathBuf;

/// Converts raw playlist data into a structured PlayList format, ensuring unique track titles and proper formatting.
pub fn raw_to_playlist(playlist_data: PlayListData) -> PlayList {
    let mut seen: HashSet<String> = HashSet::new();
    let songs: Vec<Song> = playlist_data
        .props
        .page_props
        .state
        .data
        .entity
        .track_list
        .iter()
        .filter_map(|track| {
            let formatted_title = format!(
                "{} {}",
                format_text(&track.title),
                format_text(&track.subtitle)
            );
            if seen.insert(formatted_title.clone()) {
                Some(Song {
                    title: formatted_title,
                    track_id: track.uri.split(':').last().unwrap_or("").into(),
                    ..Default::default()
                })
            } else {
                None
            }
        })
        .collect();
    PlayList {
        songs,
        name: playlist_data
            .props
            .page_props
            .state
            .data
            .entity
            .name
            .clone(),
    }
}

/// Formats a text by removing control characters, replacing multiple spaces with a single space, and replacing special HTML entities
pub fn format_text(text: &str) -> String {
    text.chars()
        .filter_map(|c| {
            if c.is_control() {
                None
            } else if c.is_whitespace() {
                Some(' ')
            } else {
                Some(c)
            }
        })
        .collect::<String>()
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|', ' '], " ")
        .trim()
        .to_string()
}

pub async fn save_tag(info: TagSong, path: &PathBuf) {
    let mut tag = Tag::new();
    tag.set_album(info.album);
    tag.set_title(info.title);
    tag.set_artist(info.artists);
    tag.set_year(info.year as i32);

    let mut with_image = false;
    {
        with_image = *WITH_IMAGE.lock().unwrap();
    }
    if with_image
        && let Some(x) = info.image
        && let Ok(y) = download_image(&x).await
    {
        let data = y;
        for pic_type in [
            PictureType::CoverFront,
            PictureType::Other,
            PictureType::Artist,
        ] {
            let picture = Picture {
                mime_type: "image/jpeg".to_string(),
                picture_type: pic_type,
                description: "cover".to_string(),
                data: data.clone(),
            };

            tag.add_frame(picture);
        }
    }

    tag.write_to_path(path, Version::Id3v24).unwrap();
}
