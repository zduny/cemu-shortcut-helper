use std::{env, error::Error, fs, path::PathBuf};

use cemu::{installed_games, Game};
use clap::{App, Arg, crate_version};
use client::create_client;
use game::game_url;
use mslnk::ShellLink;
use reqwest::blocking::Client;

use crate::image::download_icons;

mod cemu;
mod client;
mod game;
mod image;

pub const GAMEFAQS_URL: &str = "https://gamefaqs.gamespot.com";

fn sanitize_name(name: &str) -> String {
    name.replace(":", " -")
}

fn create_shortcut(
    game: &Game,
    icon_paths: &Vec<PathBuf>,
    full_screen: bool,
) -> Result<(), Box<dyn Error>> {
    let mut cemu_path = PathBuf::new();
    cemu_path.push(env::current_dir()?);
    cemu_path.push("Cemu.exe");

    let mut shortcut = ShellLink::new(cemu_path)?;

    let full_screen = if full_screen { " -f" } else { "" };
    shortcut.set_arguments(Some(format!("-g \"{}\"{}", &game.path, full_screen)));

    if icon_paths.len() > 0 {
        shortcut.set_icon_location(icon_paths[0].to_str().map(|path| path.to_string()))
    }

    let shorcut_path = format!("shortcuts/{}.lnk", sanitize_name(&game.name));
    shortcut.create_lnk(shorcut_path)?;

    Ok(())
}

fn process_game(client: &Client, game: &Game, full_screen: bool) -> Result<(), Box<dyn Error>> {
    let game_url = game_url(client, &game.name)?;
    let icon_paths = download_icons(&client, &sanitize_name(&game.name), &game_url)?;
    create_shortcut(game, &icon_paths, full_screen)?;

    Ok(())
}

fn create_shortcuts(full_screen: bool) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("shortcuts")?;

    let games = installed_games()?;
    let count = games.len();
    println!("Found {} installed games.", count);

    let client = create_client()?;
    for (i, game) in games.iter().enumerate() {
        println!(
            "\nProcessing game ({}/{}) \"{}\"...",
            i + 1,
            count,
            &game.name
        );
        match process_game(&client, &game, full_screen) {
            Ok(_) => println!("Done."),
            Err(error) => eprintln!("Error occured: {}", error),
        }
    }

    Ok(())
}

fn main() {
    let matches = App::new("Shortcut Helper for Cemu")
        .version(crate_version!())
        .about("Creates Windows shortcuts for games installed in Cemu.")
        .arg(Arg::new("fullscreen")
            .short('f')
            .long("fullscreen")
            .about("Create shortcuts that will launch games in fullscreen mode")
            .takes_value(false))
        .get_matches();

    let full_screen = matches.is_present("fullscreen");

    match create_shortcuts(full_screen) {
        Err(error) => eprintln!("Program aborted due to error: {}", error),
        _ => (),
    }
}
