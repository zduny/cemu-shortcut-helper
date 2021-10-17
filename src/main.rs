use std::error::Error;

use client::create_client;
use game::game_url;
use cemu::installed_games;

use crate::image::download_icons;

mod client;
mod game;
mod image;
mod cemu;

pub const GAMEFAQS_URL: &str = "https://gamefaqs.gamespot.com";

fn sanitize_name(name: &str) -> String {
    name.replace(":", " -")
}

fn main() -> Result<(), Box<dyn Error>> {
    let client = create_client()?;

    let games = installed_games()?;
    for game in games {
        println!("{:?}", game);

        let game_url = game_url(&client, &game.name)?;
        download_icons(&client, &sanitize_name(&game.name), &game_url)?;
    }

    Ok(())
}
