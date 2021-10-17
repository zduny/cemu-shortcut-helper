use client::create_client;
use game::game_url;

use crate::image::{download_icons, icon_urls};

mod client;
mod game;
mod image;

pub const GAMEFAQS_URL: &str = "https://gamefaqs.gamespot.com";

fn main() {
    let client = create_client().unwrap();
    let game_url = game_url(&client, "Mario Kart 8").unwrap();
    println!("{}", game_url);

    let img_urls = icon_urls(&client, &game_url).unwrap();
    for url in img_urls {
        println!("{}", url);
    }

    download_icons(&client, &game_url).unwrap();
}
