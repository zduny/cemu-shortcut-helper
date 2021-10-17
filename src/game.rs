use lazy_static::lazy_static;
use reqwest::blocking::Client;
use scraper::Selector;
use std::{collections::HashMap, error::Error};

use crate::GAMEFAQS_URL;

lazy_static! {
    static ref GAME_LINK_SELECTOR: Selector = Selector::parse(".sr_title > a.log_search").unwrap();
}

pub fn game_url(client: &Client, name: &str) -> Result<String, Box<dyn Error>> {
    let mut params = HashMap::new();
    params.insert("game", name);
    params.insert("platform", "118");

    let url = format!("{}/search_advanced", GAMEFAQS_URL);
    let response = client.post(url).form(&params).send()?;

    let body = response.text()?;

    let html = scraper::Html::parse_document(&body);

    if let Some(element) = html.select(&GAME_LINK_SELECTOR).next() {
        if let Some(href) = element.value().attr("href") {
            Ok(format!("{}{}", GAMEFAQS_URL, href.to_string()))
        } else {
            Err("Game link had no \"href\" attribute.".into())
        }
    } else {
        Err(format!("Game \"{}\" not found.", name).into())
    }
}
