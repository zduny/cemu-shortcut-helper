use ico::{IconDir, IconDirEntry, IconImage, ResourceType};
use lazy_static::lazy_static;
use reqwest::blocking::Client;
use scraper::{ElementRef, Selector};
use std::{error::Error, fs::{self, File}, path::PathBuf};
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

    if let Some(image) = image.next() {
        if let Some(alt) = image.value().attr("alt") {
            is_eu(alt) && (!is_limited_edition(alt) && !is_premium_pack(alt) && !is_steelbook_edition(alt))
        } else {
             false
        }
    } else {
        false
    }
}

fn image_url(client: &Client, url: &str) -> Result<String, Box<dyn Error>> {
    let response = client.get(url).send()?;

    let body = response.text()?;

    let html = scraper::Html::parse_document(&body);

    if let Some(image) = html.select(&IMAGE_SELECTOR).next() {
        if let Some(src) = image.value().attr("src") {
            Ok(format!("{}{}", GAMEFAQS_URL, src))
        } else {
            Err("Image had no \"src\" attribute".into())
        }
    } else {
        Err("No images found".into())
    }
}

fn icon_urls(client: &Client, game_url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!("{}/images", game_url);
    let response = client.get(url).send()?;

    let body = response.text()?;

    let html = scraper::Html::parse_document(&body);

    let links = html.select(&IMAGE_LINK_SELECTOR);
    links.filter(links_filter)
        .map(|link| {
            if let Some(href) = link.value().attr("href") {
                let covers_url = format!("{}{}", GAMEFAQS_URL, href);
                image_url(client, &covers_url)
            } else {
                Err("Link had no \"href\" attribute".into())
            }
        })
        .collect()
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

fn add_entry(icon_dir: &mut IconDir, image: &DynamicImage, size: u32) -> Result<(), Box<dyn Error>> {
    let image = image.resize(size, size, FilterType::CatmullRom);
    let rgba_data = image.into_rgba8().into_raw();
    let icon_image = IconImage::from_rgba_data(size, size, rgba_data);
    icon_dir.add_entry(IconDirEntry::encode(&icon_image)?);

    Ok(())
}

fn save_as_ico_and_return_path(name: &str, image: &DynamicImage) -> Result<PathBuf, Box<dyn Error>> {
    let mut icon_dir = IconDir::new(ResourceType::Icon);
    add_entry(&mut icon_dir, image, 256)?;
    add_entry(&mut icon_dir, image, 48)?;
    add_entry(&mut icon_dir, image, 32)?;
    add_entry(&mut icon_dir, image, 16)?;

    let relative_path = format!("icons/{}.ico", name);
    let file = File::create(relative_path.clone())?;
    icon_dir.write(file)?;

    let mut path = PathBuf::new();
    path.push(std::env::current_dir()?);
    path.push(relative_path);

    Ok(path)
}

pub fn download_icons(client: &Client, name: &str, game_url: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let icon_urls = icon_urls(client, game_url)?;
    let icons: Vec<DynamicImage> = icon_urls.iter()
        .filter_map(|url| download_and_decode(client, url).ok())
        .filter(is_square)
        .collect();

    fs::create_dir_all("icons")?;
    let add_index = icons.len() > 1;
    icons.iter().enumerate().map(|(i, icon)| {
        let name = if add_index {
            format!("{} {}", name, i)
        } else {
            name.to_string()
        };
        save_as_ico_and_return_path(&name, &icon)
    }).collect()
}
