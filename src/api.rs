use crate::utils::cookie_jar::fill_from_domain;
use serde::{Serialize, Deserialize};
use scraper::{Html, Selector};
use std::sync::Arc;
use reqwest::Url;
use reqwest::Response;


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SavedSong {
    pub name: String,
    pub artist: String,
    pub href: String,
    pub data_id: u32,

}

pub async fn get_fav_songs(url: &str) -> Vec<SavedSong> {
    let resp = get_favourites_html_page(url);
    parse_favourites_page(resp.await).await
}

async fn get_favourites_html_page(url: &str) -> Response  {
    let url = url.parse::<Url>().unwrap();
    let jar = fill_from_domain(&url, &""); 
    let client = reqwest::Client::builder()
        .cookie_provider(Arc::new(jar))
        .build()
        .unwrap();

    client.get(url).send().await.unwrap()
}

async fn parse_favourites_page(resp: Response) -> Vec<SavedSong> {
    let page_content = resp.text().await.unwrap();
    let document = Html::parse_document(&page_content);
    
    // selectors
    let fav_selector = Selector::parse("[data-list='favorites'] > *").unwrap();
    let name_selector = Selector::parse("[data-field='name']").unwrap();
    let artist_selector = Selector::parse("[data-field='artist']").unwrap();
    
    document.select(&fav_selector)
        .filter_map(|child| {
            let name = child.select(&name_selector)
                .next()?
                .text()
                .collect::<String>()
                .trim()
                .to_string();
            
            let artist = child.select(&artist_selector)
                .next()?
                .text()
                .collect::<String>()
                .trim()
                .to_string();
            

           
            let href = child.value().attr("href")?.to_string();
            let data_id = child.value().attr("data-id")?.parse::<u32>().unwrap();

            Some(SavedSong { name, artist, href, data_id })
        }).collect()
} 

//pub async fn get_content_from_url(url: &str) -> Vec<SavedSong> {
//    let url = url.parse::<Url>().unwrap();
//    let jar = fill_from_domain(&url, &""); 
//    let client = reqwest::Client::builder()
//        .cookie_provider(Arc::new(jar))
//        .build()
//        .unwrap();
//
//    let resp = client.get(url).send().await.unwrap();
//    resp.json().await.unwrap()
// }
