use std::{error::Error, fs::File, io::BufReader};

use xmltree::{Element, XMLNode};

#[derive(Debug)]
pub struct Game {
    pub path: String,
    pub name: String,
}

fn game_from_entry(entry: &Element) -> Result<Game, Box<dyn Error>> {
    let name = entry
        .get_child("name")
        .map(|name| name.get_text().unwrap_or_default())
        .ok_or::<Box<dyn Error>>("Entry had no game name".into())?
        .to_string();

    let path = entry
        .get_child("path")
        .map(|path| path.get_text().unwrap_or_default())
        .ok_or::<Box<dyn Error>>("Entry had no game path".into())?
        .to_string();

    let game = Game { name, path };

    Ok(game)
}

pub fn installed_games() -> Result<Vec<Game>, Box<dyn Error>> {
    let settings = File::open("settings.xml")?;
    let reader = BufReader::new(settings);

    let root: Element = Element::parse(reader)?;

    if let Some(game_cache) = root.get_child("GameCache") {
        game_cache
            .children
            .iter()
            .filter_map(|child| match child {
                XMLNode::Element(element) => Some(element),
                _ => None,
            })
            .map(game_from_entry)
            .collect()
    } else {
        Err("Malformed \"settings.xml\" file.".into())
    }
}
