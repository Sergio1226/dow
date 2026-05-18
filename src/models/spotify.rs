use id3::Tag;
use serde::Deserialize;
#[allow(dead_code, unused, unused_variables)]
#[derive(Debug, Deserialize)]
pub struct TagSong {
    pub title: String,
    pub artists: String,
    pub album: String,
    pub year: u32,
    pub image: Option<String>,
}


impl Default for TagSong {
    fn default() -> Self {
        Self {
            title: "song".into(),
            artists: "artist".into(),
            album: "album".into(),
            year: 2026,
            image: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PlayList {
    pub songs: Vec<Song>,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Song {
    pub title: String,
    pub track_id: String,
    pub tags: Option<TagSong>,
}

impl Default for Song {
    fn default() -> Self {
        Song {
            title: String::new(),
            track_id: String::new(),
            tags: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PlayListData {
    pub props: Props,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Props {
    pub page_props: PageProps,
}

#[derive(Debug, Deserialize)]
pub struct PageProps {
    pub state: State,
}

#[derive(Debug, Deserialize)]
pub struct State {
    pub data: Data,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub entity: Entity,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub name: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub id: String,
    pub uri: String,
    pub track_list: Vec<Track>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub uri: String,
    pub uid: String,
    pub title: String,
    pub subtitle: String,
    pub duration: u32,
    pub is_playable: bool,
    pub audio_preview: Option<AudioPreview>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct AudioPreview {
    pub url: String,
    pub format: String,
}
