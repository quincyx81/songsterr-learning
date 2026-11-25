use crate::utils::cookie_jar::fill_from_domain;
use crate::tab;
use serde::{Serialize, Deserialize};
use scraper::{Html, Selector};
use std::sync::Arc;
use reqwest::Url;
use reqwest::Response;
use serde_json::Value;


pub async fn pull_data_from_cloudfront(href: &str, data_id: u32, client: &mut reqwest::Client) {
    let [image, revision_id]: [String; 2] = get_cloudfront_url_parts(href, client).await;
    //  println!("https://dqsljvtekg760.cloudfront.net/{}/{}/{}/1.json", &json[0].data_id, revision_id, image);
    let link = format!("https://dqsljvtekg760.cloudfront.net/{}/{}/{}/0.json", data_id, revision_id, image);
    let url = link.parse::<Url>().unwrap();
    let resp = client.get(url).send().await.unwrap();
    let body = resp.text().await.unwrap();
    let value: Value = serde_json::from_str(&body).unwrap();
    let pretty = serde_json::to_string_pretty(&value).unwrap();
    std::fs::write("playground/current.json", pretty).unwrap();
    tab::json::load(&body);
}

pub async fn get_client(url: &str) -> reqwest::Client {
    let url = url.parse::<Url>().unwrap();
    let jar = fill_from_domain(&url, &""); 
    reqwest::Client::builder()
        .cookie_provider(Arc::new(jar))
        .build()
        .unwrap()
}


// Songsterr pulls tab data for each track separately from some
// https://dqsljvtekg760.cloudfront.net/<data_id>/<revision_id>/<image>/<track_num>.json
// while we have data_id and can generate track_num from overall track number 
// we also need to pull revision_id and image that are located inside 
// document pulled from <SavedSong>.href url and get both mising parts.
pub async fn get_cloudfront_url_parts(href: &str, client: &mut reqwest::Client) -> [String; 2] {
    let link = format!("https://www.songsterr.com{}", href);
    let url = link.parse::<Url>().unwrap();
    let resp = client.get(url).send().await.unwrap(); 
    parse_song_href_page(resp).await 
}

async fn parse_song_href_page(resp: Response) -> [String; 2] {
    let page_content = resp.text().await.unwrap();
    let document = Html::parse_document(&page_content);
    let json_data_selector = Selector::parse("[id='state']").unwrap();
    let json_text = document.select(&json_data_selector)
        .next()
        .unwrap()
        .text()
        .collect::<String>()
        .trim()
        .to_string();
    
    let data: Value = serde_json::from_str(&json_text).unwrap();
    let image = data["meta"]["current"]["image"]
        .as_str().unwrap()
        .to_string(); // without this returns &str 

    let revision_id = data["meta"]["current"]["revisionId"]
        .as_u64().unwrap()
        .to_string();

    return [image, revision_id]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SavedSong {
    pub name: String,
    pub artist: String,
    pub href: String,
    pub data_id: u32,

}

pub async fn get_fav_songs(link: &str, client: &mut reqwest::Client) -> Vec<SavedSong> {
    let url = link.parse::<Url>().unwrap();
    let resp = client.get(url).send().await.unwrap();
    parse_favourites_page(resp).await
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
