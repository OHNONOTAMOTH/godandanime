use reqwest;
use scraper::{Html, Selector};

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

pub fn parseverse() -> String {
    let mut initials: String = "".to_string();
    let mut index = 0;
    let hh = &get()
                .chars()
                .nth(0)
                .unwrap().
                to_string()[..];
        initials.push(hh.to_owned().chars().nth(0).unwrap());
    for i in get().chars() {
        index += 1;
        if i == ' ' {
            let h = &get()
                .chars()
                .nth(index)
                .unwrap().
                to_string()[..];
            initials.push(h.to_owned().chars().nth(0).unwrap());
            }
    }
    return initials;
}
