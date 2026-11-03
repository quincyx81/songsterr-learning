mod utils;
mod api;
mod tui;

use crate::api::{get_fav_songs};
use crate::tui::choose_tab::draw_test_span;

#[tokio::main]
async fn main() {
    
    // let api_url = "https://www.songsterr.com/api/favorites";
    let html_url = "https://www.songsterr.com/favorites";
    let json = get_fav_songs(&html_url).await;
    draw_test_span(&json).unwrap();
}


