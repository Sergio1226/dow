
use serde::Deserialize;
#[allow(dead_code,unused,unused_variables)]

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