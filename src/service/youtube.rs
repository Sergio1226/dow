use reqwest::Client;
use reqwest::header::CONTENT_TYPE;

use crate::service::scrap::get_yt_data;
use crate::service::scrap::get_video_id;

const SEARCH_URL: &str = "https://www.youtube.com/results?search_query=";

pub struct Youtube {
    pub client : Client
}

impl Youtube {
    pub fn new() -> Self {
        Youtube {
            client: reqwest::Client::builder().user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36").build().unwrap()
        }
    }


    async fn search(&self, search: &str) -> Result<String, reqwest::Error> {
        let url=format!("{}{}", SEARCH_URL, search);
        let response=self.client.get(url).header(CONTENT_TYPE, "application/json").send().await?;
        let act=response.text().await?;
        Ok(get_yt_data(&act))
    }

    pub async fn get_video_title(&self, search: &str) -> Result<String, reqwest::Error> { 
        let data=self.search(search).await?;
        Ok(get_video_id(&data).unwrap_or_default())
    }

}
