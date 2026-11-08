mod utils;
mod api;
mod tui;

use crate::api::{get_client, get_fav_songs, get_cloudfront_url_parts};
use crate::tui::choose_tab::draw_test_span;

#[tokio::main]
async fn main() {
    
    // let api_url = "https://www.songsterr.com/api/favorites";
    let html_url = "https://www.songsterr.com/favorites";
    let mut client = get_client("https://www.songsterr.com/").await;
    let json = get_fav_songs(&html_url, &mut client).await;
    draw_test_span(&json).unwrap();
    // let [image, revision_id]: [String; 2] = get_cloudfront_url_parts(&json[0].href, &mut client).await;
    // println!("https://dqsljvtekg760.cloudfront.net/{}/{}/{}/1.json", &json[0].data_id, revision_id, image);
}


