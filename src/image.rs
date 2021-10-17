use ico::{IconDir, IconDirEntry, IconImage, ResourceType};
use lazy_static::lazy_static;
use reqwest::blocking::Client;
use scraper::{ElementRef, Selector};
use std::{error::Error, fs::{self, File}};
use image::{DynamicImage, GenericImageView, imageops::FilterType};

use crate::GAMEFAQS_URL;

lazy_static! {
    static ref IMAGE_LINK_SELECTOR: Selector = Selector::parse(".vid_img > a").unwrap();
    static ref THUMBNAIL_SELECTOR: Selector = Selector::parse("img.imgboxart").unwrap();
    static ref IMAGE_SELECTOR: Selector = Selector::parse("div.img > img").unwrap();
}

fn is_limited_edition(alt: &str) -> bool {
    alt.contains("(Limited Edition)")
}

fn is_steelbook_edition(alt: &str) -> bool {
    alt.contains("(Steelbook Edition)")
}

fn is_premium_pack(alt: &str) -> bool {
    alt.contains("(Premium Pack)")
}

fn is_eu(alt: &str) -> bool {
    alt.contains("(EU)")
}

fn links_filter(element: &ElementRef) -> bool {
    let mut image = element.select(&THUMBNAIL_SELECTOR);
    let alt = image.next().unwrap().value().attr("alt").unwrap();

    is_eu(alt) && (!is_limited_edition(alt) && !is_premium_pack(alt) && !is_steelbook_edition(alt))
}

fn image_url(client: &Client, url: &str) -> Result<String, Box<dyn Error>> {
    let response = client.get(url).send()?;

    let body = response.text()?;

    let html = scraper::Html::parse_document(&body);

    let img = html.select(&IMAGE_SELECTOR).next().unwrap();
    
    let src = format!("{}{}", GAMEFAQS_URL, img.value().attr("src").unwrap());

    Ok(src)
}

pub fn icon_urls(client: &Client, game_url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!("{}/images", game_url);
    let response = client.get(url).send()?;

    let body = response.text()?;

    let html = scraper::Html::parse_document(&body);

    let links = html.select(&IMAGE_LINK_SELECTOR);
    let links = links
        .filter(links_filter)
        .map(|e| e.value().attr("href").unwrap().to_string())
        .map(|s| format!("{}{}", GAMEFAQS_URL, s));

    let urls = links.map(|url| image_url(client, &url).unwrap()).collect();

    Ok(urls)
}

fn download_and_decode(client: &Client, url: &str) -> Result<DynamicImage, Box<dyn Error>> {
    let response = client.get(url).send()?;
    let bytes = response.bytes()?;
    let image = image::load_from_memory(&bytes)?;

    Ok(image)
}

fn is_square(image: &DynamicImage) -> bool {
    image.width() == image.height()
}

fn add_entry(icon_dir: &mut IconDir, image: &DynamicImage, size: u32) {
    let image = image.resize(size, size, FilterType::CatmullRom);
    let rgba_data = image.into_rgba8().into_raw();
    let icon_image = IconImage::from_rgba_data(size, size, rgba_data);
    icon_dir.add_entry(IconDirEntry::encode(&icon_image).unwrap());
}

fn save_as_ico_and_return_relative_path(name: &str, image: &DynamicImage) -> String {
    let mut icon_dir = IconDir::new(ResourceType::Icon);
    add_entry(&mut icon_dir, image, 256);
    add_entry(&mut icon_dir, image, 48);
    add_entry(&mut icon_dir, image, 32);
    add_entry(&mut icon_dir, image, 16);

    let path = format!("icons/{}.ico", name);
    let file = File::create(path.clone()).unwrap();
    icon_dir.write(file).unwrap();

    path
}

pub fn download_icons(client: &Client, game_url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let icon_urls = icon_urls(client, game_url)?;
    let icons = icon_urls.iter()
        .map(|url| download_and_decode(client, url).unwrap());

    let icons = icons.filter(is_square);

    fs::create_dir_all("icons")?;
    let paths = icons.enumerate().map(|(i, icon)| {
        let name = format!("abc {}", i);
        save_as_ico_and_return_relative_path(&name, &icon)
    }).collect();

    Ok(paths)
}
