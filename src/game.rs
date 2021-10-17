use lazy_static::lazy_static;
use reqwest::blocking::Client;
use scraper::Selector;
use std::{collections::HashMap, error::Error, io::ErrorKind};

use crate::GAMEFAQS_URL;

lazy_static! {
    static ref GAME_LINK_SELECTOR: Selector = Selector::parse(".sr_title > a.log_search").unwrap();
}

pub fn game_url(client: &Client, name: &str) -> Result<String, Box<dyn Error>> {
    let mut params = HashMap::new();
    params.insert("game_type", "2");
    params.insert("game", name);
    params.insert("platform", "118");
    params.insert("distribution", "22");
    params.insert("category", "0");
    params.insert("date_type", "0");
    params.insert("date_1", "");
    params.insert("date_2", "");
    params.insert("date_year", "0");
    params.insert("contents_type", "0");
    params.insert("contents", "0");
    params.insert("region_type", "0");
    params.insert("region", "0");
    params.insert("company_type", "0");
    params.insert("company_text", "");
    params.insert("company", "");
    params.insert("sort", "0");
    params.insert("min_scores", "0");

    let url = format!("{}/search_advanced", GAMEFAQS_URL);
    let response = client.post(url).form(&params).send()?;

    let body = response.text()?;

    let html = scraper::Html::parse_document(&body);

    let result = if let Some(element) = html.select(&GAME_LINK_SELECTOR).next() {
        let href = element.value().attr("href").unwrap();
        Ok(format!("{}{}", GAMEFAQS_URL, href.to_string()))
    } else {
        Err(
            std::io::Error::new(ErrorKind::NotFound, format!("Game \"{}\" not found.", name))
                .into(),
        )
    };
    result
}
