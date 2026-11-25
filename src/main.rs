mod utils;
mod api;
mod tui;
mod tab;

use crate::api::{get_client, get_fav_songs};
use crate::tui::choose_tab::draw_test_span;

#[tokio::main]
async fn main() {
    
    // let api_url = "https://www.songsterr.com/api/favorites";
    let html_url = "https://www.songsterr.com/favorites";
    let mut client = get_client("https://www.songsterr.com/").await;
    let json = get_fav_songs(&html_url, &mut client).await;
    draw_test_span(&json, &mut client).unwrap();
}


