use reqwest;
use scraper::{Html, Selector};
use crate::stringinitials;

pub fn getvinitials() -> String{
    return stringinitials::parse(get());
}

pub fn get() -> String {
    let resp = reqwest::blocking::get("https://www.verseoftheday.com/").unwrap().text().unwrap();
    let selector = Selector::parse(".bilingual-left");
    let parsed = Html::parse_document(&resp);
    let mut elements:Vec<&str> = Vec::new();
    for element in parsed.select(&selector.unwrap()) {
        let h = &element.text().collect::<Vec<_>>();
        elements.push(&h[0]);
    }
    let h = elements[0].to_owned();
    return h;
}

